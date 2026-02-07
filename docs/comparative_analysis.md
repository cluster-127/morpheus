# Morpheus: Karşılaştırmalı Analiz

Bu doküman, Morpheus'ı (Topographic Execution Substrate) mevcut eşzamanlılık ve hesaplama modelleriyle karşılaştırır.

---

## 1. Model Kategorileri

Morpheus'ı doğru konumlandırmak için modelleri üç kategoriye ayırıyoruz:

| Kategori | Modeller | Odak |
|----------|----------|------|
| **Computational** | Actor Model, CSP | İş yapanlar — mesaj, davranış |
| **State** | Petri Nets, P-Systems | Durum tutanlar — token, marking |
| **Coordination** | Linda, Reaction-Diffusion | Ortam koordinasyonu — Morpheus'ın asıl alanı |

> ⚠️ Morpheus'ı Actor/CSP ile kıyaslamak **elma ile armut**tır. Morpheus bir "Coordination Medium".

---

## 2. Karşılaştırma Matrisi

| Özellik | Actor | CSP | Petri | P-Systems | Linda | R-D | **Morpheus** |
|---------|-------|-----|-------|-----------|-------|-----|---------|
| **Birim** | Actor | Process | Token | Object | Tuple | Morphogen | Shape |
| **Ortam** | Yok | Kanal | Place | Membran | Tuple Space | Field | **Space** |
| **İletişim** | Mesaj | Rendezvous | Fire | Kural | Match/Take | Diffusion | **Yok** |
| **Bellek** | Actor | - | Token | Multiset | Tuple | **Field** | **Field** |
| **Zaman** | Event | Trace | Discrete | Step | - | **Continuous** | **Tükenebilir** |

---

## BÖLÜM A: COMPUTATIONAL MODELS

### 3. Actor Model (Hewitt, 1973)

| Actor Model | Morpheus |
|-------------|-----|
| Mesaj gönderir/alır | **Mesaj yok** |
| Aktif davranış | Pasif yaşanabilirlik |
| Private state | Ortamda bellek (Stigmergy) |

> Zıt felsefe: Actor = aktif ajan, Morpheus = edilgen coğrafya.

### 4. CSP (Hoare, 1978)

| CSP | Morpheus |
|-----|-----|
| Senkron kanal (rendezvous) | **Kanal yok** |
| Trace = event sequence | Trace = skaler alan |
| Refinement semantik | Yaşanabilirlik aksiyomları |

> Ortogonal: CSP "ne olduğunu", Morpheus "nerede olabileceğini" modeller.

---

## BÖLÜM B: STATE MODELS

### 5. Petri Nets (Petri, 1962)

| Petri Nets | Morpheus |
|------------|-----|
| Token = discrete | Shape = bounded memory |
| Transition = explicit | **Transition yok** |
| Marking = countable | ρ = continuous density |
| Reachability mümkün | **Rollback imkansız** |

> Yapısal benzerlik, operasyonel fark. Morpheus'te transition yok, decay var.

### 6. P-Systems (Păun, 1998)

| P-Systems | Morpheus |
|-----------|-----|
| Membran = sınır | Space = topology |
| Kurallı evrim | **Kuralsız sönümlenme** |
| Hiyerarşik | Düz topoloji |
| Turing-complete | Turing iddiası yok |

> **En yakın akraba** (Stigmergy ortaklığı), ama Morpheus kuralsız.

---

## BÖLÜM C: COORDINATION MODELS 🎯

### 7. Linda & Tuple Spaces (Gelernter, 1985)

**Morpheus'in "Space" kavramının literatürdeki en net atası.**

| Linda | Morpheus |
|-------|-----|
| Tuple = discrete object | Trace = scalar field |
| Match/Take = retrieve | **Decay = fade** |
| Data bağımsız yaşar | Trace yoğunluk olarak birikir |
| Generative communication | **Stigmergy** |

#### Kritik Fark
- **Linda:** Veriyi "ortaya" bırakır → sonra "match" ile alınır
- **Morpheus:** Trace "ortaya" birikir → sonra "decay" ile sönümlenir

> Morpheus = **Linda'nın sürekli (continuous) ve sönümlenmeli (decaying) versiyonu**.

---

### 8. Reaction-Diffusion Systems (Turing, 1952)

**Morpheus'in matematiksel temelinin en güçlü paraleli.**

| Reaction-Diffusion | Morpheus |
|--------------------|-----|
| Morphogen concentration | Trace density (ρ) |
| Diffusion coefficient | Decay rate (δ) |
| Pattern formation | **Topografik deformasyon** |
| PDE-based (∂U/∂t = D∇²U + F) | **Discrete projection (δ)** |

#### Kritik Ortak Nokta
O 3D histogram görseli = Turing pattern formation'ın hesaplamasal gösterimi.

- **Petri:** "Deadlock" analizi yapar
- **Reaction-Diffusion:** "Stability" analizi yapar
- **Morpheus:** "Yaşanabilirlik" analizi yapar

> Morpheus, Turing'in morfogenez matematiğinin **bilgisayar bilimi yorumudur**.

---

### 9. Pi-Calculus (Milner, 1992)

**Dinamik topoloji için karşılaştırma.**

| Pi-Calculus | Morpheus |
|-------------|-----|
| Channel mobility (name-passing) | Position change (pos update) |
| Topology = **graph (links)** | Topology = **field (coordinates)** |
| Dynamic reconfiguration by link | Dynamic by **proximity** |

#### Kritik Fark
- **Pi-Calculus:** "Kimin kiminle konuştuğu" değişir (link-based)
- **Morpheus:** "Kim nerede" değişir (coordinate-based)

> Morpheus **graph-less**: bağlantı (edge) yok, yalnızca yakınlık (proximity) var.

---

## 10. Morpheus'ın Özgün Konumlandırması

### Hiçbir Modelde Olmayan Özellikler

| Özellik | Morpheus |
|---------|-----|
| **Atemporal uzay** | Uzay değişmez, gözlem değişir |
| **Trace = side-effect** | Skaler alan, object değil |
| **Source Amnesia** | Kimin iz bıraktığı bilinmez |
| **Rollback imkansız** | Matematiksel kısıt |
| **Davranışsız varlık** | Shape "yapmaz", sadece "var" |
| **Identity-Free Coordination** | Renk, tip, sahip yok — yalnızca yoğunluk (ρ) |

> Morpheus, literatürdeki **tek "Identity-Free Coordination"** modelidir.

### Literatürdeki Pozisyon

```
Morpheus = Linda ∩ Reaction-Diffusion − Rules
    = Generative Communication + Continuous Decay − Explicit Retrieval
```

Morpheus, Linda'nın "tuple'lar ortamda bağımsız yaşar" fikrini alır, Turing'in "morfogenez = aktivatör + inhibitör + difüzyon" matematiğini uygular, ama **explicit rule yoktur**.

---

## 11. Sonuç Tablosu

| Kategori | Model | Morpheus ile İlişki |
|----------|-------|----------------|
| Computational | Actor | Zıt (aktif vs pasif) |
| Computational | CSP | Ortogonal (event vs topology) |
| State | Petri | Yapısal benzerlik |
| State | P-Systems | En yakın durum modeli |
| **Coordination** | **Linda** | **Doğrudan ata** (discrete → continuous) |
| **Coordination** | **R-D** | **Matematiksel temel** |
| Coordination | Pi-Calculus | Graph vs Field farkı |

### Final Konumlandırma

> Morpheus = **Linda'nın sürekli, sönümlenmeli, kuralsız versiyonu**, Turing morfogenez matematiği üzerine inşa edilmiş bir **Coordination Medium**.
