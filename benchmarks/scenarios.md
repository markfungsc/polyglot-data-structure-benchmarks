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
