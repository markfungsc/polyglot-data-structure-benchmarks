# Linked list study (preserved snapshot)

This folder holds a **preserved linked list benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_linked_list.csv` for each language (Python, Java, C++, Rust).
- **plots/** — Log-scale plots generated from `raw/` (run `make plot-structure-study STRUCTURE=linked_list` from repo root).
- **linked_list_findings.md** — Methodology, Big O notes, numbers, pitfalls (e.g. get(i) in a loop, stack overflow), and interpretation for this study.

**To refresh from a new run:** From the repo root, run `make save-structure-study STRUCTURE=linked_list` after `make bench` to copy `results/raw/*_linked_list.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-structure-study STRUCTURE=linked_list` from the repo root.
