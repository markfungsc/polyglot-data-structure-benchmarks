# LRU cache benchmark findings (methodology & summary)

This document covers: **LRU cache** benchmark result comparison across Python, Java, C++, and Rust; **methodology** (warm-up, 5 runs, mean ± std, memory); **complexity** (Big O for get/put and eviction); **implementation notes** (two Rust variants — self-implemented with Rc/RefCell vs native `lru` crate; C++ standard-style; Java and Python standard libraries). See [methodology.md](../../docs/methodology.md) for definitions. Results come from `raw/*_lru_cache.csv` and `raw/rust_native_lru_cache.csv` in this folder; plots are in `plots/`.

---

## Complexity (Big O) and benchmark design

All implementations provide **O(1)** get and put in the average case, using a hash map plus a doubly-linked (or ordered) structure to track access order. The benchmark measures five distinct workloads.

| Operation   | Big O | What the benchmark measures |
|------------|--------|-----------------------------|
| **put_miss** | O(1) | Insert of a **new** key (cache had space; no eviction). |
| **put_hit**  | O(1) | Update of an **existing** key (value replacement + move to MRU). |
| **get_hit**  | O(1) | Lookup of an **existing** key (read + move to MRU). |
| **get_miss** | O(1) | Lookup of a **missing** key (hash lookup only; no list update). |
| **eviction** | O(1) | Put that causes **removal of the LRU** entry (cache full; insert new key). |

**CSV columns:** N, put_miss_mean_ms, put_miss_std_ms, put_hit_mean_ms, put_hit_std_ms, get_hit_mean_ms, get_hit_std_ms, get_miss_mean_ms, get_miss_std_ms, eviction_mean_ms, eviction_std_ms, memory_mb. For put_miss we time **(capacity−1)** inserts into an empty cache (no eviction). For put_hit, get_hit, get_miss, and eviction we time **N** operations per run at scale N.

---

## Implementation notes by language

**Rust — two variants:**

1. **Self-implemented (custom):** Doubly-linked list with **`Rc<RefCell<Node>>`** and a `HashMap` for O(1) key→node lookup. Used for studying ownership and interior mutability; **not** optimized for speed — `Rc`/`RefCell` add runtime overhead. Head = LRU, tail = MRU; get/put move the node to the tail.
2. **Native library:** The **`lru`** crate (e.g. 0.16) — production-oriented, no `Rc`/`RefCell` in the hot path. Typically faster and lower memory than the custom implementation. The benchmark writes **rust_native_lru_cache.csv** so both variants appear in plots (Rust vs Rust (native)).

**C++:** Custom implementation using **`std::unordered_map<int, Node*>`** and a doubly-linked list of nodes (raw pointers). Head = MRU, tail = LRU; standard approach: hash map for O(1) lookup, list for order and O(1) eviction. No standard-library LRU container; this pattern is the usual way to implement LRU in C++.

**Java:** **`LinkedHashMap`** with `accessOrder = true` and **`removeEldestEntry`** overridden. Standard, idiomatic way to get an LRU cache in Java: insertion-order or access-order iteration plus automatic eviction when size exceeds capacity. No custom node type; the standard library handles the linked structure.

**Python:** **`collections.OrderedDict`** (C implementation in CPython). No built-in “LRU cache” type with explicit get/put; `OrderedDict` plus `move_to_end` and `popitem(last=False)` gives O(1) get/put and eviction. **`functools.lru_cache`** is a decorator for memoization and does not expose explicit get/put, so it is not used in this benchmark.

---

# Findings: LRU cache benchmark comparison

Summary of results from `raw/*_lru_cache.csv` and `raw/rust_native_lru_cache.csv` (scaled N, warm-up, mean ± std over 5 runs). **Rust** is represented twice: custom (Rc/RefCell) and **native `lru` crate**. Data from current `raw/` CSVs; plots in `plots/`.

---

## Numbers at a glance (N = 1,000,000)

Times in ms (mean ± std). put_miss = time for (capacity−1) inserts; others = time for N operations.

| Implementation | put_miss (ms) | put_hit (ms) | get_hit (ms) | get_miss (ms) | eviction (ms) | Memory (MB) |
|----------------|---------------|--------------|--------------|---------------|---------------|-------------|
| C++            | 21.77 ± 1.06 | 23.86 ± 1.56 | 24.03 ± 3.05 | 1.98 ± 0.05   | 82.59 ± 5.09 | 76.5        |
| Rust (native)  | 29.04 ± 2.04 | 32.30 ± 1.64 | 24.17 ± 0.74 | 1.23 ± 0.08   | 106.26 ± 2.17| 266.1       |
| Rust (custom)  | 56.63 ± 7.74 | 67.75 ± 8.86 | 75.41 ± 17.75| 4.91 ± 0.29   | 117.41 ± 9.76| 1792.4       |
| Java           | 62.42 ± 40.38| 36.91 ± 31.30| 24.32 ± 18.94| 17.34 ± 14.76 | 36.03 ± 20.39| 2620.3      |
| Python         | 131.68 ± 3.56| 131.17 ± 1.09| 107.98 ± 1.31| 44.29 ± 2.81  | 217.91 ± 11.85| 263.9      |

---

## Summary of test results

- **put_miss / put_hit:** C++ and Rust (native) are fastest at 1M; Rust (custom) 2–3× slower. **Java** is in the **middle** at 10k–100k, then **falls behind** at 1M with non-linear scaling and high variance. Python slowest.
- **get_hit:** At 1M, **Java ≈ C++ ≈ Rust native** (~24 ms) — Java **approaches** native. At smaller N Java is middle or slower. Rust (custom) and Python slower.
- **get_miss:** C++ and Rust (native) fastest at 1M (~1–2 ms). **Java degrades at high N** (~17 ms at 1M, much worse than C++/Rust). Python slowest.
- **eviction:** **Java is fastest at 100k and 1M** (3.1 ms and 36 ms vs C++ 4.0/83, Rust 4.0/106). At 1k Java is slowest. So Java **improves relative to others** as N grows on this metric. Python slowest; high variance on Java.
- **Memory:** C++ and Rust (native) moderate at 1M; Rust (custom) very high (1792 MB). **Java highest** at all N (JVM heap).

---

## Expected vs unexpected behaviors

**Expected:**

1. **C++ and Rust (native) fastest:** AOT-compiled, hash map + linked list, no Rc/RefCell in the hot path for the native crate.
2. **Rust (custom) slower and heavier:** The self-implemented cache uses `Rc<RefCell<>>` for shared mutable nodes; this adds indirection and ref-count updates on every get/put, and increases memory.
3. **Python slowest on time:** OrderedDict is C-backed but interpreter and call overhead dominate at large N.
4. **get_miss cheapest:** No list updates; just hash lookups returning “not found.”

**Notable:**

1. **Two Rust lines in plots:** “Rust” = custom (Rc/RefCell); “Rust (native)” = `lru` crate. Comparing them shows the cost of the educational implementation vs a library tuned for performance.
2. **Java LinkedHashMap:** Behaviour **depends on metric and N**: get_hit **approaches** C++/Rust at 1M; eviction **fastest** at 100k and 1M; put_miss/put_hit **middle** at mid scales, **worse** at 1M; get_miss **degrades** at high N; memory **highest** at all N. See “Java: how it compares across N and by metric” below.
3. **C++ no standard LRU:** The “standard way” in C++ is this pattern: `unordered_map` + doubly-linked list; the benchmark uses a custom implementation following that pattern.

---

## Interpretation

### C++: standard pattern (hash map + doubly-linked list)

- Custom LRU using **`std::unordered_map<int, Node*>`** and a doubly-linked list with raw pointers. No standard-library LRU type; this is the usual approach. Fast and moderate memory.

### Rust: custom vs native

- **Custom:** Doubly-linked list with **Rc&lt;RefCell&lt;Node&gt;&gt;** and `HashMap`. Good for studying ownership and interior mutability; **not** for peak performance — ref-counts and borrow checks at runtime add cost.
- **Native:** **`lru`** crate — avoids Rc/RefCell in the hot path, typically uses a single backing store. Use this when you need a fast LRU cache in Rust.

### Java: LinkedHashMap — how it compares across N and by metric

Java does **not** follow a single pattern (e.g. “slower everywhere” or “eviction always fastest”). Depending on **which metric** and **which N**, Java is sometimes **closer to C++/Rust**, sometimes **in the middle**, sometimes **best**, and sometimes **worst**. The table below summarizes; numbers are mean ms (rounded) from the benchmark CSVs.

| Metric     | N=1k        | N=10k              | N=100k              | N=1M                    |
|------------|-------------|--------------------|----------------------|-------------------------|
| **put_miss** | Java slowest | Java middle (≈ Rust custom) | Java middle (≈ Rust custom) | Java slowest with C++/Rust native much faster |
| **put_hit**  | Java slowest | Java slower than C++/Rust, ahead of Python | Java middle (between Rust native and custom) | Java middle (between Rust native and custom); C++/Rust native faster |
| **get_hit**  | Java slowest | Java middle (worse than C++/Rust, better than Rust custom) | Java middle | **Java ≈ C++ ≈ Rust native** (~24 ms) — Java **approaches** native at 1M |
| **get_miss** | Java slow   | Java middle        | Java middle          | **Java much worse** (17 ms vs C++ 2 ms, Rust 1.2 ms) — degrades at high N |
| **eviction** | Java slowest | Java **close to Rust native** (0.28 vs 0.18 ms) | **Java fastest** (3.1 ms vs C++ 4.0, Rust 4.0) | **Java fastest** (36 ms vs C++ 83, Rust 106) |
| **memory**   | Java highest | Java highest       | Java high (between Rust native and Rust custom in “order”) | Java highest (2620 MB) |

So in words:

- **put_miss:** Java is in the **middle** at 10k–100k (between Rust native and Rust custom), then **falls behind** at 1M. Scaling from 100k to 1M is much steeper than for C++/Rust — **non-linear**.
- **put_hit:** Similar: **middle** at mid scales; at 1M still between Rust native and Rust custom, but with **high variance** (±31 ms).
- **get_hit:** Java **catches up at 1M** — essentially the same as C++ and Rust native (~24 ms). So get_hit **approaches** native performance at large N.
- **get_miss:** Java is **competitive at low N** but **degrades at high N** — at 1M it is ~8× slower than C++ and ~14× slower than Rust native. Pure hash lookups (no list update) might be hit by boxing, GC, or cache layout at large N.
- **eviction:** Java is **slowest at 1k**, then **improves relative to others**: close to Rust native at 10k, **fastest at 100k and 1M**. So eviction is the metric where Java **performs best in the middle and at high N**.
- **Memory:** Java is **highest** at all N. LinkedHashMap stores extra objects (entry Nodes, references for the double-linked list), and JVM heap overhead (object headers, alignment) increases footprint per entry — so total memory is high and does not “approach” the others.

**Why Java behaves this way, per metric:**

| Metric     | Why Java behaves this way |
|------------|---------------------------|
| **put_miss** | New insertion: LinkedHashMap must **allocate a node** and **update access order** in the linked list. At large N, **GC** can spike and scaling is non-linear. |
| **put_hit**  | Updates an existing node; **list manipulation** (move to end for MRU) is cheaper than put_miss, but still affected by object layout and ordering updates on every access. |
| **get_hit**  | Mostly **hash lookup + pointer move** in the linked list (move to end). **JIT optimizes** this path well at large N, so Java can approach C++/Rust at 1M. |
| **get_miss** | Lookup fails; **no node to move**, but **hashing + possible GC** (e.g. during probe) can slow at high N; Java degrades vs C++/Rust at 1M. |
| **eviction** | **Remove head of linked list + one map entry**; dominated by **pointer manipulation**, not hashing or new allocation. JVM optimizes this path well and it **scales well** — so Java is fastest at 100k and 1M. |
| **memory**   | **Object overhead per entry** (Node, key/value wrappers) plus **JVM heap management**; LinkedHashMap nodes and double-linked refs add extra memory per entry. |

**Why the pattern differs by metric and N:** (1) **Non-linear scaling** — JIT warm-up and GC/cache effects depend on N. (2) **Eviction** is mostly pointer ops in the linked list and does **not** require hashing or object allocation for the remove path; the JVM optimizes this well, so eviction is fastest at high N. (3) **Put vs get:** LinkedHashMap must **update the linked list for ordering on every access** (get and put both move the entry to MRU); that affects put and get differently — put_miss also allocates, put_hit only updates, get_hit is lookup + move, get_miss is lookup only. (4) **High variance** (e.g. ±20 ms on eviction, ±40 ms on put_miss): **JIT warm-up** and **GC pauses** cause run-to-run spikes; the means still show that eviction and get_hit are where Java can match or beat C++/Rust at some scales. Use **LinkedHashMap** when you need a standard LRU in Java; for cross-language comparison, look at **each metric and scale** rather than a single “Java is slower/faster” story.

**Why C++/Rust can outperform Java in some metrics despite same Big-O:** C++ and Rust (custom vs native) differ in **cache locality** and **allocation strategy** — e.g. contiguous or pool-like storage, fewer indirections, no GC — so even with the same O(1) get/put/eviction, they can be faster on put_miss, get_miss, or memory where Java pays for object overhead and GC.

### Python: OrderedDict

- **`collections.OrderedDict`** (C implementation) with `move_to_end` and `popitem(last=False)` is the usual way to get an explicit LRU cache with get/put. **`functools.lru_cache`** is for memoization and does not expose a key-value cache API for this benchmark.

---

## Pitfalls and limitations

- **put_miss vs eviction:** put_miss times **(capacity−1)** inserts with no eviction; eviction times **N** puts that each remove the LRU. So put_miss is “cheap” inserts; eviction includes the cost of list updates and removals.
- **Memory:** Process RSS or heap usage, not isolated to the cache; cross-language comparison is approximate. Rust (custom) high memory is consistent with many small Rc/RefCell allocations.
- **Rust custom is educational:** The self-implemented cache is for learning; for production use the **`lru`** crate (or similar) and expect better time and memory.

---

## Methodology note

Same N scales (1k–1M), 5 runs, mean ± std, one warm-up per scale. CSV columns: N, put_miss_mean_ms, put_miss_std_ms, put_hit_mean_ms, put_hit_std_ms, get_hit_mean_ms, get_hit_std_ms, get_miss_mean_ms, get_miss_std_ms, eviction_mean_ms, eviction_std_ms, memory_mb. **Rust** has two result files: **rust_lru_cache.csv** (custom) and **rust_native_lru_cache.csv** (native `lru` crate). See root [README.md](../../README.md) for disclaimers.

---

## Takeaways

| Workload / goal           | Best choice in this benchmark   |
|---------------------------|----------------------------------|
| Throughput (all operations) | C++ or Rust (native)          |
| Lowest memory (at 1M)     | C++                             |
| Rust production use        | Use **`lru`** crate, not Rc/RefCell custom |
| Java                      | **LinkedHashMap** (standard)     |
| Python                    | **OrderedDict** (standard); expect slower than native |

- **C++:** Standard approach is custom hash map + doubly-linked list; no std LRU type.
- **Rust:** Two implementations — **self-implemented (Rc/RefCell)** for study, **native `lru` library** for performance; both are included in the benchmark and plots.
- **Java / Python:** **Existing standard libraries** (LinkedHashMap, OrderedDict) are the usual way to implement LRU; the benchmark reflects that.
