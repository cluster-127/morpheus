# Morpheus: Spektroskopik Arayüz Spesifikasyonu

> Terminal Brutalism × Scientific Visualization

---

## 1. Tasarım Felsefesi

### 1.1 Müdahale Değil, Müşahede

Morpheus bir **Control Plane** değildir. Bir **Observation Plane**'dir.

| Yok | Var |
|-----|-----|
| Stop/Restart/Delete butonları | Sadece izleme |
| Active Shapes listesi | Yoğunluk haritası |
| Rule Builder / Drag-drop | Code-first config |

> Kullanıcı sistemi "yönetmez". Kurduğu kuralların yarattığı kaosu veya düzeni **izler**.

### 1.2 Görsel Dil

| Öğe | Tanım |
|-----|-------|
| **Palette** | `#000000` (Background), `#FFFFFF` (Peak), Grayscale heatmap |
| **Font** | `Geist Mono` veya `JetBrains Mono` |
| **Lines** | 1px hairline, grid çizgisi yok |
| **Metafor** | Weather Radar / Sonar / Electron Microscope |

---

## 2. Trace Isotopes (Spektroskopik Model)

### 2.1 Problem

Monokromatik trace, **Source Amnesia** gereği "kimin yaptığını" bilmez.
Bu, "burası kızardı" der ama "kim boğdu?" sorusunu cevaplayamaz.

### 2.2 Çözüm: RGB Vektör Modeli

Trace artık tek bir yoğunluk değeri değil, **polikromatik (RGB)** bir spektrumdur.

```rust
struct TracePixel {
    r: AtomicF32,  // Red channel
    g: AtomicF32,  // Green channel
    b: AtomicF32,  // Blue channel
}
```

| Servis | Renk |
|--------|------|
| Auth | Red |
| Payment | Green |
| Workers | Blue |
| Others | Hash → RGB |

### 2.3 Avantajlar

| Özellik | Bitmask | RGB Vector |
|---------|---------|------------|
| Oran bilgisi | ❌ Kayıp | ✅ Korunur |
| Bellek | 12 byte | 12 byte |
| Kapasite | 64 tür | Sınırsız |
| UX | "A ve B var" | "90% A, 10% B var" |

### 2.4 Ontolojik Uyum

| Aksiyom | Durum |
|---------|-------|
| Source Amnesia | ✅ "Hangi Shape?" bilinmiyor, "Hangi Tür?" biliniyor |
| O(1) Performance | ✅ Hala constant access |
| No Rollback | ✅ Replay ≠ Rollback |

---

## 3. Layout Mimarisi

```
┌─────────────────────────────────────────────┬──────┐
│                                             │      │
│           MAIN VIEWPORT                     │  S   │
│         (Spatial Now)                       │  E   │
│                                             │  T   │
│    • Canlı topografya haritası              │  T   │
│    • RGB Isotope rendering                  │  I   │
│    • Additive blending                      │  N   │
│                                             │  G   │
│    [Top-down heatmap / İzometrik 3D]        │  S   │
│                                             │      │
├─────────────────────────────────────────────┤      │
│                                             │      │
│           TEMPORAL FLUX                     │      │
│        (Slit-Scan Timeline)                 │      │
│                                             │      │
│  ◀━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▶   │      │
│  Geçmiş ←                          → Şimdi  │      │
│                                             │      │
└─────────────────────────────────────────────┴──────┘
```

### 3.1 Main Viewport (Spatial Now)

Canlı topografya haritası.

| Özellik | Açıklama |
|---------|----------|
| **Rendering** | WebGPU / wgpu |
| **Veri** | `Grid<TracePixel>` → texture |
| **Görsel** | Karanlık zemin, neon veri akışı |
| **Isotope** | RGB additive blending |
| **Efekt** | Grainy shader (sonar hissi) |
| **Filtreleme** | Tek kanal izole edilebilir |

### 3.2 Temporal Flux (Timeline)

Slit-Scan tekniği ile uzayın spektral kesiti.

| Özellik | Açıklama |
|---------|----------|
| **Teknik** | Uzay merkezinden 1px kesit, zaman içinde birikim |
| **Playhead** | Sağda sabit (şimdi), veri sola akar |
| **Scrubbing** | Geriye çekilince Main Viewport "ghost" render eder |
| **Uyarı** | Bu **Replay**, Rollback değil |

### 3.3 Settings Panel (Sağ Kenar)

| Ayar | Tip |
|------|-----|
| Decay rate | Slider |
| Threshold | Slider |
| Channel filter | Checkbox (R/G/B) |
| Zoom | Slider |

---

## 4. Telemetri Bantı

Alt bantta akan metrikler:

```
ENTROPY: 42.09 | PHASE: LIQUID | SOLID%: 12.3 | DECAY: 0.05
```

| Metrik | Açıklama |
|--------|----------|
| **ENTROPY** | Toplam trace yoğunluğu |
| **PHASE** | Baskın rejim (Solid/Liquid/Gas) |
| **SOLID%** | Doygun alan yüzdesi |
| **DECAY** | Sönümlenme hızı |

---

## 5. Developer Experience (DX)

### 5.1 Configuration as Code

GUI'de config yok. Kod ile tanım:

```rust
use morpheus::{IsotopeGrid, ServiceColor};

// Grid oluştur
let grid = IsotopeGrid::new(
    1024, 1024,      // dimensions
    5,               // decay_rate (0.005 fixed-point)
    2500,            // solid_threshold
    1250,            // liquid_threshold
);

// Servis rengi tanımla
let auth = ServiceColor::from_name("AuthService");
let payment = ServiceColor::from_name("PaymentService");

// Kontribüsyon yap
grid.contribute(x, y, magnitude, auth);
```

### 5.2 SDK Entegrasyonu

```rust
// Servis tarafında
let color = ServiceColor::from_name("AuthService");
grid.contribute(x, y, magnitude, color);
```

Her `contribute` çağrısı, ekranda o noktada renk parlaması ve decay sönümlenmesi yaratır.

### 5.3 Mevcut Implementasyon

| Özellik | Durum | Dosya |
|---------|-------|-------|
| RGB Isotope Grid | ✅ Tamamlandı | `src/isotope.rs` |
| ServiceColor hashing | ✅ Tamamlandı | `src/isotope.rs:113` |
| Difüzyon (enerji korunumlu) | ✅ Tamamlandı | `src/isotope.rs:265` |
| wgpu görselleştirme | ✅ Tamamlandı | `src/bin/viz.rs` |
| Temporal Flux (Timeline) | ❌ Henüz yok | Konsept aşamasında |
| Settings Panel (UI) | ❌ Henüz yok | Konsept aşamasında |

> **Not:** GUI spesifikasyonu konsept aşamasındadır. `morpheus-viz` binary'si temel görselleştirmeyi sağlar ancak tam GUI henüz implemente edilmemiştir.

---

## 6. Teknoloji Stack

### 6.1 Mevcut Implementasyon

| Katman | Teknoloji | Durum |
|--------|-----------|-------|
| Backend | Rust | ✅ `morpheus` crate |
| Rendering | wgpu 0.19 | ✅ `src/bin/viz.rs` |
| Window | winit 0.29 | ✅ `src/bin/viz.rs` |
| Shader | WGSL | ✅ `src/bin/viz.rs:392` |

Çalıştırma: `cargo run --bin morpheus-viz --features viz`

### 6.2 Planlanan Stack (Konsept)

| Katman | Teknoloji |
|--------|-----------|
| Backend | Rust |
| Rendering | wgpu / WebGPU |
| Window | winit veya Browser Canvas |
| Frontend | Yok (native) veya minimal HTML |

---

## 7. Özet

| Soru | Cevap |
|------|-------|
| Dashboard mı? | Hayır, **Radyoloji Cihazı** |
| Control Plane mi? | Hayır, **Observation Plane** |
| Data mı gösteriyor? | Hayır, **Arazi** gösteriyor |
| ID tracking var mı? | Hayır, **Tür (Type)** tracking var |

> Sistem artık kör bir termometre değil, **renkli bir MR cihazıdır**.
