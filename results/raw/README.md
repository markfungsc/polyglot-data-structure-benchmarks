# Raw benchmark results (CSV only)

Benchmark programs write **CSV results** here (via `RESULTS_DIR` when running `benchmarks/run_all.sh`). New runs may overwrite files here. For a preserved HashMap study snapshot, data lives in `results/hashmap/raw/`; use `make save-hashmap-study` from the repo root to copy current `*_hashmap*.csv` files into that folder and regenerate study plots. For a preserved dynamic array study, use `make save-structure-study STRUCTURE=dynamic_array`; data and plots go to `results/dynamic_array/`. Commit as needed for analysis.
