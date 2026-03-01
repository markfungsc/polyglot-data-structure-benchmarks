# Dynamic Array benchmark findings (methodology & summary)

This document covers: **Dynamic Array** benchmark result comparison across Python, Java, C++, and Rust; **methodology** (warm-up, 5 runs, mean ± std, memory measurement). See [methodology.md](../../docs/methodology.md) for full definitions. Results below come from `raw/*_dynamic_array.csv` in this folder; plots are in `plots/`.

---

# Findings: Dynamic Array benchmark comparison

Summary of results from `raw/*_dynamic_array.csv` (main scenario: scaled N, warm-up, mean ± std over 5 runs). All four implementations use **contiguous or array-backed storage** (custom code). Data from current `raw/` CSVs; plots in `plots/`.

---

## Numbers at a glance (N = 1,000,000)

| Language | Insert (ms) | Get (ms) | Memory (MB) |
|----------|-------------|----------|-------------|
| C++      | 1.62 ± 0.27 | 0.22 ± 0.08 | 15.3  |
| Python   | 166 ± 6.4   | 84.5 ± 7.2  | 76.9  |
| Java     | 9.04 ± 0.94 | 0.23 ± 0.02 | 125.6 |
| Rust     | 1.68 ± 0.12 | **0.21 ± 0.02** | **2.25** |

---

## Summary of test results

- **Main scenario (scaled N):** Insert and get scale roughly with N across languages. C++ and Rust are fastest on insert (~1.6 ms at 1M) and get (~0.21–0.22 ms at 1M). Rust uses the least memory (2.25 MB at 1M). Python is slowest on insert (166 ms) and get (84.5 ms at 1M). Java is in between on time (9 ms insert, 0.23 ms get) but reports the highest memory (125.6 MB). See the table above and `plots/`: dynamic-array_insert_log.png, dynamic-array_get_log.png, dynamic-array_memory_log.png.

---

## Expected vs unexpected behaviors

**Expected (with data):**

1. **C++/Rust fastest:** Contiguous buffers, no GC, AOT-compiled. Insert ~1.6 ms, get ~0.21 ms at 1M (raw/cpp_dynamic_array.csv, raw/rust_dynamic_array.csv).
2. **Python slowest:** Interpreter and per-element overhead. Insert 166 ms, get 84.5 ms at 1M (raw/python_dynamic_array.csv).
3. **Rust lowest memory:** Compact layout and allocator; 2.25 MB at 1M vs C++ 15.3 MB, Java 125.6 MB (memory_log.png).
4. **Java competitive on time, higher memory:** JIT-optimized loops; heap/object overhead (9 ms insert, 0.23 ms get, 125.6 MB at 1M).

**Unexpected or notable:**

1. **C++ and Rust nearly tied on insert and get** at 1M (both ~1.6 ms insert, ~0.21 ms get). Both use contiguous vectors with similar O(1) append and index access.
2. **Java get close to C++/Rust** (0.23 ms) despite GC; array access is well-optimized by the JIT.

---

## Possible explanations (with data)

- **C++/Rust lead:** Contiguous `std::vector` / `Vec`, reserve and amortized growth, no GC. Insert and get at 1M: ~1.6 ms and ~0.21 ms (raw/cpp_dynamic_array.csv, raw/rust_dynamic_array.csv).
- **Rust lowest memory:** Efficient default allocator and compact representation; 2.25 MB at 1M vs C++ 15.3 MB (raw/rust_dynamic_array.csv vs raw/cpp_dynamic_array.csv).
- **Java:** JIT compiles the hot loops; primitive int array or equivalent gives fast get. Insert 9 ms, get 0.23 ms, memory 125.6 MB (heap used, not process RSS).
- **Python:** Per-call overhead for each push/get; NumPy storage helps but operations are still at interpreter level (see Drawbacks below).

---

## Interpretation

### C++ and Rust: tied at the top

C++ and Rust are **fastest on both insert and get** (~1.6 ms insert, ~0.21 ms get at 1M):

- **Contiguous storage:** `std::vector<int>` and `Vec<i32>`; O(1) amortized append and O(1) index access.
- **No runtime overhead:** AOT-compiled, no GC, inlined loops.
- **Resize strategy:** Double-on-full (or similar) keeps amortized cost low.

Rust has **much lower reported memory** (2.25 MB vs 15.3 MB at 1M). Differences in allocator, reporting (RSS vs heap), or binary layout can explain this; both store 1M ints in a contiguous block.

---

### Java: competitive time, higher memory

Java is **between native and Python** on time (9 ms insert, 0.23 ms get at 1M) and **highest on memory** (125.6 MB):

- **JIT:** After warm-up, array access and append loops are optimized.
- **Heap measurement:** Memory column reflects heap used (e.g. `Runtime.totalMemory() - freeMemory()`), not just the array; JVM and object overhead inflate the number compared to C++/Rust RSS.

So for **managed, portable code**, Java offers good dynamic-array performance with higher memory footprint.

---

### Python: orders of magnitude slower

Python is **~100× slower than C++/Rust on insert** and **~400× slower on get** at 1M:

- **Interpreter:** Each push and get is a Python call; no tight machine-code loop.
- **NumPy-backed but element-wise:** Storage is contiguous (NumPy array), but the benchmark uses Python-level push/get (see Drawbacks below). No bulk insertion in the timed path.

Python is suitable for **prototyping and correctness**; for throughput at large N, compiled languages dominate.

---

## Drawbacks, limitations, and adjustments

- **Python implementation:** Uses NumPy-backed contiguous memory but performs **element-wise operations at interpreter level** (each `push` and `get` is a Python call). This improves performance over a pure Python list, but there is **no bulk insertion** (e.g. no `np.append` or batch insert in the timed path), so it is not optimized NumPy usage.

- **Memory:** Measurements represent **process RSS or heap used**, not isolated structure memory. Java reports heap; C++/Rust typically report process RSS. Cross-language memory numbers are not directly comparable.

- **Bounds checking:** Semantics differ across languages (e.g. Python raises `IndexError`; C++/Rust may assert or have undefined behavior). Benchmarks use in-bounds access; real-world safety and cost vary.

- **Runtime overhead:** Results include **JIT warmup, interpreter overhead, and GC behavior** and therefore do not measure **pure data structure complexity alone**. See the root [README.md](../../README.md) for the full disclaimer and limitations.

---

## Methodology note

All four use **custom dynamic array** implementations (contiguous or array-backed). Results depend on:

- Compiler and flags (C++/Rust release builds),
- JVM and GC (Java),
- Python and NumPy version.

The comparison is **like-for-like algorithms** (amortized O(1) append, O(1) index get). For std-library comparisons (e.g. `std::vector` vs `Vec` only), interpretation would be similar; adding other languages or allocators would change numbers.

---

## Takeaways

| Workload type           | Best choice in this benchmark |
|------------------------|--------------------------------|
| Insert-heavy, max speed| C++ or Rust                    |
| Get-heavy, max speed   | Rust or C++                    |
| Minimal memory         | Rust (2.25 MB at 1M)           |
| Managed / portability  | Java                           |
| Prototyping / scripting| Python                         |

- **Insert- or get-heavy:** C++ and Rust are effectively tied and fastest.
- **Minimal memory:** Rust reports the lowest footprint here.
- **Managed runtime:** Java is competitive on time with higher memory.
- **Python:** Much slower; NumPy helps over a list but the benchmark is not bulk-optimized.
