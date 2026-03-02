# Benchmarked scenarios

Each scenario is implemented equivalently across Python, Java, C++, and Rust. What is measured (time, memory, or both) is noted per scenario.

**Implemented benchmarks** (dynamic array, hashmap) use scales N = 1,000; 10,000; 100,000; 1,000,000 (1k–1M), with an **insert phase** (build structure for N elements) and a **get phase** (N indexed or key-based accesses). Each scale is timed 5 times (mean ± std) with one untimed warm-up run. See [methodology.md](../docs/methodology.md) for the common template and CSV schema.

---

## Main scenario (insert + get) — *implemented* (dynamic array, hashmap)

- **Scales:** N = 1k, 10k, 100k, 1M (one row per scale in the main CSV).
- **Insert phase:** Time to build the structure (push/append or put) for N elements (keys shuffled for hashmap).
- **Get phase:** Time for N indexed or key-based accesses (random access).
- **Output:** `<lang>_<structure>.csv` with columns N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, memory_mb.

---

## Delete workload — *planned*

Remove a large fraction of elements (e.g. half) in a defined pattern (front, back, random). Measures time and any reallocation cost.

## Mixed read/write workload — *planned*

Interleave inserts, lookups, and deletes in a controlled ratio. Measures sustained throughput and fairness across operations.

## Concurrent producer-consumer — *planned*

Multiple producer threads and consumer threads sharing a queue or similar structure. Measures throughput and correctness under contention.

## High allocation churn test — *planned*

Create and discard many short-lived structures or elements to stress allocator and GC. Measures time and peak memory.

---

## HashMap: low-entropy / near-collision — *implemented* (hashmap)

For the hashmap benchmark only: use a fixed small capacity (e.g. 64 buckets) so that many keys share few buckets (low entropy per bucket, near-collision workload). Same N scales as the main scenario. Output: `*_hashmap_low_entropy.csv`. Measures how insert/get degrade when many keys map into few buckets.

## HashMap: load factor sensitivity — *implemented* (hashmap)

For the hashmap benchmark only: fix N (e.g. 100_000) and vary initial capacity so that effective load factor is 0.25, 0.5, 0.75, 1.0. Output: `*_hashmap_loadfactor.csv`. Measures how performance changes with load factor.
