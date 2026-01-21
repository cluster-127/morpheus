//! Actor Model Baseline Benchmark
//!
//! Simulates Actor-style message handling for comparison with TES.
//! Key difference: O(n) memory per actor vs TES's O(grid) constant.
//!
//! Run with: cargo bench --bench actor_baseline

use std::collections::VecDeque;
use std::time::{Duration, Instant};

// Benchmark configuration - same as TES for fair comparison
const MAILBOX_CAPACITY: usize = 100;

/// Simulated Actor with mailbox
struct Actor {
    id: usize,
    mailbox: VecDeque<Message>,
    processed: usize,
    rejected: usize,
}

#[derive(Clone)]
struct Message {
    _payload: u32,
}

impl Actor {
    fn new(id: usize) -> Self {
        Self {
            id,
            mailbox: VecDeque::with_capacity(MAILBOX_CAPACITY),
            processed: 0,
            rejected: 0,
        }
    }

    /// Try to send message to this actor
    /// Returns true if accepted, false if mailbox full (backpressure)
    fn send(&mut self, msg: Message) -> bool {
        if self.mailbox.len() < MAILBOX_CAPACITY {
            self.mailbox.push_back(msg);
            true
        } else {
            self.rejected += 1;
            false
        }
    }

    /// Process one message from mailbox
    fn process_one(&mut self) -> bool {
        if let Some(_msg) = self.mailbox.pop_front() {
            self.processed += 1;
            true
        } else {
            false
        }
    }

    /// Memory size estimate (bytes)
    fn memory_size(&self) -> usize {
        std::mem::size_of::<Self>() + self.mailbox.capacity() * std::mem::size_of::<Message>()
    }
}

/// Actor System - manages multiple actors
struct ActorSystem {
    actors: Vec<Actor>,
    total_messages_sent: usize,
    total_rejected: usize,
}

impl ActorSystem {
    fn new(actor_count: usize) -> Self {
        let actors = (0..actor_count).map(Actor::new).collect();
        Self {
            actors,
            total_messages_sent: 0,
            total_rejected: 0,
        }
    }

    /// Send message to specific actor
    fn send_to(&mut self, actor_id: usize, msg: Message) -> bool {
        self.total_messages_sent += 1;
        if let Some(actor) = self.actors.get_mut(actor_id) {
            if actor.send(msg) {
                true
            } else {
                self.total_rejected += 1;
                false
            }
        } else {
            self.total_rejected += 1;
            false
        }
    }

    /// Process one tick (each actor processes one message)
    fn tick(&mut self) {
        for actor in &mut self.actors {
            actor.process_one();
        }
    }

    /// Total memory usage
    fn memory_usage(&self) -> usize {
        self.actors.iter().map(|a| a.memory_size()).sum()
    }
}

/// Scenario 1: Load Spike
fn benchmark_actor_load_spike(requests: usize, actors: usize) -> ActorLoadSpikeResult {
    let mut system = ActorSystem::new(actors);
    let msg = Message { _payload: 42 };

    let start = Instant::now();

    // All requests distributed across actors
    for i in 0..requests {
        let target = i % actors;
        system.send_to(target, msg.clone());
    }

    let duration = start.elapsed();
    let accepted = requests - system.total_rejected;

    ActorLoadSpikeResult {
        requests,
        actors,
        accepted,
        rejected: system.total_rejected,
        duration,
        rejection_rate: system.total_rejected as f64 / requests as f64 * 100.0,
        memory_bytes: system.memory_usage(),
    }
}

/// Scenario 2: Hotspot Saturation (single actor receives all traffic)
fn benchmark_actor_hotspot(requests_per_tick: usize, ticks: usize) -> ActorHotspotResult {
    let mut system = ActorSystem::new(1); // Single actor hotspot
    let msg = Message { _payload: 42 };

    let mut first_rejection_tick = None;

    let start = Instant::now();

    for tick in 0..ticks {
        // Send requests to the single actor
        for _ in 0..requests_per_tick {
            if !system.send_to(0, msg.clone()) {
                if first_rejection_tick.is_none() {
                    first_rejection_tick = Some(tick);
                }
            }
        }
        // Actor processes messages
        system.tick();
    }

    let duration = start.elapsed();

    ActorHotspotResult {
        ticks_to_first_rejection: first_rejection_tick.unwrap_or(ticks),
        total_rejected: system.total_rejected,
        duration,
        final_mailbox_size: system.actors[0].mailbox.len(),
    }
}

/// Scenario 3: Recovery (drain mailbox after load)
fn benchmark_actor_recovery() -> ActorRecoveryResult {
    let mut system = ActorSystem::new(1);
    let msg = Message { _payload: 42 };

    // Fill the mailbox
    for _ in 0..MAILBOX_CAPACITY {
        system.send_to(0, msg.clone());
    }

    let initial_size = system.actors[0].mailbox.len();
    let start = Instant::now();

    // Count ticks to drain
    let mut ticks_to_recover = 0usize;
    while !system.actors[0].mailbox.is_empty() {
        system.tick();
        ticks_to_recover += 1;
    }

    let duration = start.elapsed();

    ActorRecoveryResult {
        initial_mailbox_size: initial_size,
        ticks_to_recover,
        duration,
    }
}

#[derive(Debug)]
struct ActorLoadSpikeResult {
    requests: usize,
    actors: usize,
    accepted: usize,
    rejected: usize,
    duration: Duration,
    rejection_rate: f64,
    memory_bytes: usize,
}

#[derive(Debug)]
struct ActorHotspotResult {
    ticks_to_first_rejection: usize,
    total_rejected: usize,
    duration: Duration,
    final_mailbox_size: usize,
}

#[derive(Debug)]
struct ActorRecoveryResult {
    initial_mailbox_size: usize,
    ticks_to_recover: usize,
    duration: Duration,
}

fn main() {
    println!("=== Actor Model Baseline Benchmark ===\n");

    // Scenario 1: Load Spike
    println!("--- Scenario 1: Load Spike ---");
    for requests in [1_000, 10_000, 100_000, 1_000_000] {
        let actors = 100; // 100 actors to receive messages
        let result = benchmark_actor_load_spike(requests, actors);
        println!(
            "  {:>8} requests → {} actors: {:>6} accepted, {:>6} rejected ({:>5.1}%), memory: {} KB",
            requests, actors, result.accepted, result.rejected, result.rejection_rate,
            result.memory_bytes / 1024
        );
    }

    println!("\n--- Scenario 2: Hotspot Saturation ---");
    let sat_result = benchmark_actor_hotspot(10, 100);
    println!(
        "  First rejection at tick: {}\n  Total rejected: {}\n  Final mailbox: {}\n  Duration: {:?}",
        sat_result.ticks_to_first_rejection,
        sat_result.total_rejected,
        sat_result.final_mailbox_size,
        sat_result.duration
    );

    println!("\n--- Scenario 3: Recovery ---");
    let rec_result = benchmark_actor_recovery();
    println!(
        "  Initial mailbox: {}\n  Ticks to drain: {}\n  Duration: {:?}",
        rec_result.initial_mailbox_size, rec_result.ticks_to_recover, rec_result.duration
    );

    println!("\n=== Actor Key Metrics ===");
    println!("  Memory: O(n) = grows with actor count and mailbox depth");
    println!("  Rejection: Mailbox overflow (queue-based)");
    println!("  Recovery: Explicit processing required");
}
