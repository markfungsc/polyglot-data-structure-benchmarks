# LRU cache study (preserved snapshot)

This folder holds a **preserved LRU cache benchmark study**: CSVs and findings that are not overwritten by normal `make bench` runs (which write to `results/raw/`).

- **raw/** — `*_lru_cache.csv` for each language (Python, Java, C++, Rust) and **rust_native_lru_cache.csv** for the Rust `lru` crate. Two Rust variants: self-implemented (Rc/RefCell) and native library.
- **plots/** — Log-scale plots generated from `raw/` (put_miss, put_hit, get_hit, get_miss, eviction, memory). Run `make plot-structure-study STRUCTURE=lru_cache` from repo root.
- **lru_cache_findings.md** — Methodology, Big O, implementation notes (Rust custom vs native; C++ standard pattern; Java LinkedHashMap; Python OrderedDict), and interpretation.

**To refresh from a new run:** From the repo root, run `make save-structure-study STRUCTURE=lru_cache` after `make bench` to copy `results/raw/*_lru_cache.csv` and `rust_native_lru_cache.csv` here and regenerate plots.

**To regenerate plots only:** Run `make plot-structure-study STRUCTURE=lru_cache` from the repo root.
