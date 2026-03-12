# Concurrency study (preserved snapshot)

This folder holds a **preserved concurrency (producer–consumer) benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_concurrency.csv` for each language (C++, Java, Rust). Python omitted (GIL).
- **plots/** — Throughput, elapsed, and memory by (P, C) config (run `make plot-concurrency-study` from repo root).
- **concurrency_findings.md** — Methodology, implementation notes (C++/Rust custom mutex+condvar vs Java ArrayBlockingQueue), behaviour summary, and possible explanations.

**To refresh from a new run:** From the repo root, run `make save-concurrency-study` after running the concurrency benchmarks to copy `results/raw/*_concurrency.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-concurrency-study` from the repo root.
