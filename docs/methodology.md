# Methodology

Benchmarks are run in isolation on the same machine (or the same Docker image) so that results are comparable across languages.

## Common benchmark template

All data-structure benchmarks follow the same pattern so that plots and cross-language comparisons are meaningful.

- **SCALES (N)**: Fixed scenario sizes: 1,000; 10,000; 100,000; 1,000,000 elements.
- **NUM_RUNS**: Each scenario is timed 5 times; we report mean and standard deviation.
- **Warm-up**: One untimed run per scale before timed iterations to reduce cold-start effects.
- **Insert phase**: Time to build the structure (push / insert / put) for N elements (keys shuffled).
- **Get phase**: Time to access N elements (get by key/index, peek, etc.).
- **CSV schema**: Every structure writes a main CSV with header  
  `N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb`  
  and one row per scale. Output path: `RESULTS_DIR` (default `results/raw/`), filename `<lang>_<structure>.csv` (e.g. `cpp_heap.csv`).
- **Memory**: Best-effort per language (see below). Column `memory_mb`; may be 0 if unavailable.

### Operations per structure

| Structure       | Insert operation   | Get / access operation   | Notes |
|-----------------|--------------------|---------------------------|-------|
| Dynamic array   | push / append      | get(index)                | Index access O(1). |
| HashMap         | insert             | get(key)                  | Optional: low-entropy, load-factor CSVs. |
| Heap            | push               | peek (O(1)) or pop        | Get phase can measure peek; push and pop can be benchmarked separately. |
| Linked list     | push               | get(index)                | Get is O(n) per index. |
| LRU cache       | put(key, value)    | get(key)                  | Optional: eviction stress (capacity < N). |

Optional structure-specific CSVs (e.g. hashmap low-entropy, load-factor) use the same column conventions where applicable.

---

- **Warmup**: One untimed run per scale (or per scenario) before timed iterations.
- **Fixed N**: Scenario sizes (e.g. 1k, 10k, 100k, 1M) are fixed so that runs are comparable.
- **Iterations**: Each scenario runs 5 times; we report mean and standard deviation (insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms).
- **Memory**: Best-effort per language: Python uses `resource.getrusage` (Linux) peak RSS; Java uses `Runtime.totalMemory() - freeMemory()` (heap used); C++ uses `getrusage(RUSAGE_SELF).ru_maxrss` (Linux); Rust reads `/proc/self/status` VmRSS (Linux). Reported in MB. Not comparable across languages due to GC and allocator differences.
