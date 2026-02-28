# Analysis

Analysis compares outputs under `results/raw/` across languages.

**CSV outputs (hashmap benchmarks):**

- `*_hashmap.csv`: N, insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms, memory_mb (main scenario, random keys).
- `*_hashmap_low_entropy.csv`: Low-entropy / near-collision (fixed small capacity 64) — same columns except memory.
- `*_hashmap_loadfactor.csv`: Load factor sensitivity (N=100k, capacity varied for 0.25, 0.5, 0.75, 1.0).

**Plots:** Run `make plots` (or `python3 benchmarks/plot_results.py`) to generate log-scale graphs in `results/plots/`: insert_log.png, get_log.png, memory_log.png, low_entropy_insert_log.png, loadfactor_insert.png. For the preserved HashMap study, run `make plot-hashmap-study` (reads `results/hashmap/raw/`, writes `results/hashmap/plots/`).

See also [hashmap/hashmap_tests_findings.md](hashmap/hashmap_tests_findings.md) for interpretation and takeaways; preserved hashmap CSVs for that analysis live in `results/hashmap/raw/` (update with `make save-hashmap-study` after a run).
