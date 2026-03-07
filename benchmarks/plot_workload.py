#!/usr/bin/env python3
"""
Plot workload benchmark CSVs: one graph per measurement (sum, vwap, ma, filter, memory).
Finds *_workload_*.csv in raw_dir; writes *_sum_log.png, *_vwap_log.png, etc. to plots_dir.
Usage: python3 plot_workload.py [--raw-dir DIR] [--plots-dir DIR]
"""
import argparse
import csv
import os
import sys

STRUCTS = ("vec", "vecdeque", "linkedlist", "columnar")
STRUCT_LABELS = {"vec": "Vec", "vecdeque": "VecDeque", "linkedlist": "LinkedList", "columnar": "Columnar"}
OPS = ("sum", "vwap", "ma", "filter")


def _default_raw_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "workloads", "raw")


def _default_plots_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "workloads", "plots")


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


def plot_memory(rows, mem_cols, out_path):
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
    ax.set_title("Memory by structure (log N)")
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

        for op in OPS:
            # Columns are {struct}_sum_mean_ms, etc.
            required = [f"{s}_{op}_mean_ms" for s in STRUCTS]
            if not any(c in fieldnames for c in required):
                continue
            # Build column list for this op (some structs might be missing)
            cols_for_op = [(f"{s}_{op}_mean_ms", STRUCT_LABELS[s]) for s in STRUCTS if f"{s}_{op}_mean_ms" in fieldnames]
            if not cols_for_op:
                continue
            ylabel = f"{op.replace('_', ' ').title()} time (ms)"
            title = f"Workload {op} (log scale)"
            out_path = os.path.join(plots_dir, f"{prefix}_{op}_log.png")
            plot_one_metric(rows, cols_for_op, ylabel, title, out_path, log_y=True)

        # Memory: vec_memory_mb, etc.
        mem_required = [f"{s}_memory_mb" for s in STRUCTS]
        if any(c in fieldnames for c in mem_required):
            mem_list = [(s, STRUCT_LABELS[s]) for s in STRUCTS if f"{s}_memory_mb" in fieldnames]
            if mem_list:
                out_path = os.path.join(plots_dir, f"{prefix}_memory_log.png")
                plot_memory(rows, mem_list, out_path)

    print("Plots saved to", plots_dir)


if __name__ == "__main__":
    main()
