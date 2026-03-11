# Workload: Heap — comparison of priority-queue–style structures (findings)

This document summarizes the **workload_heap** benchmark: comparison of **BinaryHeap**, **BTreeSet**, **SortedVec** (insert-in-order with binary search), and **Vec + Sort** in Rust across four operations (push, pop, peek, topk) and memory. The benchmark uses integer values 0..N to compare how each structure handles build, drain-by-max, repeated peek at max, and top-k over a stream. Results come from `raw/rust_workload_heap.csv` in this folder; plots are in `plots/`.

---

## What this benchmark measures

- **Structures:** BinaryHeap (Rust max-heap), BTreeSet (ordered set, peek = last), SortedVec (Vec kept sorted by insert with binary_search + insert), Vec + Sort (copy then sort_unstable for each op).
- **Operations:** (1) **Push** — build the structure from N values. (2) **Pop** — drain the structure (pop max until empty; BTreeSet/SortedVec pop from end = max). (3) **Peek** — k repeated peeks at the current max (k = N/10). (4) **TopK** — stream N values, maintain the top k (largest), return their sum. (5) **Memory** — RSS delta from building the structure (MB).
- **Scales:** N = 1,000; 10,000; 100,000; 1,000,000. Warm-up, 5 runs, mean ± std.

---

## Big-O by operation and structure

| Operation | BinaryHeap | BTreeSet | SortedVec | Vec + Sort |
|-----------|------------|----------|-----------|------------|
| **Push** (build from N) | O(N log N) | O(N log N) | O(N²) | O(N log N) |
| **Pop** (drain all) | O(N log N) | O(N log N) | O(N) | O(N log N) |
| **Peek** (k peeks at max) | O(k) | O(k log N) | O(k) | O(N log N) + O(k) |
| **TopK** (N items, keep top k) | O(N log k) | O(N log k) | O(N·k) | O(N log N) |

- **Push:** BinaryHeap and BTreeSet do N inserts at O(log n) each. SortedVec does N × (binary_search + insert) → O(N²). Vec + Sort is copy O(N) plus sort O(N log N).
- **Pop:** SortedVec pops from end in O(1) each → O(N). Others do O(log n) per pop.
- **Peek:** Heap and SortedVec have O(1) access to max; BTreeSet::last() is O(log N) per call. Vec + Sort pays full sort cost then O(1) per peek.
- **TopK:** Heap and BTreeSet maintain a set of size ≤ k → O(N log k). SortedVec keeps a sorted vec of size k with insert + remove(0) per element → O(N·k). Vec + Sort does one full sort then slice → O(N log N).

---

## Vec + Sort outperforming despite same or worse Big-O

In the benchmark, **Vec + Sort often wins** (or is competitive) on wall-clock time even when its asymptotic complexity is the same or worse:

- **Push:** Same O(N log N) as BinaryHeap/BTreeSet, but Vec + Sort is much faster in practice (e.g. at N=1M: ~0.46 ms vs ~7.8 ms heap, ~48 ms BTreeSet). **Reason:** One contiguous allocation, one cache-friendly sort (e.g. quicksort/intro sort); no per-element heap/tree overhead, no pointer chasing or node allocations.
- **Pop:** Same O(N log N), but Vec + Sort is again much faster (one sort then N × O(1) pops from the end). **Reason:** Sort is done once; popping from the end of a Vec is O(1) and very cache-friendly; heap/btree pay O(log N) per pop with more indirection.
- **Peek:** Vec + Sort has *worse* Big-O (O(N log N) for the sort vs O(1) or O(k) for heap/sorted_vec), but if k is large or the constant factors dominate, the single sort + sequential access can still be competitive. **Reason:** Modern CPUs favor sequential access; one sort may be cheaper than many tree/heap traversals in practice.
- **TopK:** Vec + Sort is O(N log N) vs O(N log k) for heap/BTreeSet (worse when k ≪ N). Nevertheless it can be faster at moderate N. **Reason:** One pass of a highly optimized sort over a contiguous buffer beats many small heap/tree operations (allocation, pointer updates, cache misses).

**Summary:** Big-O describes growth with n; constant factors and memory layout matter a lot. Vec + Sort benefits from **contiguous memory**, **few allocations**, and **cache-friendly** access, so it often outperforms pointer-based structures (BinaryHeap, BTreeSet) and the quadratic SortedVec (insert-in-order) even when asymptotics are equal or worse.

---

## When is each structure fastest?

- **Push:** **Vec + Sort** is fastest (and BinaryHeap second) at scale. SortedVec is slow due to O(N²) insert-in-order.
- **Pop:** **Vec + Sort** and **SortedVec** (O(N) pops from end) are fastest; BinaryHeap and BTreeSet pay O(log N) per pop.
- **Peek:** **BinaryHeap**, **SortedVec**, and **Vec + Sort** can all be fast; BTreeSet pays O(log N) per last().
- **TopK:** **Vec + Sort** and **BinaryHeap** are fast at scale. **SortedVec** (O(N·k)) becomes very slow for large N and k (e.g. 1M and 100k). BTreeSet is O(N log k) but with higher constant factors than the heap.
- **Memory:** BTreeSet typically uses more space (node overhead); BinaryHeap and Vec/SortedVec are more compact. (Reported values are RSS deltas from building each structure.)

---

## Patterns and summary

1. **Vec + Sort is a strong choice when the workload is batch-oriented:** One build (copy + sort) and then many pops or peeks is very fast due to locality and low overhead, even with the same or worse Big-O than heap/btree.

2. **BinaryHeap is best when you need incremental updates and O(log n) push/pop:** e.g. streaming top-k with small k, or a true priority queue with interleaved pushes and pops.

3. **BTreeSet gives ordered iteration and O(log n) operations** but with higher constant factors and memory; use when you need the set abstraction or order, not when a heap or sorted vec suffices.

4. **SortedVec (insert-in-order) is O(N²) on push and O(N·k) on topk;** it only makes sense for very small N or when you need to maintain order under incremental inserts without a full re-sort.

---

## Takeaways

| Workload / goal | Best structure in this benchmark |
|-----------------|----------------------------------|
| Batch build then drain or peek | Vec + Sort |
| Streaming top-k, small k | BinaryHeap (or Vec + Sort at moderate N) |
| Need ordered iteration / set semantics | BTreeSet |
| Minimal memory, batch-only | Vec + Sort |

- **Vec + Sort** often outperforms the others on wall-clock time despite the same or worse Big-O, thanks to **cache-friendly contiguous access** and **low constant factors**. Prefer it when the workload is batch-oriented; prefer **BinaryHeap** when you need incremental priority-queue behavior.
