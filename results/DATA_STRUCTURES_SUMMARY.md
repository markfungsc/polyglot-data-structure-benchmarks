# Data structures benchmark — summary and cross-language patterns

This document gives a **short summary** of each data structure’s findings (complexity, who’s fastest/slowest, why, memory) and then **language-level patterns** (C++, Rust, Java, Python). Full details, methodology, and numbers are in the per-structure findings files linked below.

---

## Dynamic Array

**Full findings:** [dynamic_array/dynamic_array_findings.md](dynamic_array/dynamic_array_findings.md)

### Theoretical complexity

| Operation   | Complexity   |
|------------|--------------|
| Insert (append) | O(1) amortized |
| Get (index)    | O(1)         |

### Benchmark results (N = 1M)

- **C++** — fastest (tied with Rust on insert/get).
- **Rust** — fastest (tied with C++); **lowest memory**.
- **Java** — mid (competitive get, slower insert); **highest memory**.
- **Python** — slowest (orders of magnitude slower).

### Why?

- **C++ / Rust:** Contiguous `std::vector` / `Vec`, reserve and amortized growth, no GC, AOT-compiled.
- **Rust:** Compact layout and allocator → lowest reported memory.
- **Java:** JIT optimizes array access; get close to C++/Rust; object/heap overhead → high memory.
- **Python:** Interpreter and per-element call overhead; no tight machine-code loop.

### Memory usage

Rust lowest → C++ moderate → Java high → Python high (Rust ~2.25 MB, C++ ~15 MB, Java ~126 MB at 1M).

---

## Min Heap

**Full findings:** [heap/heap_findings.md](heap/heap_findings.md)

### Theoretical complexity

| Operation   | Complexity   |
|------------|--------------|
| Insert     | O(log n)     |
| Peek       | O(1)        |
| Pop (min)  | O(log n) per pop |

*Benchmark “get” = N consecutive pops (drain heap), so O(n log n) total.*

### Benchmark results (N = 1M)

- **C++** — fastest insert and pop.
- **Rust** — close to C++; **lowest memory**.
- **Java** — mid; **non-linear scaling** (competitive at N=100k, worse at 1M); highest memory.
- **Python** — slowest (uses `heapq`, C-backed, but interpreter overhead).

### Why?

- **C++:** Contiguous array, move semantics in pop/sift-down, no GC.
- **Rust:** Same; move-style assignments in sift; allocator → very low memory.
- **Java:** Custom array-backed heap; JIT helps at 100k; at 1M, GC and cache effects hurt; object overhead.
- **Python:** `heapq` is best possible in Python (C impl); per-call overhead dominates.

### Memory usage

Rust lowest → C++ moderate → Python higher → Java highest (Rust ~2.2 MB, C++ ~12 MB, Java ~126 MB at 1M).

---

## Linked List (singly-linked, head + tail)

**Full findings:** [linked_list/linked_list_findings.md](linked_list/linked_list_findings.md)

### Theoretical complexity

| Operation        | Complexity | Notes                          |
|------------------|------------|--------------------------------|
| Insert (push_back) | O(1)     | With tail pointer.             |
| Get (traverse)   | O(n)       | One full scan; **not** get(i) in a loop (that would be O(n²)). |
| Delete           | O(n) for last/arbitrary | Need predecessor; delete head O(1). |

### Benchmark results (N = 1M)

- **C++** — fastest insert and traverse.
- **Rust** — close; slightly lower memory than C++.
- **Java** — mid; high variance; delete phase sometimes very fast.
- **Python** — slowest; highest per-node overhead.

### Why?

- **C++ / Rust:** Plain structs, raw pointers / `Box`, minimal per-node overhead; no GC.
- **Java:** Object header and reference per node; GC; JIT can optimize but variance is high.
- **Python:** Large per-object overhead (PyObject header, etc.); interpreter cost.

### Memory usage

Rust ≈ C++ (moderate, ~36–38 MB at 1M) → Java high → Python high (~90–98 MB at 1M). Per-node overhead: C++/Rust minimal; Java object header; Python much larger.

---

## HashMap (separate chaining)

**Full findings:** [hashmap/hashmap_tests_findings.md](hashmap/hashmap_tests_findings.md)

### Theoretical complexity

| Operation | Complexity (average) |
|-----------|----------------------|
| Insert    | O(1)                 |
| Get       | O(1)                 |

*Degrades with poor hash distribution (low-entropy / collisions) and high load factor.*

### Benchmark results (N = 1M)

- **C++** — fastest insert.
- **Rust** — fastest get and **lowest memory**; insert slower than C++.
- **Java** — mid on time; **highest memory**.
- **Python** — slowest (insert and get).

### Why?

- **C++:** Chaining with reserve, bulk resize; cache locality and allocation strategy.
- **Rust:** Very fast get (layout/codegen); insert path differs (allocation strategy).
- **Java:** JIT helps; allocator/GC and boxed types cost; object overhead → high memory.
- **Python:** Interpreter and object overhead; much slower than AOT.

### Memory usage

Rust lowest → C++ moderate → Java high → Python high (Rust ~82 MB, C++ ~130 MB, Java ~1763 MB at 1M).

---

## LRU Cache (hash map + access-ordered list)

**Full findings:** [lru_cache/lru_cache_findings.md](lru_cache/lru_cache_findings.md)

### Theoretical complexity

| Operation   | Complexity |
|------------|------------|
| put_miss   | O(1)       |
| put_hit    | O(1)       |
| get_hit    | O(1)       |
| get_miss   | O(1)       |
| eviction   | O(1)       |

### Benchmark results (N = 1M, by metric)

- **C++** — fastest on put_miss, put_hit, get_hit, get_miss; eviction second to Java.
- **Rust (native)** — close to C++; **Rust (custom)** 2–3× slower (Rc/RefCell overhead).
- **Java** — **eviction fastest** at 100k and 1M; **get_hit ≈ C++/Rust** at 1M; put_miss/put_hit mid then worse at 1M; **get_miss degrades** at high N; **highest memory**.
- **Python** — slowest on time; moderate memory vs Java.

### Why?

- **C++:** Custom `unordered_map` + doubly-linked list; standard pattern; good locality and allocation.
- **Rust (native):** `lru` crate, no Rc/RefCell in hot path; **Rust (custom):** educational, Rc/RefCell cost and high memory.
- **Java:** LinkedHashMap; eviction is pointer manipulation, minimal allocation → JVM optimizes well; put/get involve ordering updates and allocation; get_miss degrades (hashing/GC at high N); object overhead → high memory.
- **Python:** OrderedDict (C-backed); interpreter overhead.

### Memory usage

C++ lowest (among real implementations) → Rust (native) moderate → Rust (custom) very high → Java highest (C++ ~76 MB, Java ~2620 MB at 1M). *Rust custom is an outlier due to Rc/RefCell.*

---

## Concurrency (bounded producer–consumer queue)

**Full findings:** [concurrency/concurrency_findings.md](concurrency/concurrency_findings.md)

*Not a data structure per se; benchmark compares bounded blocking-queue throughput across C++, Java, and Rust (Python omitted — GIL).*

### What is measured

- **Setup:** One bounded queue (capacity 4096), P producer threads and C consumer threads; 100k items total. Configs: (1,1), (2,2), (4,4), (8,8), (4,1), (1,4).
- **Implementations:** C++ and Rust use **custom** mutex + two condition variables (not_full, not_empty). Java uses **`ArrayBlockingQueue`** (optimized standard library).

### Benchmark results (throughput by config)

- **Java** — **highest throughput** in almost every config; **scales with threads** (e.g. (2,2) and (4,4) faster than (1,1)). Asymmetric (4,1) and (1,4) stay near (1,1).
- **C++** — strong at (1,1) and (4,4); (2,2) and (8,8) show variance. Asymmetric configs much slower (~1 M/s) due to many block/wake cycles.
- **Rust** — best at (1,1); **throughput falls as P and C increase** (single mutex contention). Asymmetric (4,1)/(1,4) **very slow** (~0.2 M/s); fair, non-spinning mutex under heavy blocking.

### Why?

- **Java:** ArrayBlockingQueue is a tuned, lock-optimized structure; JIT and library design give best throughput and scaling in this benchmark.
- **C++ / Rust:** Hand-rolled mutex+condvar is correct but a single lock; more threads → more contention. Rust’s `std::sync::Mutex` does not spin and is fair → larger drop with many threads and in asymmetric configs.

### Memory

Rust ~1.2–1.6 MB (process); C++ ~2–8 MB; Java ~99–295 MB (JVM heap, not directly comparable).

---

# Language features and patterns (cross-structure)

These patterns recur across the five data structures and the concurrency benchmark and explain why the same language often lands in a similar place (e.g. C++/Rust fast, Java mid with high memory, Python slowest; concurrency is an exception where Java leads).

## C++

- **Contiguous memory / control:** `std::vector`, raw pointers, custom nodes → predictable layout and cache use.
- **Move semantics:** Used in heap (pop/sift), avoids extra copies.
- **No GC:** No pause or allocation overhead from a collector.
- **Result:** Consistently **fastest or tied for fastest** on time; **low or moderate memory** (often higher than Rust due to allocator/reporting).

## Rust

- **Contiguous / compact layout:** `Vec`, `HashMap`, and native `lru` crate → good locality.
- **No GC, no Rc in hot path (when using native libs):** Native LRU and standard collections avoid ref-count cost in the hot path; **custom Rc/RefCell** (e.g. Rust LRU custom) is slower and heavier.
- **Bounds checks:** Can add cost vs C++ in theory; in practice often optimized away or small; Rust still **lowest or very low memory** in most benchmarks (allocator, representation).
- **Result:** **Tied or close to C++** on time; **lowest memory** in almost every structure; **custom vs library** choice matters (e.g. LRU).

## Java

- **JIT:** After warm-up, hot loops (array access, get, eviction) can approach native; **competitive at mid N** (e.g. heap at 100k, LRU get_hit at 1M, LRU eviction at 100k/1M).
- **Non-linear scaling:** From 100k to 1M, many metrics scale worse than 10× (GC, allocation pressure, cache effects) → **“mid” at mid N, sometimes worse at 1M** (e.g. put_miss, get_miss).
- **Object overhead:** Per-node/entry headers, boxed types, LinkedHashMap nodes → **highest memory** in almost every benchmark.
- **Variance:** High standard deviations (JIT warm-up, GC pauses) → “Java mid” or “Java fastest on eviction” can vary run to run.
- **Result:** **Middle** on time in most structures; **best on some metrics at some N** (e.g. LRU eviction, LRU get_hit at 1M); **highest memory** consistently.

## Python

- **Interpreter overhead:** Every operation is a Python call; no tight machine-code loop → **slowest** on time in every structure.
- **Per-object overhead:** PyObject header and runtime cost → **high memory** and per-element cost (linked list, hash map, etc.).
- **Standard library helps but doesn’t close the gap:** `heapq`, `OrderedDict`, NumPy-backed array still leave Python **orders of magnitude** slower than AOT languages.
- **Result:** **Slowest** on time; **high memory**; suitable for prototyping and correctness, not for throughput at large N.

---

# Links to full findings

| Structure       | Findings file |
|----------------|----------------|
| Dynamic Array  | [dynamic_array/dynamic_array_findings.md](dynamic_array/dynamic_array_findings.md) |
| Min Heap       | [heap/heap_findings.md](heap/heap_findings.md) |
| Linked List    | [linked_list/linked_list_findings.md](linked_list/linked_list_findings.md) |
| HashMap        | [hashmap/hashmap_tests_findings.md](hashmap/hashmap_tests_findings.md) |
| LRU Cache      | [lru_cache/lru_cache_findings.md](lru_cache/lru_cache_findings.md) |
| Concurrency    | [concurrency/concurrency_findings.md](concurrency/concurrency_findings.md) |

Each findings file contains: methodology, Big O, numbers at a glance, interpretation by language, pitfalls, and takeaways. Benchmark CSVs live in `raw/` under each structure folder (or in [results/raw/](raw/) for the default run); plots in `plots/` under each structure folder.
