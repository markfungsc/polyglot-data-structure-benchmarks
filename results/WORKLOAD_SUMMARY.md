# Workload benchmarks — summary

This document gives a **short summary** of each workload benchmark: what structures are compared, who is fastest (and when), and where to find full details. Workload tests use **Rust only** to isolate structure and access-pattern effects; they run multi-op sequences (e.g. sum, VWAP, filter) or key-space scenarios (e.g. full, eviction) rather than single insert/get. Full methodology and numbers are in the per-workload findings files linked below.

---

## Workload: Dynamic array (linear structures)

**Full findings:** [workloads/dynamic_array/workload_dynamic_array_findings.md](workloads/dynamic_array/workload_dynamic_array_findings.md)

- **Compares:** Vec, VecDeque, LinkedList, Columnar (structure-of-arrays) on tick-style data.
- **Ops:** Sum, VWAP, moving average, filter, memory.
- **At 1M:** **Columnar** fastest on filter; **Vec / Columnar** best on VWAP/MA; **LinkedList** slowest on all ops and highest memory (non-contiguous cost).
- **Takeaway:** Contiguous layout wins for scan and windowed analytics; columnar wins when the op is column-oriented (e.g. filter on one column).

---

## Workload: Hashmap (key–aggregate structures)

**Full findings:** [workloads/hashmap/workload_hashmap_findings.md](workloads/hashmap/workload_hashmap_findings.md)

- **Compares:** HashMap (FxHasher), BTreeMap, VecMap (`Vec<Option<Agg>>` by symbol id) on tick-style per-symbol aggregation.
- **Ops:** Update, lookup, scan, range, memory.
- **At 1M:** **VecMap** fastest on update, lookup, and scan when keys are dense indices; **HashMap** strong for sparse keys; **BTreeMap** for ordered/range needs.
- **Takeaway:** VecMap dominates when keys are dense symbol ids; HashMap (FxHasher) for generic keys; BTreeMap when order or range matters.

---

## Workload: Heap (priority-queue–style structures)

**Full findings:** [workloads/heap/workload_heap_findings.md](workloads/heap/workload_heap_findings.md)

- **Compares:** BinaryHeap, BTreeSet, SortedVec (insert-in-order), Vec + Sort.
- **Ops:** Push, pop, peek (k× at max), topk, memory.
- **At 1M:** **Vec + Sort** often fastest on wall-clock (push, pop, topk) despite same or worse Big-O; **BinaryHeap** best when incremental updates matter; **SortedVec** O(N²) push — avoid at scale.
- **Takeaway:** Batch build then drain → Vec + Sort; streaming or incremental priority queue → BinaryHeap.

---

## Workload: LRU (cache-like structures)

**Full findings:** [workloads/lru/workload_lru_findings.md](workloads/lru/workload_lru_findings.md)

- **Compares:** HashMap (no eviction), NaiveLRU (Vec + index map), LruCache (crate), LinkedHashMap (hashlink) across three key-space scenarios: **full** (capacity = N), **high locality** (capacity = N/10, key_space = N/10), **eviction** (capacity = N/10, key_space = N).
- **Ops:** Put, get (get-only timed), mostly_get (90% get / 10% put), balanced (50/50), memory.
- **Winners depend on N and scenario:** At large N, **LinkedHashMap** leads on get and mostly_get in most scenarios; **HashMap** on full put/balanced; **LinkedHashMap** or **LruCache** on eviction. **NaiveLRU** is often slowest on get/balanced; **LruCache** has an outlier on mostly_get at 1M full (~9× slower than LinkedHashMap).
- **Takeaway:** For LRU with eviction and get-heavy traffic, prefer **LinkedHashMap** or **LruCache**; HashMap when no capacity limit. See findings for the “who wins” table by scenario and op.

---

# Links to full findings

| Workload        | Findings file |
|-----------------|----------------|
| Dynamic array   | [workloads/dynamic_array/workload_dynamic_array_findings.md](workloads/dynamic_array/workload_dynamic_array_findings.md) |
| Hashmap         | [workloads/hashmap/workload_hashmap_findings.md](workloads/hashmap/workload_hashmap_findings.md) |
| Heap            | [workloads/heap/workload_heap_findings.md](workloads/heap/workload_heap_findings.md) |
| LRU             | [workloads/lru/workload_lru_findings.md](workloads/lru/workload_lru_findings.md) |

Each workload folder under `results/workloads/<name>/` has `raw/` (CSVs), `plots/` (PNGs), and the findings file. Run workload benchmarks with `./benchmarks/run_all.sh rust workload_<name>`; plot with `python3 benchmarks/plot_workload.py --raw-dir ... --plots-dir ...`.
