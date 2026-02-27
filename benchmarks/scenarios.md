# Benchmarked scenarios

Each scenario is implemented equivalently across Python, Java, C++, and Rust. What is measured (time, memory, or both) is noted per scenario.

## Insert 1M elements

Append one million elements to the structure (dynamic array, or equivalent). Measures time and optionally peak memory.

## Random access

Perform many indexed or key-based reads (e.g. array by index, hash map by key). Measures throughput or latency.

## Delete workload

Remove a large fraction of elements (e.g. half) in a defined pattern (front, back, random). Measures time and any reallocation cost.

## Mixed read/write workload

Interleave inserts, lookups, and deletes in a controlled ratio. Measures sustained throughput and fairness across operations.

## Concurrent producer-consumer

Multiple producer threads and consumer threads sharing a queue or similar structure. Measures throughput and correctness under contention.

## High allocation churn test

Create and discard many short-lived structures or elements to stress allocator and GC. Measures time and peak memory.

## HashMap: low-entropy / near-collision

For the hashmap benchmark only: use a fixed small capacity (e.g. 64 buckets) so that many keys share few buckets (low entropy per bucket, near-collision workload). Same N scales as the main scenario. Output: `*_hashmap_low_entropy.csv`. Measures how insert/get degrade when many keys map into few buckets.

## HashMap: load factor sensitivity

For the hashmap benchmark only: fix N (e.g. 100_000) and vary initial capacity so that effective load factor is 0.25, 0.5, 0.75, 1.0. Output: `*_hashmap_loadfactor.csv`. Measures how performance changes with load factor.
