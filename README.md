# Polyglot Data Structure Benchmark

This repository implements core data structures and concurrency primitives in:

- Python
- Java
- C++
- Rust

These languages were chosen to compare/ study interpreted, JIT-compiled, and AOT-compiled runtimes and to contrast garbage-collected with manual memory management.

## Goals

1. Compare runtime performance
2. Compare memory usage
3. Compare concurrency behavior
4. Understand tradeoffs between GC and manual memory management
5. Demonstrate cross-language engineering competence

## Language Implementation Notes

- **C++**: Compiled to native code; manual memory management (with optional RAII). Benchmarks run in release mode with optimizations enabled.
- **Rust**: Compiled to native code; memory safety guaranteed by the borrow checker without a garbage collector. Benchmarks also run with optimizations (`--release`).
- **Java**: Runs on the JVM (Java Virtual Machine), which is largely implemented in C++. Uses garbage collection, JIT compilation, and runtime bounds checks.
- **Python**: Interpreted via CPython (mostly C), with automatic reference counting and optional garbage collection. Slower raw execution than compiled languages.

## Data Structures Implemented

- Dynamic Array
- Linked List
- HashMap
- Binary Heap
- LRU Cache

## Benchmarked Scenarios

Implemented benchmarks (dynamic array, linked list, heap, hashmap) use:

- **Scales:** N = 1,000; 10,000; 100,000; 1,000,000 (1k–1M).
- **Insert phase:** Time to build the structure (push/put) for N elements.
- **Get phase:** Time for N indexed or key-based accesses (linked list: one full traverse; heap: N pops of the minimum, not random index).
- **Runs:** 5 timed runs per scale (mean ± std); one warm-up run per scale.
- **Linked list:** Insert, get (traverse), delete (one delete per run); see [scenarios.md](benchmarks/scenarios.md).
- **Heap:** Insert, get (pop min N times); Python uses `heapq`; see [scenarios.md](benchmarks/scenarios.md).
- **HashMap only:** Main scenario above; plus low-entropy (same N scales, fixed small capacity) and load-factor sensitivity (e.g. N=100k, load factors 0.25–1.0).

Planned or optional (see [scenarios.md](benchmarks/scenarios.md)): mixed read/write, concurrent producer-consumer, high allocation churn.

## Results summaries (readable overviews)

- **Structure benchmarks (cross-language):** [results/DATA_STRUCTURES_SUMMARY.md](results/DATA_STRUCTURES_SUMMARY.md) — short summary per structure (complexity, who's fastest, memory, language patterns); links to full findings under `results/<structure>/`.
- **Workload benchmarks (Rust, multi-op / key-space):** [results/WORKLOAD_SUMMARY.md](results/WORKLOAD_SUMMARY.md) — short summary per workload (dynamic array, hashmap, heap, LRU); links to full findings under `results/workloads/<name>/`. Scenario definitions: [benchmarks/workload_scenarios.md](benchmarks/workload_scenarios.md).

## Tests and benchmarks summary

This project implements the same data structures and benchmarks in Python, Java, C++, and Rust. Run `make test` for unit tests and `make bench` for benchmarks; see Quick start below.

**Finished structure benchmarks**

- **Dynamic array** — Insert/get and memory at 1k–1M elements across Python, Java, C++, and Rust. C++ and Rust fastest; Rust lowest memory. Results and plots: [results/dynamic_array/](results/dynamic_array/).
- **Linked list** — Insert, get (traverse), delete and memory at 1k–1M elements. C++ and Rust fastest; Java shows non-linear scaling and high variance. Results and findings: [results/linked_list/](results/linked_list/) ([linked_list_findings.md](results/linked_list/linked_list_findings.md)).
- **Heap** — Insert and get (pop min N times) at 1k–1M elements. C++ and Rust fastest; Rust lowest memory; Python uses `heapq`. Results and findings: [results/heap/](results/heap/) ([heap_findings.md](results/heap/heap_findings.md)).
- **HashMap** — Main scenario (scaled N) plus low-entropy and load-factor scenarios. C++ fastest insert; Rust fastest get and lowest memory. Results and plots: [results/hashmap/](results/hashmap/).

**Workload benchmarks (Rust)** — multi-op or key-space scenarios, one language to isolate structure choice: [workload_scenarios.md](benchmarks/workload_scenarios.md). Summaries: [results/WORKLOAD_SUMMARY.md](results/WORKLOAD_SUMMARY.md). Implemented: workload_dynamic_array, workload_hashmap, workload_heap, workload_lru (see `./benchmarks/run_all.sh rust workload_<name>`).

**Benchmarks implemented**

| Structure      | Python | Java | C++ | Rust |
|----------------|--------|------|-----|------|
| Dynamic Array  | ✓      | ✓    | ✓   | ✓    |
| Linked List    | ✓      | ✓    | ✓   | ✓    |
| HashMap        | ✓      | ✓    | ✓   | ✓    |
| Heap           | ✓      | ✓    | ✓   | ✓    |
| LRU Cache      | ✓      | ✓    | ✓   | ✓    |
| Concurrency    | —      | —    | —   | —    |

## Methodology

Each language implements equivalent logic.
Benchmarks are run in isolation on same machine.
Results stored in `results/` (see [results/raw/](results/raw/README.md)).
Results include runtime overhead differences (JIT warmup, interpreter overhead, GC behavior) and therefore do not measure pure data structure complexity alone.

### Test and benchmark results

- **Benchmark results:** `make bench` writes CSV files to [results/raw/](results/raw/). Each run can overwrite; use `make save-hashmap-study` or `make save-structure-study STRUCTURE=...` to copy into preserved study folders.
- **Preserved study results:** [results/hashmap/](results/hashmap/) (HashMap), [results/dynamic_array/](results/dynamic_array/) (dynamic array), [results/linked_list/](results/linked_list/) (linked list), [results/heap/](results/heap/) (heap). Each has `raw/` (CSVs) and `plots/` (PNGs); findings in `*_findings.md`.
- **Unit tests:** Run with `make test`; no results files are committed (pytest/mvn/cargo output to console).

## Limitations and Experimental Considerations

- Results reflect full runtime overhead (JIT, interpreter, GC) rather than pure data structure complexity.
- Memory measurements represent total process RSS (or heap used), not isolated structure memory.
- Native languages (C++, Rust) were compiled in release mode; Java performance reflects JIT-optimized execution.
- Bounds checking semantics differ across languages.
- Results are hardware- and OS-dependent.
- Each scenario uses one untimed warm-up run and 5 timed runs per scale; CSV schema is consistent across structures.

## Project structure

- **benchmarks/** — [scenarios.md](benchmarks/scenarios.md) (structure scenario definitions), [workload_scenarios.md](benchmarks/workload_scenarios.md) (workload test definitions), [run_all.sh](benchmarks/run_all.sh) (orchestration)
- **docs/** — [design.md](docs/design.md), [methodology.md](docs/methodology.md)
- **results/** — CSV output in [results/raw/](results/raw/README.md). **Summaries:** [DATA_STRUCTURES_SUMMARY.md](results/DATA_STRUCTURES_SUMMARY.md) (structure benchmarks), [WORKLOAD_SUMMARY.md](results/WORKLOAD_SUMMARY.md) (workload benchmarks). Preserved studies: [results/hashmap/](results/hashmap/), [results/dynamic_array/](results/dynamic_array/), [results/linked_list/](results/linked_list/), [results/heap/](results/heap/), [results/lru_cache/](results/lru_cache/), each with `raw/`, `plots/`, `*_findings.md`; workload results under [results/workloads/](results/workloads/) (e.g. `workloads/lru/`). Use `make save-hashmap-study`, `make save-structure-study STRUCTURE=...`, or `make plot-structure-study STRUCTURE=...` for structure studies.
- **python/**, **java/**, **cpp/**, **rust/** — per-language sources, benchmarks, and tests

## Quick start

```bash
make test      # run tests for all languages
make bench     # run all benchmarks (CSV in results/raw/)
make plots     # log-scale plots from results/raw (requires matplotlib)
```

**Structure studies** (copy raw CSVs to study folder and plot): `STRUCTURE` = `dynamic_array` | `linked_list` | `heap` | `lru_cache`

```bash
make save-structure-study STRUCTURE=dynamic_array   # copy + plot dynamic array
make save-structure-study STRUCTURE=linked_list     # copy + plot linked list
make save-structure-study STRUCTURE=heap            # copy + plot heap
make save-structure-study STRUCTURE=lru_cache       # copy + plot LRU cache (incl. rust_native)
make plot-structure-study STRUCTURE=<name>         # regenerate plots only (no copy)
make save-hashmap-study   # hashmap: copy results/raw/*_hashmap*.csv to results/hashmap/raw/ and plot
make plot-hashmap-study   # hashmap: regenerate plots from results/hashmap/raw/
```

**Workload studies** (Rust workload benchmarks): `TEST` = `dynamic_array` | `hashmap` | `heap` | `lru`

```bash
make save-workload-study TEST=lru        # copy results/raw/*_workload_lru*.csv to results/workloads/lru/raw/ and plot
make plot-workload-study TEST=lru        # regenerate workload plots only (no copy)
# Same for TEST=dynamic_array, hashmap, heap
```

**Other**

```bash
make bench-hashmap-study  # run all benchmarks into results/hashmap/raw/, then plot hashmap
make docker-bench         # run benchmarks in Docker (optional)
```

## Why This Exists

To deeply understand runtime, memory, and concurrency tradeoffs
across modern production languages.