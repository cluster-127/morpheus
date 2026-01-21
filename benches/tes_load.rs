//! TES Load Benchmark
//!
//! Measures TES performance under load:
//! - Memory: O(grid_size) constant regardless of request count
//! - Rejection latency: sub-microsecond
//! - Recovery: automatic via decay
//!
//! Run with: cargo bench --bench tes_load

use std::time::{Duration, Instant};
use tes::{IsotopeGrid, ServiceColor};

// Benchmark configuration - matches viz.rs physics
const GRID_WIDTH: usize = 256;
const GRID_HEIGHT: usize = 256;
const DECAY_RATE: u32 = 5; // Balanced for recovery
const HABITABILITY_THRESHOLD: u32 = 2500; // Earlier choking
const INTENSITY: u32 = 200;

/// Scenario 1: Load Spike
/// 10,000 requests hit the system simultaneously
fn benchmark_load_spike(requests: usize) -> LoadSpikeResult {
    let grid = IsotopeGrid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        DECAY_RATE,
        HABITABILITY_THRESHOLD,
        HABITABILITY_THRESHOLD / 2,
    );

    let color = ServiceColor::from_name("TestService");
    let mut accepted = 0usize;
    let mut rejected = 0usize;

    let start = Instant::now();

    // Simulate load spike - all requests target center hotspot
    for i in 0..requests {
        let x = (GRID_WIDTH / 2) + (i % 50);
        let y = (GRID_HEIGHT / 2) + (i / 50);

        if x < GRID_WIDTH && y < GRID_HEIGHT {
            if grid.is_habitable(x, y, HABITABILITY_THRESHOLD) {
                grid.contribute(x, y, INTENSITY, color);
                accepted += 1;
            } else {
                rejected += 1;
            }
        }
    }

    let duration = start.elapsed();

    LoadSpikeResult {
        requests,
        accepted,
        rejected,
        duration,
        rejection_rate: rejected as f64 / requests as f64 * 100.0,
        avg_latency_ns: duration.as_nanos() as f64 / requests as f64,
    }
}

/// Scenario 2: Hotspot Saturation
/// One service receives continuous traffic until saturation
fn benchmark_hotspot_saturation() -> SaturationResult {
    let grid = IsotopeGrid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        DECAY_RATE,
        HABITABILITY_THRESHOLD,
        HABITABILITY_THRESHOLD / 2,
    );

    let color = ServiceColor::from_name("HotspotService");
    let center_x = GRID_WIDTH / 2;
    let center_y = GRID_HEIGHT / 2;

    let mut ticks_to_first_rejection = 0usize;
    let mut found_first_rejection = false;
    let mut total_rejected = 0usize;

    let start = Instant::now();

    // Run until we see steady rejection rate
    for tick in 0..1000 {
        // 100 contributions per tick
        for _ in 0..100 {
            if grid.is_habitable(center_x, center_y, HABITABILITY_THRESHOLD) {
                grid.contribute(center_x, center_y, INTENSITY, color);
            } else {
                if !found_first_rejection {
                    ticks_to_first_rejection = tick;
                    found_first_rejection = true;
                }
                total_rejected += 1;
            }
        }
        grid.apply_decay();
    }

    let duration = start.elapsed();

    SaturationResult {
        ticks_to_first_rejection,
        total_rejected,
        duration,
        final_density: grid.density(center_x, center_y),
    }
}

/// Scenario 3: Cold Start Recovery
/// After saturation, measure time to clear via decay
fn benchmark_cold_start_recovery() -> RecoveryResult {
    let grid = IsotopeGrid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        DECAY_RATE,
        HABITABILITY_THRESHOLD,
        HABITABILITY_THRESHOLD / 2,
    );

    let color = ServiceColor::from_name("RecoveryService");
    let center_x = GRID_WIDTH / 2;
    let center_y = GRID_HEIGHT / 2;

    // Saturate the center
    for _ in 0..100 {
        grid.contribute(center_x, center_y, INTENSITY, color);
    }

    let initial_density = grid.density(center_x, center_y);
    let start = Instant::now();

    // Count ticks until habitable again
    let mut ticks_to_recover = 0usize;
    while !grid.is_habitable(center_x, center_y, HABITABILITY_THRESHOLD) {
        grid.apply_decay();
        ticks_to_recover += 1;
        if ticks_to_recover > 10000 {
            break; // Safety limit
        }
    }

    let duration = start.elapsed();

    RecoveryResult {
        initial_density,
        ticks_to_recover,
        duration,
        final_density: grid.density(center_x, center_y),
    }
}

#[derive(Debug)]
struct LoadSpikeResult {
    requests: usize,
    accepted: usize,
    rejected: usize,
    duration: Duration,
    rejection_rate: f64,
    avg_latency_ns: f64,
}

#[derive(Debug)]
struct SaturationResult {
    ticks_to_first_rejection: usize,
    total_rejected: usize,
    duration: Duration,
    final_density: u32,
}

#[derive(Debug)]
struct RecoveryResult {
    initial_density: u32,
    ticks_to_recover: usize,
    duration: Duration,
    final_density: u32,
}

fn main() {
    println!("=== TES Load Benchmark ===\n");

    // Scenario 1: Load Spike
    println!("--- Scenario 1: Load Spike ---");
    for requests in [1_000, 10_000, 100_000, 1_000_000] {
        let result = benchmark_load_spike(requests);
        println!(
            "  {:>8} requests: {:>6} accepted, {:>6} rejected ({:>5.1}%), avg latency: {:.0}ns",
            requests,
            result.accepted,
            result.rejected,
            result.rejection_rate,
            result.avg_latency_ns
        );
    }

    println!("\n--- Scenario 2: Hotspot Saturation ---");
    let sat_result = benchmark_hotspot_saturation();
    println!(
        "  First rejection at tick: {}\n  Total rejected: {}\n  Final density: {}\n  Duration: {:?}",
        sat_result.ticks_to_first_rejection,
        sat_result.total_rejected,
        sat_result.final_density,
        sat_result.duration
    );

    println!("\n--- Scenario 3: Cold Start Recovery ---");
    let rec_result = benchmark_cold_start_recovery();
    println!(
        "  Initial density: {}\n  Ticks to recover: {}\n  Final density: {}\n  Duration: {:?}",
        rec_result.initial_density,
        rec_result.ticks_to_recover,
        rec_result.final_density,
        rec_result.duration
    );

    println!("\n=== TES Key Metrics ===");
    println!(
        "  Memory: O(grid_size) = {} bytes constant",
        GRID_WIDTH * GRID_HEIGHT * 12
    );
    println!("  Rejection: Physical barrier (not rate limiting)");
    println!("  Recovery: Automatic via decay");
}
