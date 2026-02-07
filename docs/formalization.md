# Morpheus: Matematiksel Formalizasyon

Bu doküman, Topographic Execution Substrate (Morpheus) modelinin formel matematiksel tanımını içerir.

---

## 1. Teori Kullanımı

| Teori | Kullanım Amacı |
|-------|----------------|
| **Set theory** | Ontolojik *varlık* tanımı |
| **Type-theoretic kısıtlar** | *Yasaklı durumların* negatif sınır tanımı |

> Type kısıtları **enforcement iddiası taşımaz**. Static guarantee beklentisi yoktur.

---

## 2. Temel Aksiyom

```
act → space
```

`act` yalnızca bu ontolojik seviyede kullanılır ve **kapsam dışı başlangıç koşulu**nu ifade eder.

---

## 3. Primitifler

### 3.1 Space (Uzay)

```
Space := (V, ω, δ, τ)
```

| Bileşen | Tip | Tanım |
|---------|-----|-------|
| `V` | `⊆ ℝⁿ` | Vektörel durum uzayı |
| `ω` | `V → ℝ⁺` | Weight — yerel yoğunluk (immutable) |
| `δ` | `(ω, t) → ω'` | Gözlemsel projeksiyon fonksiyonu |
| `τ` | `V → T` | Yerel yaşam süresi fonksiyonu |

> **Kritik:** Uzay **atemporal ve edilgen**dir. `ω` değişmez; `δ` uzayı mutate etmez, **gözlemi projekte eder**.

### 3.2 Shape (Taşıyıcı)

```
Shape := (id, pos, β, λ, σ)
```

| Bileşen | Tip | Tanım |
|---------|-----|-------|
| `id` | `ℕ` | Benzersiz tanımlayıcı |
| `pos` | `V` | Uzaydaki konum |
| `β` | `ℕ` | Hafıza bütçesi |
| `λ` | `T` | Yaşam süresi |
| `σ` | `ℝ` | Duyarlılık — uzayın shape'e etkisi |

> `σ` (sensitivity) **tek yönlüdür**: uzay → shape. Shape uzayı etkilemez.

### 3.3 Trace (Skaler Alan)

```
Trace := ρ : V → ℝ⁺
```

> **Trace bir data structure değil, skaler alandır.** Origin (kaynak) bilgisi tutulmaz (Source Amnesia). Trace, uzayı büken topografik deformasyondur.

| Bileşen | Tip | Tanım |
|---------|-----|-------|
| `ρ` | `V → ℝ⁺` | Yoğunluk alanı |
| `μ` | `V → ℝ⁺` | Sürtünme alanı |

---

## 4. Boyut Ayrımı

| Boyut | Özellik |
|-------|---------|
| **Uzaysal** | Shape yaşanabilirlik koşullarına **maruz kalır** — hareket/etkileşim yok |
| **Temporal** | Shape tükenebilir kaynak boyunca var olur — trace desenleri burada oluşur |

> Tüm gözlemlenen "etki" yalnızca **zamansal izdüşüm** üzerinde ortaya çıkar.

---

## 5. Aksiyomlar

### A0: Temporal Başlangıç Koşulu

```
temporal_observation(s) ⟺ payload(s) ≠ ∅ ∧ ∃s': related(s, s')
```

> Initialize anında temporal gözlem yoktur. Gözlem = payload × ilişkisellik.

### A1: Yaşanabilirlik (Lokal)

```
inhabits(s, space) ⟺ β(s) > 0 ∧ λ(s) > 0 ∧ ω(pos(s)) < threshold(space)
```

### A2: Temporal Tükenme

```
∀t₁ < t₂: λ(s, t₂) ≤ λ(s, t₁)
```

### A3: Trace Birikimi

```
presence(s, v) ⟹ ρ(v) := ρ(v) + contribution(s)
```

> Side-effect semantiği. `ρ` skaler alan olarak güncellenir.

### A4: Decay (Projeksiyon)

```
observe(v, t) = δ(ω(v), t)
```

> `δ` mutasyon değil, **gözlemsel projeksiyon**.

### A5: Ölüm Kaçınılmazlığı

```
∀s: ∃t* ∈ T: λ(s, t*) = 0 ⟹ ¬inhabits(s, space)
```

---

## 6. Temporal Model

```
Time := PartialResource ℕ

spend(t, Δ) =
  | t ≥ Δ   → t - Δ
  | t < Δ   → ⊥
```

---

## 7. Faz Rejimleri

```
regime : V → {Solid, Liquid, Gas}
```

| Rejim | Trace Davranışı |
|-------|-----------------|
| **Solid** | Düşük geçirgenlik, hızlı saturasyon |
| **Liquid** | Orta geçirgenlik, dağılım |
| **Gas** | Yüksek geçirgenlik, desen oluşmaz |

> Faz = gradient bilgisi. Düz zemin = Gas, eğim = Liquid, zirve = Solid.

---

## 8. Stigmergy Prensibi

> Bellek shape'te değil, **ortamda** saklanır.

```
intelligence ∝ 1/shape_complexity × space_sensitivity
```

Shape aptal, uzay duyarlı olduğunda → akıl uzayın topografisinde ortaya çıkar.

---

## 9. Tutarlılık Koşulları

| İlişki | Sonuç |
|--------|-------|
| A0 ∧ A1 | Initialize anında presence yok, trace yok |
| A1 ∧ A5 | Yaşanabilirlik geçicidir |
| A3 ∧ A4 | Trace birikimi projeksiyon ile dengelenir |
| **A1 ∧ A3** | `¬inhabits(s) ⟹ presence(s, v)` tanımsızdır — ghost trace engeli |

---

## 10. Kapsam Dışı

- Origin tracking (Source Amnesia by design)
- Rollback / Replay
- Global yaşanabilirlik metrikleri
- Nedensel açıklama (causal explanation)
- Determinizm garantisi
- Shape → Space etkisi

---

## 11. Implementation Reference

Gerçek implementasyondaki karşılıklar:

| Formal | Implementasyon | Dosya |
|--------|----------------|-------|
| `Space := (V, ω, δ, τ)` | `Space::new(width, height, decay_rate, threshold)` | `src/space.rs:35` |
| `Shape := (id, pos, β, λ, σ)` | `Shape::new(id, x, y, budget, lifetime, sensitivity, contribution)` | `src/shape.rs:39` |
| `Trace := ρ : V → ℝ⁺` | `DensityGrid.cells: Vec<AtomicU32>` | `src/grid.rs:23` |
| `presence(s, v) ⟹ ρ(v) += contribution(s)` | `grid.contribute(x, y, amount)` | `src/grid.rs:77` |
| `observe(v, t) = δ(ω(v), t)` | `grid.apply_decay()` | `src/grid.rs:116` |
| `regime : V → {Solid, Liquid, Gas}` | `Regime` enum | `src/grid.rs:34` |

### Kod Örneği

```rust
use morpheus::{Substrate, IsotopeGrid, ServiceColor};

// Temel simülasyon
let mut sub = Substrate::new(100, 100, 5, 1000);
sub.spawn(50, 50, 100, 10);
sub.run(100);

// RGB Isotope ile spektroskopik analiz
let grid = IsotopeGrid::new(256, 256, 5, 2500, 1250);
let auth = ServiceColor::from_name("AuthService");
grid.contribute(100, 100, 200, auth);

// Difüzyon (enerji korunumlu)
grid.diffuse();
grid.apply_decay();
```
