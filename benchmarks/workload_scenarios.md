# Workload benchmark scenarios

Workload tests run **multi-operation sequences** or **key-space scenarios** in **Rust only**, to compare different data structures on the same workload (e.g. Vec vs LinkedList on sum/VWAP/filter, or HashMap vs LruCache vs LinkedHashMap on put/get under eviction). Scales are N = 1k, 10k, 100k, 1M unless noted. Each workload writes CSV to a raw dir and can be plotted with `benchmarks/plot_workload.py`. Summary: [results/WORKLOAD_SUMMARY.md](../results/WORKLOAD_SUMMARY.md).

---

## Workload: Dynamic array (linear structures)

- **Structures:** Vec, VecDeque, LinkedList, Columnar (structure-of-arrays).
- **Scenario:** Tick-style data (price, volume); sequential ingestion and windowed/analytic ops.
- **Operations:** Sum (10 passes over price×volume), VWAP (sliding window), moving average, filter (price > threshold), memory (RSS delta).
- **Output:** `rust_workload_dynamic_array.csv` — N, per-structure columns for each op (mean_ms, std_ms), memory_mb. Typically under `results/workloads/dynamic_array/raw/`.
- **Findings:** [results/workloads/dynamic_array/workload_dynamic_array_findings.md](../results/workloads/dynamic_array/workload_dynamic_array_findings.md).

---

## Workload: Hashmap (key–aggregate structures)

- **Structures:** HashMap (FxHasher), BTreeMap, VecMap (dense `Vec<Option<Agg>>` by symbol id).
- **Scenario:** Tick-style data (symbol, price, volume); per-symbol aggregation. Distinct symbols = N/10.
- **Operations:** Update (apply tick to aggregate), lookup (N query keys), scan (sum over all entries), range (sum in key range 100..200), memory.
- **Output:** `rust_workload_hashmap.csv` — N, per-structure columns for update/lookup/scan/range (mean_ms, std_ms), memory_mb. Typically under `results/workloads/hashmap/raw/`.
- **Findings:** [results/workloads/hashmap/workload_hashmap_findings.md](../results/workloads/hashmap/workload_hashmap_findings.md).

---

## Workload: Heap (priority-queue–style structures)

- **Structures:** BinaryHeap, BTreeSet, SortedVec (insert-in-order), Vec + Sort.
- **Scenario:** Integer values 0..N; build, drain by max, repeated peek at max, top-k over stream.
- **Operations:** Push (build from N), pop (drain all), peek (k = N/10 repeated peeks at max), topk (stream N, keep top k), memory.
- **Output:** `rust_workload_heap.csv` — N, per-structure columns for push/pop/peek/topk (mean_ms, std_ms), memory_mb. Typically under `results/workloads/heap/raw/`.
- **Findings:** [results/workloads/heap/workload_heap_findings.md](../results/workloads/heap/workload_heap_findings.md).

---

## Workload: LRU (cache-like structures)

- **Structures:** HashMap (no eviction), NaiveLRU (Vec + index map), LruCache (crate), LinkedHashMap (hashlink). All LRU-like update recency on get/mostly_get/balanced.
- **Scenarios (key-space × capacity):**
  - **Full:** key_space = N, capacity = N (no eviction).
  - **High locality:** key_space = N/10, capacity = N/10 (repeated keys, no eviction).
  - **Eviction:** key_space = N, capacity = N/10 (working set > cache).
- **Operations:** Put (build from N requests), get (N lookups, get-only timed), mostly_get (90% get / 10% put), balanced (50% get / 50% put), memory.
- **Output:** `rust_workload_lru.csv` — N, scenario, per-structure columns for put/get/mostly_get/balanced (mean_ms, std_ms), memory_mb. Typically under `results/workloads/lru/raw/` or `results/raw/`. Plots are per scenario (e.g. `workload_lru_full_put_log.png`).
- **Findings:** [results/workloads/lru/workload_lru_findings.md](../results/workloads/lru/workload_lru_findings.md).

---

## Running and plotting

- **Run (e.g. Rust):** `./benchmarks/run_all.sh rust workload_dynamic_array` (or `workload_hashmap`, `workload_heap`, `workload_lru`).
- **Plot:** `python3 benchmarks/plot_workload.py --raw-dir <path-to-raw> --plots-dir <path-to-plots>`. Default raw/plots dirs are under `results/workloads/`; for LRU you may pass `--raw-dir results/raw` if CSVs are there.
