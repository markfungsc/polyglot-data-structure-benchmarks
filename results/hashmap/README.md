# HashMap study (preserved snapshot)

This folder holds a **preserved HashMap benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_hashmap.csv`, `*_hashmap_low_entropy.csv`, `*_hashmap_loadfactor.csv` for each language (Python, Java, C++, Rust).
- **plots/** — Log-scale and load-factor plots generated from `raw/` (run `make plot-hashmap-study` from repo root).
- **hashmap_tests_findings.md** — Methodology, numbers, and interpretation for this study.

**To refresh from a new run:** From the repo root, run `make save-hashmap-study` after `make bench` to copy `results/raw/*_hashmap*.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-hashmap-study` from the repo root.
