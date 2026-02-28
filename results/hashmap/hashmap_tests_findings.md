# HashMap benchmark findings (methodology & test cases)

This document covers: **HashMap** benchmark result comparison across Python, Java, C++, and Rust; **methodology** (warm-up, 5 runs, mean ± std, memory measurement); and **test cases** (main scaled scenario, low-entropy / near-collision, load-factor sensitivity). See [scenarios.md](../../benchmarks/scenarios.md) and [methodology.md](../../docs/methodology.md) for full definitions.

---

# Findings: HashMap benchmark comparison

Summary of results from `raw/*_hashmap.csv` (main scenario: random keys, scaled N, warm-up, mean ± std over 5 runs). All four implementations use **separate chaining** (custom code, not std-library hash tables). Data below from current `raw/` CSVs; plots in `plots/`.

---

## Numbers at a glance (N = 1,000,000)

| Language | Insert (ms) | Get (ms) | Memory (MB) |
|----------|-------------|----------|-------------|
| Python   | 3610 ± 163  | 4548 ± 228 | 473   |
| Java     | 255 ± 17    | 118 ± 23  | 1763 |
| C++      | 57.6 ± 4.4  | 16.6 ± 1.5 | 130  |
| Rust     | 240 ± 41    | **2.1 ± 0.14** | **82**  |

---

## Summary of test results

- **Main scenario (scaled N, random keys):** Insert and get scale roughly with N across languages. C++ is fastest on insert (57.6 ms at 1M), Rust fastest on get (2.1 ms at 1M) and lowest memory (82 MB). Python is slowest (3610 ms insert, 4548 ms get at 1M). See the table above and `plots/`: insert_log.png, get_log.png, memory_log.png.

- **Low-entropy / near-collision (capacity=64):** With a fixed small capacity, many keys collide; insert times increase vs main at N=1M: C++ 185 ms vs 58 ms, Rust 293 ms vs 240 ms, Java 392 ms vs 255 ms, Python 4743 ms vs 3610 ms. Get also degrades (e.g. C++ get 19.9 ms vs 16.6 ms). Data from `*_hashmap_low_entropy.csv`; plot: low_entropy_insert_log.png.

- **Load-factor sensitivity (N=100k):** From `*_hashmap_loadfactor.csv`, insert time varies with load factor (0.25–1.0). C++ insert is best at load factor 0.5 (0.87 ms), higher at 1.0 (3.79 ms). Rust insert is relatively flat (~2–7 ms). Java and Python are much higher and more variable. Plot: loadfactor_insert.png.

---

## Expected vs unexpected behaviors

**Expected (with data):**

1. **Python/Java slower than AOT C++/Rust:** At 1M, Python insert 3610 ms vs C++ 57.6 ms (~63×); Java get 118 ms vs Rust 2.1 ms (~56×). Supported by main scenario CSVs.
2. **Rust get very fast:** 2.1 ms at 1M; flat layout, no GC. get_log.png shows Rust get well below others.
3. **C++ insert fast:** 57.6 ms at 1M; chaining with reserve. insert_log.png shows C++ insert lowest.
4. **Low-entropy degrades performance:** At 1M, C++ insert 185 ms (low_entropy) vs 57.6 ms (main); long chains cause more scans. Data from raw/cpp_hashmap_low_entropy.csv vs raw/cpp_hashmap.csv.
5. **Java high memory:** 1763 MB at 1M (heap, object headers, JVM). memory_log.png and raw/java_hashmap.csv memory_mb.

**Unexpected or notable:**

1. **Rust get ~8× faster than C++ get at 1M** (2.1 ms vs 16.6 ms) despite both using chaining. Likely contributors: compiler optimization of the inner loop, cache behavior of Rust’s layout, or allocation pattern differences.
2. **Load-factor sensitivity differs by language:** C++ insert best at mid load factor (0.87 ms at 0.5); Rust insert relatively stable (~2–7 ms across 0.25–1.0). raw/cpp_hashmap_loadfactor.csv vs raw/rust_hashmap_loadfactor.csv.

---

## Possible explanations (with data)

- **C++ insert lead:** Per-bucket preallocation (see bonus finding below), simple append, bulk resize. Insert at 1M: C++ 57.6 ms vs Rust 240 ms, Java 255 ms (from main CSVs).
- **Rust get lead:** Contiguous layout, no GC pauses, efficient codegen. Get at 1M: Rust 2.1 ms vs C++ 16.6 ms (from raw/rust_hashmap.csv, raw/cpp_hashmap.csv).
- **Java:** JIT helps but allocator/GC and boxed types cost. Insert 255 ms, get 118 ms, memory 1763 MB at 1M (raw/java_hashmap.csv).
- **Python:** Interpreter and object overhead. Insert 3610 ms, get 4548 ms at 1M vs C++ 57.6 ms / 16.6 ms (raw/python_hashmap.csv vs raw/cpp_hashmap.csv).
- **Low-entropy:** Few buckets → long chains → more linear scans. C++ insert at 1M: 57.6 ms (main) vs 185 ms (low_entropy); raw/rust_hashmap_low_entropy.csv shows similar degradation.
- **Load factor:** C++ insert 0.87 ms at load factor 0.5 vs 3.79 ms at 1.0 (raw/cpp_hashmap_loadfactor.csv) illustrates realloc/collision tradeoff; more buckets at low LF reduce collisions but more resize work at high LF.

---

## Interpretation

### C++ insert winner

The C++ chaining implementation is the **fastest on insert** (57.6 ms at 1M):

- **Allocation-efficient:** `vector<vector<pair<int,int>>>` with reserved buckets; resizes in bulk. Per-bucket preallocation (see **Lesson learnt** below) significantly improves insert performance.
- **Cache-friendly:** Contiguous buckets, then contiguous pairs in each chain.
- **Compiled with optimization:** No runtime overhead; inlined, no GC.
- **Simple logic:** Hash → bucket → linear scan or append; few branches.

So at 1M elements, C++ insert is about **4× faster than Rust** and **4× faster than Java** in this benchmark.

---

### Rust get winner (by a lot)

Rust **get is ~8× faster than C++** at 1M (2.1 ms vs 16.6 ms), and orders of magnitude faster than Java and Python:

- **Flat, predictable layout:** Buckets and chains are in contiguous memory; good cache line use.
- **No pointer chasing** beyond the bucket index; chains are `Vec<Entry>` in place.
- **Compiled, no GC:** No runtime overhead during the get loop.
- **Low variance:** Small std dev (0.14 ms at 1M) suggests stable, predictable codegen and memory access.

So for **lookup-heavy workloads**, this Rust chaining implementation is the clear winner among the four.

---

### Why C++ wins insert but Rust wins get

- **Insert:** C++ does minimal work per insert (hash, find bucket, push_back). Rust's logic is similar but may do more work per insertion (e.g. growth policy, rehashing, or more checks). Small differences in resize strategy and allocation add up at 1M elements.
- **Get:** Rust's get loop is extremely cheap: same chaining idea, but the way the compiler optimizes the inner loop (and possibly better cache behavior of the Rust layout) yields much lower get times. So in this codebase, **Rust's chaining get is far ahead** even though both use chaining.

So:

- **Insert-heavy workloads:** C++ chaining is the fastest here.
- **Lookup-heavy workloads:** Rust chaining dominates in these benchmarks.

---

### Java: competitive, managed runtime

Java sits between C++/Rust and Python:

- **Insert 255 ms, Get 118 ms at 1M** — much faster than Python, slower than C++/Rust.
- **Memory 1763 MB** — heap overhead and object headers (Integer keys/values) plus JVM bookkeeping explain the higher footprint.
- **JIT helps:** After warm-up, the tight loops get compiled; still can't match AOT-compiled C++/Rust or avoid GC/allocator overhead.

So for **managed, portable code**, Java is a reasonable tradeoff: no manual memory management, but slower and more memory than the compiled implementations.

---

### Python: orders of magnitude slower

Python is **~63× slower than C++ on insert** and **~2200× slower than Rust on get** at 1M:

- **Interpreted + dynamic:** Every operation is a Python call and hash/lookup; no tight machine-code loop.
- **Object overhead:** Integers and container types are heap-allocated objects; no compact array of ints.
- **GIL and runtime:** No low-level control over memory layout or cache; implementation is for clarity and correctness, not raw speed.

So Python is suitable for **prototyping and correctness**, not for throughput-sensitive hashmap workloads at large N.

---

### Lesson learnt: C++ bucket preallocation

Adding a **small per-bucket preallocation** in the C++ hashmap constructor significantly improved insert performance.

- **What:** In [cpp/src/hashmap.cpp](../../cpp/src/hashmap.cpp), after `buckets_.resize(capacity_)`, each bucket is initialized with `bucket.reserve(2)` (lines 6–9). So every chain reserves space for two entries at creation time.
- **Why it helps:** As the first few entries are added to each chain, the vector does not reallocate; the hot insert path avoids repeated growth and copy. The cost is a small, one-time allocation per bucket at construction.
- **Result:** C++ insert is significantly faster with this change. The current C++ insert lead (57.6 ms at 1M vs Rust 240 ms, Java 255 ms) is partly due to this optimization. Lesson: even a small, predictable reserve in a hot path can yield large gains when scaling to millions of operations.

---

## Takeaways

| Workload type              | Best choice in this benchmark |
|----------------------------|--------------------------------|
| Insert-heavy, max speed    | C++ chaining                  |
| Lookup-heavy, max speed    | Rust chaining                  |
| Need minimal memory       | Rust (82 MB at 1M)             |
| Managed runtime / portability | Java                       |
| Prototyping / scripting   | Python                         |

- **Insert-heavy:** C++ chaining is extremely fast here.
- **Lookup-heavy:** Rust's chaining get dominates; flat, cache-friendly access and no GC.
- **Managed runtime:** Java is competitive but slower and higher memory.
- **Dynamic language:** Python is orders of magnitude slower; use when productivity matters more than raw throughput.

---

## Methodology note

All four use **custom separate chaining** (no std `HashMap`/`dict` in the timed path). Results depend on:

- Compiler and flags (C++/Rust release builds),
- JVM and GC (Java),
- Python version and interpreter.

So the comparison is **like-for-like algorithms**, not "Rust std vs C++ std." For std-library comparisons (e.g. Rust's default open-addressing HashMap vs this C++ chaining), results and interpretation would differ.
