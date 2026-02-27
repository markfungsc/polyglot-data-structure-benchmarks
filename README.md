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

## Project structure

- **benchmarks/** — [scenarios.md](benchmarks/scenarios.md) (scenario definitions), [run_all.sh](benchmarks/run_all.sh) (orchestration)
- **docs/** — [design.md](docs/design.md), [methodology.md](docs/methodology.md), [findings.md](docs/findings.md)
- **results/** — raw output in `results/raw/`, [analysis.md](results/analysis.md) for summaries
- **python/**, **java/**, **cpp/**, **rust/** — per-language sources, benchmarks, and tests

## Quick start

```bash
make test      # run tests for all languages
make bench     # run all benchmarks (output in results/raw/)
make docker-bench   # run benchmarks in Docker (optional)
```

## Why This Exists

To deeply understand runtime, memory, and concurrency tradeoffs
across modern production languages.