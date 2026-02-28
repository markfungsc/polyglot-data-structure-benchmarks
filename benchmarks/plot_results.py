#!/usr/bin/env python3
"""
Read hashmap CSV results from results/raw/ and plot log-scale graphs.
Saves figures to results/plots/. Supports custom dirs via RESULTS_DIR or
RESULTS_RAW_DIR (CSV input), RESULTS_PLOTS_DIR (PNG output), or --raw-dir/--plots-dir.
"""
import argparse
import csv
import os
import sys

def _default_results_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "raw")

def _default_plots_dir():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, "..", "results", "plots")

def lang_display(lang):
    return {"cpp": "C++", "python": "Python", "java": "Java", "rust": "Rust"}.get(lang, lang)

def load_main_csv(path):
    """Load main hashmap CSV; return (lang, rows) where rows are dicts with N, insert_mean_ms, get_mean_ms, etc."""
    lang = os.path.basename(path).replace("_hashmap.csv", "").replace(".csv", "")
    rows = []
    with open(path, newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            row["_lang"] = lang
            # Normalize column names
            if "insert_mean_ms" not in row and "insert_ms" in row:
                row["insert_mean_ms"] = row["insert_ms"]
                row["insert_std_ms"] = "0"
            if "get_mean_ms" not in row and "get_ms" in row:
                row["get_mean_ms"] = row["get_ms"]
                row["get_std_ms"] = "0"
            if "memory_mb" not in row:
                row["memory_mb"] = "0"
            rows.append(row)
    return lang, rows

def load_low_entropy_csv(path):
    lang = os.path.basename(path).replace("_hashmap_low_entropy.csv", "").replace(".csv", "")
    rows = []
    with open(path, newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            row["_lang"] = lang
            if "insert_mean_ms" not in row and "insert_ms" in row:
                row["insert_mean_ms"], row["insert_std_ms"] = row["insert_ms"], "0"
            if "get_mean_ms" not in row and "get_ms" in row:
                row["get_mean_ms"], row["get_std_ms"] = row["get_ms"], "0"
            rows.append(row)
    return lang, rows

def load_loadfactor_csv(path):
    lang = os.path.basename(path).replace("_hashmap_loadfactor.csv", "").replace(".csv", "")
    rows = []
    with open(path, newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            row["_lang"] = lang
            if "insert_mean_ms" not in row and "insert_ms" in row:
                row["insert_mean_ms"], row["insert_std_ms"] = row["insert_ms"], "0"
            if "get_mean_ms" not in row and "get_ms" in row:
                row["get_mean_ms"], row["get_std_ms"] = row["get_ms"], "0"
            rows.append(row)
    return lang, rows

def main():
    parser = argparse.ArgumentParser(description="Plot hashmap benchmark CSVs to PNGs.")
    parser.add_argument("--raw-dir", help="Directory containing *_hashmap*.csv (overrides env)")
    parser.add_argument("--plots-dir", help="Directory to write PNGs (overrides env)")
    args = parser.parse_args()

    raw_dir = args.raw_dir or os.environ.get("RESULTS_RAW_DIR") or os.environ.get("RESULTS_DIR") or _default_results_dir()
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

    # Collect main CSVs
    main_data = {}
    for name in ["python_hashmap.csv", "java_hashmap.csv", "cpp_hashmap.csv", "rust_hashmap.csv"]:
        path = os.path.join(raw_dir, name)
        if not os.path.isfile(path):
            continue
        lang, rows = load_main_csv(path)
        main_data[lang] = rows

    if not main_data:
        print("No *_hashmap.csv files found in", raw_dir, file=sys.stderr)
        sys.exit(1)

    # Plot 1: N vs insert time (log-log)
    fig, ax = plt.subplots()
    for lang, rows in main_data.items():
        N = [int(r["N"]) for r in rows]
        insert = [float(r.get("insert_mean_ms", r.get("insert_ms", 0))) for r in rows]
        ax.loglog(N, insert, "o-", label=lang_display(lang), markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel("Insert time (ms)")
    ax.set_title("HashMap insert (log scale)")
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(os.path.join(plots_dir, "insert_log.png"), dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", os.path.join(plots_dir, "insert_log.png"))

    # Plot 2: N vs get time (log-log)
    fig, ax = plt.subplots()
    for lang, rows in main_data.items():
        N = [int(r["N"]) for r in rows]
        get_t = [float(r.get("get_mean_ms", r.get("get_ms", 0))) for r in rows]
        ax.loglog(N, get_t, "o-", label=lang_display(lang), markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel("Get time (ms)")
    ax.set_title("HashMap get (log scale)")
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(os.path.join(plots_dir, "get_log.png"), dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", os.path.join(plots_dir, "get_log.png"))

    # Plot 3: N vs memory (if any language has memory_mb > 0)
    has_memory = any(
        float(r.get("memory_mb", 0) or 0) > 0
        for rows in main_data.values()
        for r in rows
    )
    if has_memory:
        fig, ax = plt.subplots()
        for lang, rows in main_data.items():
            N = [int(r["N"]) for r in rows]
            mem = [float(r.get("memory_mb", 0) or 0) for r in rows]
            if any(m > 0 for m in mem):
                ax.semilogx(N, mem, "o-", label=lang_display(lang), markersize=6)
        ax.set_xlabel("N (number of elements)")
        ax.set_ylabel("Memory (MB)")
        ax.set_title("HashMap memory (log N)")
        ax.legend()
        ax.grid(True, which="both", alpha=0.3)
        fig.savefig(os.path.join(plots_dir, "memory_log.png"), dpi=120, bbox_inches="tight")
        plt.close(fig)
        print("Wrote", os.path.join(plots_dir, "memory_log.png"))

    # Low-entropy / near-collision: N vs insert time
    low_entropy_data = {}
    for name in ["python_hashmap_low_entropy.csv", "java_hashmap_low_entropy.csv", "cpp_hashmap_low_entropy.csv", "rust_hashmap_low_entropy.csv"]:
        path = os.path.join(raw_dir, name)
        if os.path.isfile(path):
            lang, rows = load_low_entropy_csv(path)
            low_entropy_data[lang] = rows
    if low_entropy_data:
        fig, ax = plt.subplots()
        for lang, rows in low_entropy_data.items():
            N = [int(r["N"]) for r in rows]
            insert = [float(r.get("insert_mean_ms", r.get("insert_ms", 0))) for r in rows]
            ax.loglog(N, insert, "o-", label=lang_display(lang), markersize=6)
        ax.set_xlabel("N (number of elements)")
        ax.set_ylabel("Insert time (ms)")
        ax.set_title("HashMap low-entropy / near-collision (capacity=64, log scale)")
        ax.legend()
        ax.grid(True, which="both", alpha=0.3)
        fig.savefig(os.path.join(plots_dir, "low_entropy_insert_log.png"), dpi=120, bbox_inches="tight")
        plt.close(fig)
        print("Wrote", os.path.join(plots_dir, "low_entropy_insert_log.png"))

    # Load factor: load_factor vs insert time
    lf_data = {}
    for name in ["python_hashmap_loadfactor.csv", "java_hashmap_loadfactor.csv", "cpp_hashmap_loadfactor.csv", "rust_hashmap_loadfactor.csv"]:
        path = os.path.join(raw_dir, name)
        if os.path.isfile(path):
            lang, rows = load_loadfactor_csv(path)
            lf_data[lang] = rows
    if lf_data:
        fig, ax = plt.subplots()
        for lang, rows in lf_data.items():
            lf = [float(r["load_factor"]) for r in rows]
            insert = [float(r.get("insert_mean_ms", r.get("insert_ms", 0))) for r in rows]
            ax.plot(lf, insert, "o-", label=lang_display(lang), markersize=6)
        ax.set_xlabel("Load factor")
        ax.set_ylabel("Insert time (ms)")
        ax.set_title("HashMap load factor sensitivity (N=100k)")
        ax.legend()
        ax.grid(True, alpha=0.3)
        fig.savefig(os.path.join(plots_dir, "loadfactor_insert.png"), dpi=120, bbox_inches="tight")
        plt.close(fig)
        print("Wrote", os.path.join(plots_dir, "loadfactor_insert.png"))

    print("Plots saved to", plots_dir)

if __name__ == "__main__":
    main()
