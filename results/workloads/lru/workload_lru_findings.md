# Workload: LRU — comparison of cache-like structures (findings)

This document summarizes the **workload_lru** benchmark: comparison of **HashMap** (no eviction), **NaiveLRU** (hand-rolled with Vec + index map), **LruCache** (the `lru` crate), and **LinkedHashMap** (hashlink) in Rust across four operations (put, get, mostly_get, balanced) and memory. The benchmark uses request sequences over a key space to compare how each structure handles build, lookup, get-heavy (90% get / 10% put), and mixed (50% get / 50% put) workloads under three key-space scenarios. Results come from `raw/rust_workload_lru.csv` in this folder; plots are in `plots/`. The test uses a single language (Rust) to isolate structure and access-pattern effects.

**Structures in brief:** **LruCache** is a dedicated LRU cache library (no Rc/RefCell in the hot path). **NaiveLRU** is a hand-rolled LRU using a **Vec** for recency order plus a **HashMap&lt;Key, usize&gt;** for key → index; recency update is O(1) (swap with last + two index updates). **LinkedHashMap** maintains insertion order and evicts via pop_front; hashlink’s `get` does not update order by default, so the benchmark calls **to_back(k)** on each read hit for a fair comparison. All three LRU-like structures update recency on get, mostly_get, and balanced.

---

## What this benchmark measures

- **Structures:** HashMap (no capacity limit), NaiveLRU (HashMap + Vec + index map), LruCache (crate), LinkedHashMap (hashlink).
- **Operations:** (1) **Put** — build from N requests. (2) **Get** — N lookups with recency update on hit. (3) **Mostly get** — 90% get / 10% put. (4) **Balanced** — 50% get / 50% put. (5) **Memory** — process RSS when holding the structure (MB).
- **Scales:** N = 1,000; 10,000; 100,000; 1,000,000. Warm-up, 5 runs, mean ± std.

---

## NaiveLRU approach (O(1) recency)

NaiveLRU uses three pieces: **HashMap&lt;K, V&gt;** for key→value, **Vec&lt;K&gt;** for recency order (index 0 = oldest, last = newest), and **HashMap&lt;K, usize&gt;** for key→index in the Vec.

- **Move to back (recency update):** Given key `k`, index `i = index[k]`, last position `j = order.len()-1`. If `i != j`, swap `order[i]` with `order[j]`, set `index[replaced_key] = i`, `index[k] = j`. O(1).
- **Eviction:** Oldest is `order[0]`. Swap `order[0]` with `order[last]`, pop, remove evicted key from map and index, set `index[swapped_key] = 0`. O(1).
- **Put (new key):** Append to `order`, insert into map and index. O(1) amortized.

This gives O(1) get (with recency) and O(1) put (with eviction), but each get does one main map lookup plus a Vec swap and two index-map operations, so constant factors are higher than a single-structure cache. LruCache and LinkedHashMap update recency in one structure (internal list or `to_back`), which can be cheaper in practice.

---

## Key-space scenarios

| Scenario        | Key space | Capacity | Meaning |
|-----------------|-----------|----------|---------|
| **Full**        | N         | N        | No eviction; all keys fit. Baseline "plain map" behavior. |
| **High locality** | N/10    | N/10     | Few distinct keys, repeated access; no eviction but heavy reuse. |
| **Eviction**    | N         | N/10     | Working set larger than cache; constant eviction and recency updates. |

---

## Numbers at a glance (N = 1,000,000)

### Full (key_space = N, capacity = N)

| Structure     | Put (ms) | Get (ms) | Mostly get (ms) | Balanced (ms) | Memory (MB) |
|---------------|----------|----------|-----------------|---------------|-------------|
| HashMap       | 13.5     | 18.0     | 7.5             | 14.1          | 225.9       |
| NaiveLRU      | 37.5     | 124.6    | 14.2            | 24.9          | 226.1       |
| LruCache      | 65.6     | 17.5     | **50.4**        | 28.5          | 226.1       |
| LinkedHashMap | 56.2     | 12.7     | 5.7             | 23.6          | 247.7       |

**Note:** LruCache **mostly_get** at 1M full is ~9× slower than LinkedHashMap (50.4 vs 5.7 ms); the crate’s internal list is likely cache-unfriendly at this scale.

### High locality (key_space = N/10, capacity = N/10)

| Structure     | Put (ms) | Get (ms) | Mostly get (ms) | Balanced (ms) | Memory (MB) |
|---------------|----------|----------|-----------------|---------------|-------------|
| HashMap       | 15.0     | 14.3     | 5.7             | 7.4           | 118.1       |
| NaiveLRU      | 7.3      | 32.7     | 5.4             | 20.1          | 118.1       |
| LruCache      | 10.2     | 5.0      | 2.1             | 5.4           | 118.1       |
| LinkedHashMap | 7.8      | 4.6      | 2.6             | 4.9           | 118.1       |

### Eviction (key_space = N, capacity = N/10)

| Structure     | Put (ms) | Get (ms) | Mostly get (ms) | Balanced (ms) | Memory (MB) |
|---------------|----------|----------|-----------------|---------------|-------------|
| HashMap       | 13.1     | 18.9     | 7.9             | 14.7          | 82.3        |
| NaiveLRU      | 50.1     | 13.0     | 7.3             | 35.6          | 82.3        |
| LruCache      | 24.9     | 3.6      | 5.1             | 14.2          | 82.3        |
| LinkedHashMap | 21.6     | 3.7      | 5.5             | 12.5          | 82.3        |

(HashMap does not evict; included for reference.)

---

## How winners change with N and scenario

Performance rankings **depend on both N and scenario**: at small N (1k–10k) all four are often within a small factor; at large N (100k–1M) clear winners and crossovers show up in the plots. Below is a compact summary of **who is typically fastest at large N** for each (scenario, operation). Use the plots to see where curves cross as N grows.

| Scenario       | Put        | Get           | Mostly get    | Balanced      |
|----------------|------------|---------------|---------------|---------------|
| **Full**       | HashMap    | LinkedHashMap | LinkedHashMap | HashMap       |
| **High locality** | NaiveLRU / Linked | LinkedHashMap | LruCache / Linked | LinkedHashMap |
| **Eviction**   | LinkedHashMap | LinkedHashMap | LinkedHashMap | LinkedHashMap |

At small N, NaiveLRU can be competitive (e.g. put in high locality); at large N it is usually slowest on get and balanced. LruCache wins in several cells but loses on **mostly_get** in full at 1M (outlier). The plots in `plots/` show these crossovers by N.

---

## Comparing key-space scenarios

- **Full:** When everything fits, **HashMap** is fastest on put and balanced; **LinkedHashMap** is fastest on get (12.7 ms) and mostly_get (5.7 ms). **NaiveLRU** get (124.6 ms) is ~7× slower than LinkedHashMap because each get does a main map lookup plus Vec swap and two index-map updates. **LruCache** get (17.5 ms) is close to HashMap (18.0 ms); LruCache **mostly_get** (50.4 ms) is a clear outlier (~9× LinkedHashMap), likely due to internal list cost at 1M entries.

- **High locality:** Small working set, repeated keys. **LruCache** and **LinkedHashMap** lead on get and mostly_get (2–5 ms). **NaiveLRU** get (32.7 ms) and balanced (20.1 ms) are slower due to extra index map and Vec work. **LinkedHashMap** can be slightly faster than HashMap on mostly_get (e.g. 2.6 vs 5.7 ms) due to layout/access pattern.

- **Eviction:** **LinkedHashMap** and **LruCache** are fastest on get (3.6–3.7 ms) and put (21.6–24.9 ms). **NaiveLRU** put (50.1 ms) is ~2× slower (swap + two map updates per eviction); balanced (35.6 ms) is also costlier.

**Summary by scenario:** Full → HashMap or LinkedHashMap (LinkedHashMap best for get/mostly_get). High locality → LruCache or LinkedHashMap. Eviction → LinkedHashMap or LruCache; NaiveLRU is slowest on put and balanced.

---

## When is each structure fastest?

- **Put:** **HashMap** when no capacity limit. Under eviction, **LinkedHashMap** (21.6 ms at 1M) then LruCache (24.9 ms); NaiveLRU slowest (50.1 ms).
- **Get (get-only):** **LinkedHashMap** is fastest across scenarios (3.7–12.7 ms at 1M). LruCache and HashMap are close in full (17–18 ms). NaiveLRU is slowest (13–125 ms) due to two hashmaps + Vec swap per get.
- **Mostly get:** **LinkedHashMap** or **LruCache** (2–6 ms at 1M except LruCache full 50.4 ms). NaiveLRU is 5–14 ms. LruCache full is the only large outlier.
- **Balanced:** **HashMap** in full; **LinkedHashMap** or **LruCache** in high_locality and eviction. NaiveLRU is slower (20–36 ms at 1M).
- **Memory:** Similar at a given (N, scenario); **LinkedHashMap** is higher at full 1M (247.7 vs ~226 MB).

---

## Patterns and summary

1. **HashMap is best when there is no capacity limit.** No recency or list overhead.

2. **NaiveLRU (Vec + index) has O(1) recency but higher constant factors.** Each get does main map lookup + Vec swap + two index-map operations. Library caches (LruCache, LinkedHashMap) often do a single-structure update and can be faster in practice.

3. **LruCache mostly_get at 1M full is ~9× slower than LinkedHashMap** (50.4 vs 5.7 ms). The crate’s internal recency structure is likely cache-unfriendly at 1M entries; this is a real implementation cost, not measurement artifact.

4. **LinkedHashMap is a strong default for LRU-style workloads** in this benchmark: fastest or tied on get and mostly_get, competitive on put under eviction, with explicit **to_back** for recency on read.

---

## Takeaways

| Workload / goal                    | Best structure in this benchmark |
|------------------------------------|-----------------------------------|
| No eviction, full key set          | HashMap (put/balanced); LinkedHashMap (get/mostly_get) |
| LRU with eviction                  | LinkedHashMap or LruCache         |
| Get-heavy, any scenario            | LinkedHashMap (or LruCache except 1M full mostly_get) |
| Eviction stress, fast put/get      | LinkedHashMap                     |
| Minimal memory at same capacity    | HashMap or LruCache (LinkedHashMap higher at full) |
| Hand-rolled O(1) LRU               | NaiveLRU (Vec + index) is correct but slower than libraries |

- **HashMap** wins when capacity is not a constraint. **LinkedHashMap** leads on get and mostly_get in this benchmark; **LruCache** is competitive except on mostly_get at 1M full. **NaiveLRU** (Vec + index map) demonstrates O(1) recency and eviction but is slower than the libraries due to extra structure and operations per access.
