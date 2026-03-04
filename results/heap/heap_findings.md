# Heap benchmark findings (methodology & summary)

This document covers: **Binary min-heap** benchmark result comparison across Python, Java, C++, and Rust; **methodology** (warm-up, 5 runs, mean ± std, memory); **complexity** (Big O for insert, peek, pop); and **implementation notes** (move vs swap, Python heapq). See [methodology.md](../../docs/methodology.md) for definitions. Results come from `raw/*_heap.csv` in this folder; plots are in `plots/`.

---

## Complexity (Big O) and benchmark design

All implementations are **binary min-heaps** (array-backed). The benchmark measures insert and the cost of **removing the minimum** repeatedly (pop), not random index access.

| Operation | Big O | Notes |
|-----------|--------|--------|
| **Insert** | O(log n) | Push element, then sift up. |
| **Peek** | O(1) | Read root (index 0). |
| **Pop (get min)** | O(log n) per pop | Swap root with last, remove last, sift down the new root. The benchmark **“get”** phase times **N consecutive pops** (pop the minimum element N times until the heap is empty), **not** reading a random index. |

**What “get” measures here:** The CSV column `get_mean_ms` / `get_std_ms` is the time to **pop the first (minimum) element N times** — i.e. drain the heap by repeated `pop()`. So “get” is **pop-min**, not random access. This matches typical heap usage (priority queue: repeatedly extract min).

---

## Implementation notes: move vs swap, Python heapq

**Move semantics (C++, Rust) vs plain swap:** In C++, the heap uses **`std::move`** when moving elements during pop and sift-down (e.g. `T result = std::move(data_[0]);`, `data_[index] = std::move(data_[parent]);`). Rust also uses **move semantics**: in `sift_up` and `sift_down`, a value is read from the index, then elements are shifted by assignment (`data[index] = data[parent]` or `data[index] = data[smallest]`), then the saved value is written back — assignment in Rust moves (or copies for `Copy` types like `i32`). In `pop`, Rust swaps root and last with `swap(0, last_index)` then `pop()` to remove the last; the sift-down that follows uses the same move-style assignments. So both C++ and Rust avoid extra copies in the sift logic; **prefer move semantics where available** when implementing heap pop/sift to get the best performance.

**Python: `heapq` for best possible performance.** The Python implementation uses the standard **`heapq`** module (implemented in C in CPython). So the Python numbers represent the **best achievable heap performance in Python** for this workload (insert + pop min), not a custom Python-only heap. Even so, interpreter overhead and per-call cost keep Python slower than AOT-compiled languages.

---

# Findings: Heap benchmark comparison

Summary of results from `raw/*_heap.csv` (scaled N, warm-up, mean ± std over 5 runs). C++, Java, and Rust use **custom array-backed min-heaps**; Python uses **`heapq`** (C-backed). Data from current `raw/` CSVs; plots in `plots/`.

---

## Numbers at a glance (N = 1,000,000)

| Language | Insert (ms) | Get = pop min N times (ms) | Memory (MB) |
|----------|-------------|-----------------------------|-------------|
| C++      | 8.09 ± 0.35 | 69.46 ± 3.15               | 11.6        |
| Rust     | 9.16 ± 0.29 | 66.45 ± 0.81               | 2.19        |
| Java     | 22.50 ± 4.01| 80.26 ± 3.91                | 125.8       |
| Python   | 158.24 ± 20.05 | 457.44 ± 14.52          | 62.0        |

---

## Summary of test results

- **Insert:** C++ and Rust are fastest at 1M (~8–9 ms); Java ~22 ms; Python ~158 ms (using `heapq`). All scale with N and O(log n) per insert.
- **Get (pop min):** The “get” phase is **N pops** (extract minimum until empty). C++ and Rust are fastest (~66–69 ms at 1M); Java ~80 ms; Python ~457 ms. Pop is O(log n) per call, so total is O(n log n) for N pops.
- **Memory:** Rust reports lowest at 1M (2.19 MB); C++ 11.6 MB; Python 62 MB; Java highest (125.8 MB, heap usage). See `plots/`: heap_insert_log.png, heap_get_log.png, heap_memory_log.png.

---

## Expected vs unexpected behaviors

**Expected:**

1. **C++/Rust fastest:** AOT-compiled, array-backed heap with O(log n) insert and pop. Move semantics in C++ (and efficient layout in Rust) avoid unnecessary copies.
2. **Python slowest on time:** Even with `heapq` (C implementation), per-call Python overhead dominates at large N.
3. **Get (pop min) dominates at large N:** N pops cost O(n log n), so the “get” phase is heavier than the single insert phase (N × O(log n) insert).

**Notable:**

1. **Rust lowest memory:** 2.19 MB at 1M vs C++ 11.6 MB — allocator and representation differences.
2. **Python uses heapq:** So we are testing the **best** heap performance achievable in Python, not a custom Python-only heap; the gap to native is still large.
3. **Java non-linear scaling and close performance at N = 100k:** Java timings do not scale uniformly with N. At **N = 100,000**, Java’s get (pop) time (5.78 ms) is **very close** to C++ (5.43 ms) and Rust (5.06 ms) — all three sit in the same ~5–6 ms band. So at that scale Java is competitive with native code on the pop-heavy phase. From 100k to 1M, however, Java’s insert jumps from ~1.65 ms to ~22.5 ms (~13.6× for 10× N) and get from ~5.78 ms to ~80.26 ms (~14×), while C++/Rust scale closer to ~10×. So Java’s **relative** performance worsens at 1M. Likely contributors: JIT behavior and code quality at different scales, GC and allocation pressure as the heap and working set grow, and cache effects when the array no longer fits in cache. The “really close at 100k” result is consistent with the JVM doing well once warmed up at a moderate scale, before memory and GC dominate at 1M.

---

## Interpretation

### C++ and Rust: fastest, both use move semantics

- **Insert and pop (get min):** Both use contiguous storage and O(log n) sift-up/sift-down. C++ uses **`std::move`** in pop and sift; Rust uses **move-style assignments** in sift (read value, shift by assignment, write back) and one `swap` in pop before `pop()`. Both avoid unnecessary copies in the sift logic.
- **Memory:** Rust’s lower reported memory at 1M reflects different allocation and reporting.

### Java: in between, non-linear scaling and competitive at N = 100k

- Custom min-heap with backing array; JIT optimizes the hot loops. **Non-linear scaling:** Insert and get (pop) do not scale uniformly with N — the step from 100k to 1M is much steeper than from 10k to 100k (e.g. get ~5.78 ms at 100k → ~80.26 ms at 1M, ~14× for 10× N). **Close at 100k:** At N = 100,000, Java’s get time is within the same range as C++ and Rust (~5–6 ms), so Java is competitive at that scale; at 1M, GC, allocation pressure, and cache effects likely widen the gap. Memory is highest due to JVM heap usage.

### Python: heapq gives best-possible heap performance

- **`heapq`** is implemented in C (CPython); the benchmark therefore measures the **best possible** heap performance in Python. Insert and pop are still much slower than native due to interpreter and call overhead.

---

## Pitfalls and limitations

- **“Get” is not random index:** The heap benchmark **get** phase is **repeated pop of the minimum** (drain the heap), not `get(i)` at random indices. Heaps do not support O(1) random access; they support O(1) peek at the root and O(log n) pop.
- **Memory:** Process RSS or heap used, not isolated to the heap structure; cross-language comparison is approximate.
- **Move vs swap:** For non-trivial element types, implementations that use **move** in pop and sift-down (C++ `std::move`, Rust assignment-based shift) are preferable to repeated full copy/swap for performance.

---

## Methodology note

C++, Java, and Rust use **custom array-backed min-heaps**; Python uses **`heapq`**. Same N scales (1k–1M), 5 runs, mean ± std, one warm-up per scale. CSV columns: N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, memory_mb (where “get” = time to pop min N times). See root [README.md](../../README.md) for disclaimers.

---

## Takeaways

| Workload type        | Best choice in this benchmark |
|----------------------|------------------------------|
| Insert + pop min     | C++ or Rust                   |
| Minimal memory       | Rust                          |
| Managed / portability| Java                          |
| Python (best heap)   | Use `heapq`; expect slower than native |

- **Throughput:** C++ and Rust are fastest for insert and for draining the heap by repeated pop min.
- **Python:** Using **heapq** tests the best possible heap performance in Python; for large N, native code is still far ahead.
- **Implementation:** Both C++ and Rust use **move semantics** in the sift logic (C++ `std::move`, Rust assignment-based shift); prefer that over repeated copy/swap to improve performance.
