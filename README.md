# Polyglot Data Structure Benchmark

This repository implements core data structures and concurrency primitives in:

- Python
- Java
- C++
- Rust

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

- Insert 1M elements
- Random access
- Delete workload
- Mixed read/write workload
- Concurrent producer-consumer
- High allocation churn test

## Methodology

Each language implements equivalent logic.
Benchmarks are run in isolation on same machine.
Results stored in `results/` (see [results/raw/](results/raw/README.md)).
Results include runtime overhead differences (JIT warmup, interpreter overhead, GC behavior) and therefore do not measure pure data structure complexity alone.

## Limitations and Experimental Considerations

- Results reflect full runtime overhead (JIT, interpreter, GC) rather than pure data structure complexity.
- Memory measurements represent total process RSS (or heap used), not isolated structure memory.
- Native languages (C++, Rust) were compiled in release mode; Java performance reflects JIT-optimized execution.
- Bounds checking semantics differ across languages.
- Results are hardware- and OS-dependent.
- Each scenario uses one untimed warm-up run and 5 timed runs per scale; CSV schema is consistent across structures.

## Project structure

- **benchmarks/** — [scenarios.md](benchmarks/scenarios.md) (scenario definitions), [run_all.sh](benchmarks/run_all.sh) (orchestration)
- **docs/** — [design.md](docs/design.md), [methodology.md](docs/methodology.md)
- **results/** — CSV output in [results/raw/](results/raw/README.md), [analysis.md](results/analysis.md) for summaries; preserved HashMap study in [results/hashmap/](results/hashmap/README.md) (raw CSVs in `results/hashmap/raw/`, [hashmap_tests_findings.md](results/hashmap/hashmap_tests_findings.md)); structure-specific studies (e.g. dynamic array) in [results/dynamic_array/](results/dynamic_array/) with [dynamic_array_findings.md](results/dynamic_array/dynamic_array_findings.md); use `make save-hashmap-study`, `make save-structure-study STRUCTURE=dynamic_array`, or `make plot-structure-study STRUCTURE=...` to update study data/plots
- **python/**, **java/**, **cpp/**, **rust/** — per-language sources, benchmarks, and tests

## Quick start

```bash
make test      # run tests for all languages
make bench     # run all benchmarks (CSV in results/raw/)
make plots     # generate log-scale graphs from results/raw (requires matplotlib)
make save-hashmap-study   # copy results/raw/*_hashmap*.csv to results/hashmap/raw/ and plot there
make plot-hashmap-study   # regenerate plots from results/hashmap/raw/ into results/hashmap/plots/
make docker-bench   # run benchmarks in Docker (optional)
```

## Why This Exists

To deeply understand runtime, memory, and concurrency tradeoffs
across modern production languages.