# Heap study (preserved snapshot)

This folder holds a **preserved heap benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_heap.csv` for each language (Python, Java, C++, Rust).
- **plots/** — Log-scale plots generated from `raw/` (run `make plot-structure-study STRUCTURE=heap` from repo root).
- **heap_findings.md** — Methodology, Big O, move vs swap, Python heapq, and interpretation. Note: “get” in the benchmark = pop min N times (drain heap), not random index access.

**To refresh from a new run:** From the repo root, run `make save-structure-study STRUCTURE=heap` after `make bench` to copy `results/raw/*_heap.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-structure-study STRUCTURE=heap` from the repo root.
