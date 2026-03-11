# Benchmarked scenarios

Each scenario is implemented equivalently across Python, Java, C++, and Rust. What is measured (time, memory, or both) is noted per scenario.

**Implemented benchmarks** (dynamic array, linked list, heap, hashmap) use scales N = 1,000; 10,000; 100,000; 1,000,000 (1k–1M), with an **insert phase** (build structure for N elements) and a **get phase** (N indexed or key-based accesses; linked list: one full traverse; heap: N pops of the minimum). Each scale is timed 5 times (mean ± std) with one untimed warm-up run. See [methodology.md](../docs/methodology.md) for the common template and CSV schema.

---

## Main scenario (insert + get) — *implemented* (dynamic array, hashmap)

- **Scales:** N = 1k, 10k, 100k, 1M (one row per scale in the main CSV).
- **Insert phase:** Time to build the structure (push/append or put) for N elements (keys shuffled for hashmap).
- **Get phase:** Time for N indexed or key-based accesses (random access).
- **Output:** `<lang>_<structure>.csv` with columns N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, memory_mb.

---

## Linked list scenario (insert + traverse + delete) — *implemented* (linked list)

- **Scales:** N = 1k, 10k, 100k, 1M (one row per scale).
- **Insert phase:** Time to build the list (push_back) for N elements (keys shuffled).
- **Get phase:** Time for **one full sequential traversal** (e.g. `traverse(f)`), not a loop of `get(i)` (which would be O(n²)). Ensures comparable O(n) “read all” cost across languages.
- **Delete phase:** Time for one delete (e.g. delete last element). Delete at end is O(n) for a singly-linked list (must traverse to predecessor).
- **Output:** `<lang>_linked_list.csv` with columns N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, delete_mean_ms, delete_std_ms, memory_mb.
- **Findings:** [results/linked_list/linked_list_findings.md](../results/linked_list/linked_list_findings.md) (Big O, pitfalls like get(i) in a loop, Java non-linear scaling).

---

## Heap scenario (insert + get = pop min) — *implemented* (heap)

- **Scales:** N = 1k, 10k, 100k, 1M (one row per scale).
- **Insert phase:** Time to build the heap (insert/push) for N elements (keys shuffled).
- **Get phase:** Time for **N consecutive pops of the minimum** (drain the heap by repeated `pop()`). This is **not** random index access — heaps support O(1) peek at root and O(log n) pop min; the benchmark measures the cost of extracting the minimum element N times.
- **Output:** `<lang>_heap.csv` with columns N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, memory_mb.
- **Notes:** Python uses `heapq` (C implementation) for best possible heap performance. C++ uses move semantics in pop/sift for performance.
- **Findings:** [results/heap/heap_findings.md](../results/heap/heap_findings.md) (Big O, move vs swap, Python heapq).

---

## LRU Cache scenario (put/get, eviction) — *implemented* (LRU cache)

- **Scales:** N = 1k, 10k, 100k, 1M (one row per scale). Capacity is typically derived from N (e.g. N/10 or fixed ratio).
- **Phases:** put_miss (insert new keys), put_hit (update existing), get_hit, get_miss, eviction (trigger evictions by inserting until capacity exceeded and evictions occur). Memory after build.
- **Output:** `<lang>_lru_cache.csv` (e.g. `rust_lru_cache.csv`, `rust_native_lru_cache.csv` for the native `lru` crate). Columns include N, put_miss_mean_ms, put_hit_mean_ms, get_hit_mean_ms, get_miss_mean_ms, eviction_mean_ms, memory_mb.
- **Findings:** [results/lru_cache/lru_cache_findings.md](../results/lru_cache/lru_cache_findings.md). Rust has two variants: custom (Rc/RefCell) and native `lru` crate.

---

## HashMap: low-entropy / near-collision — *implemented* (hashmap)

For the hashmap benchmark only: use a fixed small capacity (e.g. 64 buckets) so that many keys share few buckets (low entropy per bucket, near-collision workload). Same N scales as the main scenario. Output: `*_hashmap_low_entropy.csv`. Measures how insert/get degrade when many keys map into few buckets.

## HashMap: load factor sensitivity — *implemented* (hashmap)

For the hashmap benchmark only: fix N (e.g. 100_000) and vary initial capacity so that effective load factor is 0.25, 0.5, 0.75, 1.0. Output: `*_hashmap_loadfactor.csv`. Measures how performance changes with load factor.

---

## Delete workload — *partially implemented*

- **Implemented:** Linked list has a **delete phase** (one delete of last element per run); see [Linked list scenario](#linked-list-scenario-insert--traverse--delete--implemented-linked-list) above.
- **Planned:** A full delete workload (remove a large fraction of elements, e.g. half, in a defined pattern: front, back, random) across multiple structures is not yet implemented. Would measure time and reallocation cost.

## Mixed read/write workload — *partially implemented*

- **Implemented:** **Workload LRU** (Rust) includes mixed read/write: **balanced** (50% get / 50% put) and **mostly_get** (90% get / 10% put). See [workload_scenarios.md](workload_scenarios.md) and [results/workloads/lru/workload_lru_findings.md](../results/workloads/lru/workload_lru_findings.md).
- **Planned:** A generic cross-language mixed read/write scenario (interleave inserts, lookups, deletes in a controlled ratio) for other structures (array, hashmap, heap) is not yet implemented.

## Concurrent producer-consumer — *planned*

Multiple producer threads and consumer threads sharing a queue or similar structure. Measures throughput and correctness under contention.

## High allocation churn test — *planned*

Create and discard many short-lived structures or elements to stress allocator and GC. Measures time and peak memory.
