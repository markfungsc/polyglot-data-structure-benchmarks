# Workload: Dynamic array — comparison of linear data structures (findings)

This document summarizes the **workload_dynamic_array** benchmark: comparison of **contiguous** (Vec, VecDeque, columnar) vs **non-contiguous** (LinkedList) storage in Rust across four operations (sum, VWAP, moving average, filter) and memory. The benchmark uses **tick data** (price and volume per tick) to mimic real-world trading data processing — sequential ingestion, windowed analytics (VWAP, moving average), and filtering by price. Results come from `raw/rust_workload_dynamic_array.csv` in this folder; plots are in `plots/`. The test uses a single language (Rust) to isolate layout and access-pattern effects.

---

## What this benchmark measures

- **Structures:** Vec (contiguous), VecDeque (contiguous ring buffer), LinkedList (non-contiguous, pointer chasing), Columnar (structure-of-arrays: separate `Vec<f64>` for price and volume).
- **Operations:** (1) **Sum** — sequential iteration, 10 passes over `price × volume` (cache-friendly scan). (2) **VWAP** — sliding-window volume-weighted average price. (3) **Moving average** — sliding-window average of price. (4) **Filter** — collect ticks with `price > threshold` into a new collection. (5) **Memory** — *delta* in process RSS when holding the structure (after − before build), in MB.
- **Scales:** N = 1,000; 10,000; 100,000; 1,000,000 ticks. Warm-up, 5 runs, mean ± std.

---

## Contiguous vs non-contiguous

**Contiguous (Vec, VecDeque, columnar):** Elements live in a single block (or a few blocks). Sequential access benefits from **cache locality**: consecutive elements are in the same cache line, so sum and windowed ops (VWAP, MA) stream through memory efficiently.

**Non-contiguous (LinkedList):** Each node is allocated separately; following pointers causes **cache misses** and no spatial locality. Sequential iteration (sum) and sliding windows (VWAP, MA) all pay the cost of pointer chasing.

The benchmark shows this clearly: **LinkedList is consistently slower** on every timed operation at every scale, and uses **more memory** (node + pointer overhead). It is included to quantify the cost of non-contiguous layout when the workload is sequential or windowed.

---

## Numbers at a glance (N = 1,000,000)

| Structure   | Sum (ms) | VWAP (ms) | MA (ms) | Filter (ms) | Memory (MB, delta) |
|-------------|----------|-----------|---------|-------------|---------------------|
| Vec         | 4.45     | 0.81      | 0.80    | 1.43        | 131.7               |
| VecDeque    | 4.50     | 1.31      | 0.85    | 1.12        | 131.7               |
| LinkedList  | 28.95    | 5.87      | 4.33    | 10.17       | 154.6               |
| Columnar    | 3.85     | 0.77      | 0.74    | **0.36**    | 154.6               |

(Sum/VWAP/MA/Filter are mean ms; memory is MB. Columnar memory here is for two `Vec<f64>`; reported RSS can match other structures depending on allocator and measurement.)

**Note on LinkedList VWAP/MA:** The benchmark reuses the Vec implementation for LinkedList by collecting the list into a `Vec` and then running the same sliding-window code. So VWAP and MA for LinkedList are *not* a fair measure of a native sliding window on a linked list — they are for reference only. A true sliding window on LinkedList would be naturally inefficient (no random access; each window would require walking pointers), so the reported times are optimistic for LinkedList in that sense.

---

## When is columnar fastest?

- **Filter:** Columnar is **fastest** at filter (0.36 ms at 1M). Filter only needs to scan the price column and sum `price × volume` for matching rows; no need to move full row structs. SoA avoids building filtered `Tick` structs and benefits from a single dense scan on `prices` with a zip over `volumes`.
- **Sum:** Vec, VecDeque, and columnar are all close (3.9–4.5 ms at 1M). Columnar does two array scans (prices, volumes) with zip; Vec does one struct scan. Similar cache behavior; columnar is competitive.
- **VWAP / MA:** Vec and columnar are **fastest** (VWAP ~0.77–0.81 ms, MA ~0.74–0.80 ms at 1M). Sliding windows over contiguous data are ideal. VecDeque pays for indexed access (ring-buffer offset) in the inner loop, so VWAP is a bit slower (1.31 ms); MA is 0.85 ms. LinkedList times are *not* native (see note above): the benchmark converts to Vec first, so they are for reference only; real sliding windows on LinkedList would be naturally inefficient.

**Summary:** Columnar wins on **filter** (column-wise condition, no row materialization). Vec and columnar are best for **sliding-window analytics** (VWAP, MA). For **pure sequential sum**, Vec, VecDeque, and columnar are comparable; LinkedList is slowest.

---

## Patterns and summary

1. **LinkedList is slowest on all ops and uses more memory.** Non-contiguous layout hurts sequential and windowed access. Use linked lists only when insertion/deletion in the middle without moving elements is the dominant need, not bulk scans or analytics.

2. **Vec and columnar are best for sliding-window ops (VWAP, MA).** Contiguous slices (or parallel column slices) give simple window bounds and good locality. VecDeque indexed access adds some cost in the inner loop. LinkedList VWAP/MA here are measured by converting to Vec first (for reference only); native sliding windows on LinkedList would be inefficient.

3. **Columnar is best for filter.** Predicate on one column, write indices or a second column; no row struct allocation. Good fit for analytics that select by condition and then operate on a subset of columns.

4. **Sum (sequential scan):** All contiguous layouts (Vec, VecDeque, columnar) are within ~10% at 1M. Choice can depend on how data is produced or consumed elsewhere; raw throughput is similar.

5. **Memory:** Vec and VecDeque report the same RSS (single buffer of ticks). LinkedList is higher (pointers and node headers). Columnar stores two vectors (prices, volumes); total size is similar to Vec<Tick> (two f64s per tick), but allocator and fragmentation can make RSS differ slightly.

---

## Takeaways

| Workload / goal              | Best structure in this benchmark |
|-----------------------------|-----------------------------------|
| Sliding-window (VWAP, MA)   | Vec or columnar (contiguous)       |
| Filter by column predicate  | Columnar (SoA)                    |
| Simple sequential sum       | Vec, VecDeque, or columnar (tie)   |
| Minimal memory, no pointers | Vec or columnar                   |
| Avoid for bulk scans        | LinkedList (non-contiguous)        |

- **Contiguous allocation (Vec, columnar buffers) wins** for scan and windowed analytics; **columnar wins** when the op is column-oriented (e.g. filter on one column). **LinkedList** is consistently the worst for these access patterns and should be avoided when the workload is sequential or windowed.
