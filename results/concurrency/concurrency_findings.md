# Concurrency benchmark findings (producer–consumer, bounded queue)

This document covers: **bounded blocking queue** producer–consumer benchmark across C++, Java, and Rust; **methodology** (fixed N and capacity, varying P and C, 5 runs, mean ± std); **implementation notes** (C++ and Rust use **custom/native** mutex + condition-variable queues; Java uses the **optimized** `ArrayBlockingQueue`); and **observed behaviour** (throughput, scaling, asymmetric configs). Python is omitted (GIL). Results come from `raw/*_concurrency.csv` in this folder; plots are in `plots/`.

---

## Benchmark design

- **Data structure:** A single **bounded blocking queue** (capacity 4096). P producer threads push items; C consumer threads pop until 100,000 items have been passed through.
- **Parameters:** total_items = 100,000, capacity = 4096. Configs (P, C): (1,1), (2,2), (4,4), (8,8), (4,1), (1,4).
- **Metrics:** Wall-clock time (elapsed_mean_ms), throughput (items/s), and memory_mb per (P, C). No work per item — we measure **queue and lock contention**.
- **CSV columns:** num_producers, num_consumers, capacity, total_items, elapsed_mean_ms, elapsed_std_ms, throughput_per_sec_mean, memory_mb.

---

## Implementation notes by language

**C++ and Rust: custom / native implementation**

- **C++:** Hand-written **bounded blocking queue**: `std::queue<int>` guarded by one **`std::mutex`** and two **`std::condition_variable`** (not_full, not_empty). Producers block when `size == capacity`; consumers block when empty (or when `done` is set after producers finish). Same pattern as a textbook producer–consumer queue.
- **Rust:** Same semantics: **`Mutex<(VecDeque<i32>, bool)>`** (queue + done flag) and two **`Condvar`** (not_full, not_empty). One lock for both queue and done to match C++ and reduce extra contention. No standard-library bounded blocking queue; this is the usual “mutex + condvar” approach.

**Java: optimized standard library**

- **Java:** **`java.util.concurrent.ArrayBlockingQueue<Integer>(capacity)`**. Producers call **`put()`**, consumers **`take()`**; blocking is built-in. Implementation is highly tuned (single ReentrantLock, two conditions, array ring buffer). **Poison pills** are used so that after producers finish, each consumer receives one sentinel and exits cleanly (avoids one consumer taking the last item and others blocking forever on an empty queue).

So: **C++ and Rust** use a **custom/native** mutex+condvar queue; **Java** uses an **optimized** standard concurrent queue. The benchmark compares “hand-rolled” vs “library” in addition to language/runtime effects.

---

## Numbers at a glance (from raw CSVs)

Throughput (items/s) and elapsed (ms). Capacity 4096, 100k items.

| Config | C++ (M/s) | Java (M/s) | Rust (M/s) |
|--------|-----------|------------|------------|
| P1 C1  | 10.3      | 7.9        | 8.3        |
| P2 C2  | 2.8       | **16.2**   | 5.0        |
| P4 C4  | 5.9       | **13.1**   | 2.8        |
| P8 C8  | 3.7       | **8.7**    | 2.4        |
| P4 C1  | 0.93      | **7.7**    | 0.21       |
| P1 C4  | 1.05      | **8.3**    | 0.21       |

Memory (MB): Rust ~1.2–1.6; C++ ~2–8; Java ~99–295 (JVM heap, not comparable to process RSS).

---

## Summary of behaviour

- **Java:** Highest throughput in almost every config. **Scales with threads** for symmetric (P,C): (2,2) and (4,4) are **faster** than (1,1). (8,8) drops but still strong. Asymmetric (4,1) and (1,4) stay near (1,1) — single consumer or single producer is the bottleneck, and `ArrayBlockingQueue` handles blocking/wakeup efficiently.
- **C++:** Strong single-thread (1,1) and good (4,4); (2,2) and (8,8) show **variance** (throughput dips). Asymmetric (4,1) and (1,4) are **much slower** (~1 M/s): one side blocks often, so more condvar wakeups and lock handoffs.
- **Rust:** Best at (1,1), then **throughput falls as P and C increase** (2,2 → 4,4 → 8,8). Asymmetric (4,1) and (1,4) are **very slow** (~0.21 M/s, ~470 ms): many wait/wake cycles on a single mutex; Rust’s `std::sync::Mutex` does not spin and is fair, so contention cost is high.

---

## Possible explanations

**Why Java wins on throughput**

- **ArrayBlockingQueue** is a mature, lock-optimized structure (single lock, two conditions, array-backed). Hot path is tuned for the JVM (inlining, lock inflation).
- JIT warms up over runs; with multiple threads the JVM can show good throughput once optimised.
- Same lock + condvar idea as C++/Rust, but the **library** is built for this exact use case and has seen years of tuning.

**Why C++ and Rust use “custom” and can be slower under contention**

- **Single mutex:** Every push and pop takes the same lock. With many threads (e.g. 8,8 or asymmetric 4,1 / 1,4), contention rises and throughput drops or becomes variable.
- **C++:** `std::mutex` and `std::condition_variable` are general-purpose; scheduling and lock behaviour can favour (1,1) or (4,4) and hurt (2,2) or (8,8) in a given run. Asymmetric configs cause many blocks/wakeups → lower throughput.
- **Rust:** `std::sync::Mutex` does not spin; threads sleep on contention. **Fairness** spreads the lock across many waiters, which can reduce throughput when many threads compete. Asymmetric (4,1)/(1,4) stress exactly that: one side does all the work, the other side blocks often → many context switches and condvar operations.

**Why asymmetric (4,1) and (1,4) are hard**

- **4 producers, 1 consumer:** Queue fills quickly; producers block on “not full.” Only one consumer drains, so we get a lot of: consumer pops one → notifies one producer → producer pushes → notifies consumer. Many lock acquisitions per item.
- **1 producer, 4 consumers:** Queue is often empty; consumers block on “not empty.” One producer feeds four consumers; again, many wakeups and lock handoffs per item.
- So (4,1) and (1,4) are **not** “more parallelism” — they create a **single bottleneck** and maximise blocking. Java’s optimised queue and runtime handle this relatively well; C++ and especially Rust pay a large cost for the extra threads and condvar traffic.

---

## Interpretation by language

**Java:** Use **ArrayBlockingQueue** (or similar `java.util.concurrent` queues) when you need a bounded blocking queue. In this benchmark it is the fastest and scales best with symmetric and asymmetric (P,C). Memory numbers reflect JVM heap, not just the queue.

**C++:** A **custom** mutex + two condition variables is a standard approach and performs well at (1,1) and (4,4). Throughput is more variable at (2,2) and (8,8). For production, consider lock-free or multi-queue designs if you need higher throughput under heavy contention; this benchmark is a single-queue, high-contention baseline.

**Rust:** The same **custom** Mutex + Condvar pattern is correct but **throughput drops as threads increase** and is very low for (4,1)/(1,4). This reflects single-lock contention and the behaviour of `std::sync::Mutex` (no spinning, fair). For higher throughput under contention, consider channels (`std::sync::mpsc`, or crates like `crossbeam`) or other structures that reduce contention; this benchmark documents the cost of the simple mutex+condvar queue.

---

## Pitfalls and limitations

- **No work per item:** We only push and pop integers. Real pipelines do work in producers/consumers; then the queue is one of several bottlenecks. Results here are **queue and lock** dominated.
- **Memory:** Java reports JVM heap; C++/Rust report process memory. Not directly comparable across languages.
- **Python omitted:** Global interpreter lock (GIL) makes a true multi-threaded producer–consumer queue in CPython unsuitable for this comparison.
- **One queue:** All implementations use a **single** shared queue. Scaling with P and C is limited by that one lock; “more threads” does not imply “faster” in this setup.

---

## Takeaways

| Goal                         | Observation in this benchmark        |
|-----------------------------|--------------------------------------|
| Highest throughput          | **Java** (ArrayBlockingQueue)        |
| Predictable single-thread   | **C++** or **Rust** (1,1)             |
| Minimal process memory      | **Rust**                             |
| Custom / educational queue  | C++ and Rust (mutex + condvar)       |
| Production-ready in Java    | Prefer **ArrayBlockingQueue**        |

- **C++ and Rust** used **custom/native** bounded queues (mutex + two condition variables). **Java** used the **optimized** `ArrayBlockingQueue`. The difference in behaviour (Java faster, scales with threads; Rust slows with more threads and is very slow on asymmetric configs) is consistent with library tuning vs a single hand-rolled lock and with Rust’s fair, non-spinning mutex under heavy contention.
- Asymmetric (4,1) and (1,4) configs show that **one producer or one consumer** becomes the bottleneck and that **blocking and wakeup cost** dominate; they are useful for stressing the queue implementation, not for maximising throughput.
