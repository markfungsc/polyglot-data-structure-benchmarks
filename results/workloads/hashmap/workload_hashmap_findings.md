# Workload: Hashmap — comparison of key–aggregate structures (findings)

This document summarizes the **workload_hashmap** benchmark: comparison of **HashMap** (with FxHasher), **BTreeMap**, and **VecMap** (dense `Vec<Option<Agg>>` by symbol index) in Rust across four operations (update, lookup, scan, range) and memory. The benchmark uses **tick data** (symbol, price, volume) to mimic per-symbol aggregation — update aggregates by key, lookup by query list, full scan, and range query where order exists. Results come from `raw/rust_workload_hashmap.csv` in this folder; plots are in `plots/`. The test uses a single language (Rust) to isolate structure and access-pattern effects.

**Note on HashMap hashing:** The benchmark uses **FxHasher** (via `rustc_hash`) with `BuildHasherDefault` for the HashMap implementation. FxHasher provides **deterministic, fast hashing** (not cryptographically secure), which gives reproducible benchmarks and typically better throughput than the default SipHash for integer-like keys such as symbol IDs.

---

## What this benchmark measures

- **Structures:** HashMap (hash table with FxHasher), BTreeMap (ordered, log-time access), VecMap (dense array indexed by symbol id: `Vec<Option<Agg>>`).
- **Operations:** (1) **Update** — apply each tick to the aggregate for its symbol (entry API). (2) **Lookup** — query latest aggregate for each of N query keys. (3) **Scan** — sum `total_volume` over all entries (full iteration). (4) **Range** — sum volumes for keys in a fixed range (100..200); only BTreeMap and VecMap have a meaningful range; HashMap reports a placeholder. (5) **Memory** — process RSS when holding the structure (MB).
- **Scales:** N = 1,000; 10,000; 100,000; 1,000,000 ticks. Number of distinct symbols = N/10 (e.g. 100 at 1k, 100k at 1M). Warm-up, 5 runs, mean ± std.

---

## Numbers at a glance (N = 1,000,000)

| Structure  | Update (ms) | Lookup (ms) | Scan (ms) | Range (ms) | Memory (MB) |
|------------|-------------|-------------|-----------|------------|-------------|
| HashMap    | 5.21        | 3.92        | 0.085     | —          | 49.6        |
| BTreeMap   | 39.27       | 34.25       | 0.215     | 0.0009     | 47.1        |
| VecMap     | 1.83        | 0.78        | **0.046** | 0.0002     | 47.1        |

(Update/Lookup/Scan/Range are mean ms; memory is MB. HashMap has no range; value shown as —.)

---

## When is each structure fastest?

- **Update / Lookup:** **VecMap** is fastest when symbol ids are dense and small (direct index). **HashMap** (FxHasher) is next: O(1) expected, with lower constant factor than BTreeMap. **BTreeMap** is slowest (O(log n) per op and cache-unfriendly tree traversal).
- **Scan:** **VecMap** wins (single contiguous pass over `Vec<Option<Agg>>`). HashMap and BTreeMap iterate over entries with more indirection; BTreeMap scan is still fast but a bit slower than HashMap.
- **Range:** **BTreeMap** has a true ordered range (`range(100..200)`); **VecMap** simulates it with a filter on indices. Both are sub-ms; HashMap does not support range (reported as N/A).
- **Memory:** All three are in the same ballpark at 1M scale (~47–50 MB); VecMap and BTreeMap are slightly lower than HashMap in the reported run.

---

## Patterns and summary

1. **VecMap dominates when keys are dense indices.** If symbol ids (or key space) are 0..S and S is manageable, a `Vec<Option<Agg>>` gives the best update, lookup, and scan performance and simple range-by-index.

2. **HashMap (FxHasher) is a strong default for arbitrary integer-like keys.** Deterministic hashing makes benchmarks reproducible and FxHasher is fast; use it when key space is sparse or not a small contiguous range.

3. **BTreeMap is best when you need ordered iteration or range queries.** Update and lookup are slower than HashMap and much slower than VecMap; use when ordering or range operations are required.

4. **Range:** Only BTreeMap and VecMap are compared on range; both are very fast for a small fixed range.

5. **Memory:** Reported values are process RSS when holding the structure; all three are comparable for this workload at given N.

---

## Takeaways

| Workload / goal           | Best structure in this benchmark |
|---------------------------|-----------------------------------|
| Dense key index (0..S)    | VecMap (`Vec<Option<Agg>>`)       |
| Sparse keys, no order     | HashMap (e.g. with FxHasher)      |
| Range / ordered access    | BTreeMap (or VecMap if key = index) |
| Reproducible hash bench   | HashMap with FxHasher (deterministic) |

- **VecMap** wins on update, lookup, and scan when keys are dense symbol indices. **HashMap** with **FxHasher** is a good choice for generic key–aggregate workloads with deterministic, fast hashing. **BTreeMap** is the choice when order or range queries matter.
