#!/usr/bin/env python3
"""
Plot workload benchmark CSVs: one graph per measurement.
Supports dynamic_array (sum, vwap, ma, filter, memory) and hashmap (update, lookup, scan, range, memory).
Finds *_workload_*.csv in raw_dir; writes *_<op>_log.png and *_memory_log.png to plots_dir.
Usage: python3 plot_workload.py [--raw-dir DIR] [--plots-dir DIR]
"""
import argparse
import csv
import os
import sys

# Workload schemas: (structs tuple, struct_labels dict, ops tuple)
WORKLOAD_CONFIGS = {
    "dynamic_array": {
        "structs": ("vec", "vecdeque", "linkedlist", "columnar"),
        "labels": {"vec": "Vec", "vecdeque": "VecDeque", "linkedlist": "LinkedList", "columnar": "Columnar"},
        "ops": ("sum", "vwap", "ma", "filter"),
    },
    "hashmap": {
        "structs": ("hashmap", "btreemap", "vecmap"),
        "labels": {"hashmap": "HashMap", "btreemap": "BTreeMap", "vecmap": "VecMap"},
        "ops": ("update", "lookup", "scan", "range"),
    },
    "heap": {
        "structs": ("binary_heap", "b_tree_set", "sorted_vec", "vec"),
        "labels": {"binary_heap": "BinaryHeap", "b_tree_set": "BTreeSet", "sorted_vec": "SortedVec", "vec": "Vec"},
        "ops": ("push", "pop", "peek", "topk"),
    },
    "lru": {
        "structs": ("hashmap", "naive_lru", "lru", "linked"),
        "labels": {"hashmap": "HashMap", "naive_lru": "NaiveLRU", "lru": "LruCache", "linked": "LinkedHashMap"},
        "ops": ("put", "get", "mostly_get", "balanced"),
    },
}


def _detect_workload_config(fieldnames):
    """Return (structs, labels, ops) for this CSV based on column names."""
    if "lru_put_mean_ms" in fieldnames:
        c = WORKLOAD_CONFIGS["lru"]
        return c["structs"], c["labels"], c["ops"]
    if "binary_heap_push_mean_ms" in fieldnames:
        c = WORKLOAD_CONFIGS["heap"]
        return c["structs"], c["labels"], c["ops"]
    if "hashmap_update_mean_ms" in fieldnames:
        c = WORKLOAD_CONFIGS["hashmap"]
        return c["structs"], c["labels"], c["ops"]
    c = WORKLOAD_CONFIGS["dynamic_array"]
    return c["structs"], c["labels"], c["ops"]


def _default_raw_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "workloads", "raw")


def _default_plots_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "workloads", "plots")


def _group_rows_by(rows, key_col="scenario"):
    """Group rows by key_col. Returns {key: [rows]}. If key_col not in rows, returns {'': rows}."""
    if not rows or key_col not in rows[0]:
        return {"": rows}
    groups = {}
    for r in rows:
        k = r.get(key_col, "")
        groups.setdefault(k, []).append(r)
    # Sort each group by N for consistent x-axis order
    for k in groups:
        groups[k] = sorted(groups[k], key=lambda r: int(r.get("N", 0)))
    return groups


def load_workload_csv(path):
    """Load a workload CSV; return list of row dicts."""
    rows = []
    with open(path, newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            rows.append(row)
    return rows


def plot_one_metric(rows, mean_cols, ylabel, title, out_path, log_y=True):
    """Plot N vs mean for each structure (mean_cols: list of (column_name, label))."""
    import matplotlib.pyplot as plt

    fig, ax = plt.subplots()
    N = [int(r["N"]) for r in rows]
    for col, label in mean_cols:
        if col not in rows[0]:
            continue
        vals = [float(r.get(col, 0) or 0) for r in rows]
        if log_y:
            ax.loglog(N, vals, "o-", label=label, markersize=6)
        else:
            ax.semilogx(N, vals, "o-", label=label, markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel(ylabel)
    ax.set_title(title)
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(out_path, dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", out_path)


def plot_memory(rows, mem_cols, out_path, title=None):
    """Plot N vs memory (MB) for each structure (mem_cols: list of (struct_key, label); column is {struct_key}_memory_mb)."""
    import matplotlib.pyplot as plt

    fig, ax = plt.subplots()
    N = [int(r["N"]) for r in rows]
    for struct_key, label in mem_cols:
        col = f"{struct_key}_memory_mb"
        if col not in rows[0]:
            continue
        vals = [float(r.get(col, 0) or 0) for r in rows]
        ax.semilogx(N, vals, "o-", label=label, markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel("Memory (MB)")
    ax.set_title(title if title is not None else "Memory by structure (log N)")
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(out_path, dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", out_path)


def main():
    parser = argparse.ArgumentParser(description="Plot workload CSVs (one graph per measurement).")
    parser.add_argument("--raw-dir", help="Directory containing *_workload_*.csv")
    parser.add_argument("--plots-dir", help="Directory to write PNGs")
    args = parser.parse_args()

    raw_dir = args.raw_dir or os.environ.get("RESULTS_RAW_DIR") or _default_raw_dir()
    plots_dir = args.plots_dir or os.environ.get("RESULTS_PLOTS_DIR") or _default_plots_dir()
    raw_dir = os.path.abspath(raw_dir)
    plots_dir = os.path.abspath(plots_dir)

    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
    except ImportError:
        print("matplotlib not found. Install with: pip install matplotlib", file=sys.stderr)
        sys.exit(1)

    os.makedirs(plots_dir, exist_ok=True)

    # Discover *_workload_*.csv
    workload_files = []
    if os.path.isdir(raw_dir):
        for name in sorted(os.listdir(raw_dir)):
            if "_workload_" in name and name.endswith(".csv"):
                workload_files.append((name, os.path.join(raw_dir, name)))

    if not workload_files:
        print(f"No *_workload_*.csv files found in {raw_dir}", file=sys.stderr)
        sys.exit(1)

    for basename, path in workload_files:
        # Prefix: rust_workload_dynamic_array.csv -> workload_dynamic_array
        stem = basename.replace(".csv", "")
        prefix = stem.replace("rust_workload_", "workload_") if stem.startswith("rust_workload_") else stem

        rows = load_workload_csv(path)
        if not rows:
            continue

        fieldnames = list(rows[0].keys())
        structs, struct_labels, ops = _detect_workload_config(fieldnames)
        group_col = "scenario" if "scenario" in fieldnames else None
        groups = _group_rows_by(rows, group_col) if group_col else {"": rows}

        for group_key, subset_rows in groups.items():
            plot_prefix = f"{prefix}_{group_key}" if group_key else prefix
            title_suffix = f" ({group_key}, log scale)" if group_key else " (log scale)"
            memory_title_suffix = f" ({group_key}, log N)" if group_key else " (log N)"

            for op in ops:
                # Columns are {struct}_{op}_mean_ms
                required = [f"{s}_{op}_mean_ms" for s in structs]
                if not any(c in fieldnames for c in required):
                    continue
                cols_for_op = [(f"{s}_{op}_mean_ms", struct_labels[s]) for s in structs if f"{s}_{op}_mean_ms" in fieldnames]
                if not cols_for_op:
                    continue
                ylabel = f"{op.replace('_', ' ').title()} time (ms)"
                title = f"Workload {op}{title_suffix}"
                out_path = os.path.join(plots_dir, f"{plot_prefix}_{op}_log.png")
                plot_one_metric(subset_rows, cols_for_op, ylabel, title, out_path, log_y=True)

            # Memory: {struct}_memory_mb
            mem_required = [f"{s}_memory_mb" for s in structs]
            if any(c in fieldnames for c in mem_required):
                mem_list = [(s, struct_labels[s]) for s in structs if f"{s}_memory_mb" in fieldnames]
                if mem_list:
                    out_path = os.path.join(plots_dir, f"{plot_prefix}_memory_log.png")
                    title = f"Memory by structure{memory_title_suffix}"
                    plot_memory(subset_rows, mem_list, out_path, title=title)

    print("Plots saved to", plots_dir)


if __name__ == "__main__":
    main()
