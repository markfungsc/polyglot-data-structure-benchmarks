# Dynamic array study (preserved snapshot)

This folder holds a **preserved dynamic array benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_dynamic_array.csv` for each language (Python, Java, C++, Rust).
- **plots/** — Log-scale plots generated from `raw/` (run `make plot-structure-study STRUCTURE=dynamic_array` from repo root).
- **dynamic_array_findings.md** — Methodology, numbers, and interpretation for this study.

**To refresh from a new run:** From the repo root, run `make save-structure-study STRUCTURE=dynamic_array` after `make bench` to copy `results/raw/*_dynamic_array.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-structure-study STRUCTURE=dynamic_array` from the repo root.
