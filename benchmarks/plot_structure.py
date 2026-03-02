#!/usr/bin/env python3
"""
Plot benchmark CSVs for any structure (same schema as hashmap: N, insert_mean_ms, get_mean_ms, memory_mb).
Usage: python3 plot_structure.py --structure heap [--raw-dir DIR] [--plots-dir DIR]
Finds <lang>_<structure>.csv in raw_dir and writes insert_log.png, get_log.png, memory_log.png to plots_dir.
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

def load_structure_csv(path, structure):
    """Load structure CSV (same schema as hashmap main); return (lang, rows)."""
    basename = os.path.basename(path)
    suffix = f"_{structure}.csv"
    lang = basename.replace(suffix, "") if basename.endswith(suffix) else basename.replace(".csv", "").split("_", 1)[0]
    rows = []
    with open(path, newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            row["_lang"] = lang
            if "insert_mean_ms" not in row and "insert_ms" in row:
                row["insert_mean_ms"] = row["insert_ms"]
                row["insert_std_ms"] = row.get("insert_std_ms", "0")
            if "get_mean_ms" not in row and "get_ms" in row:
                row["get_mean_ms"] = row["get_ms"]
                row["get_std_ms"] = row.get("get_std_ms", "0")
            if "memory_mb" not in row:
                row["memory_mb"] = "0"
            rows.append(row)
    return lang, rows

def main():
    parser = argparse.ArgumentParser(description="Plot benchmark CSVs for a given structure (same schema as hashmap).")
    parser.add_argument("--structure", required=True, help="Structure name (e.g. heap, dynamic_array, linked_list, lru_cache)")
    parser.add_argument("--raw-dir", help="Directory containing *_<structure>.csv")
    parser.add_argument("--plots-dir", help="Directory to write PNGs")
    args = parser.parse_args()

    raw_dir = args.raw_dir or os.environ.get("RESULTS_RAW_DIR") or os.environ.get("RESULTS_DIR") or _default_results_dir()
    plots_dir = args.plots_dir or os.environ.get("RESULTS_PLOTS_DIR") or _default_plots_dir()
    raw_dir = os.path.abspath(raw_dir)
    plots_dir = os.path.abspath(plots_dir)
    structure = args.structure
    suffix = f"_{structure}.csv"

    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
    except ImportError:
        print("matplotlib not found. Install with: pip install matplotlib", file=sys.stderr)
        sys.exit(1)

    os.makedirs(plots_dir, exist_ok=True)

    # Discover *_<structure>.csv
    main_data = {}
    for name in os.listdir(raw_dir):
        if name.endswith(suffix):
            path = os.path.join(raw_dir, name)
            if os.path.isfile(path):
                lang, rows = load_structure_csv(path, structure)
                main_data[lang] = rows

    if not main_data:
        print(f"No *{suffix} files found in {raw_dir}", file=sys.stderr)
        sys.exit(1)

    prefix = structure.replace("_", "-")
    title_prefix = structure.replace("_", " ").title()

    # Plot 1: N vs insert time (log-log)
    fig, ax = plt.subplots()
    for lang, rows in main_data.items():
        N = [int(r["N"]) for r in rows]
        insert = [float(r.get("insert_mean_ms", r.get("insert_ms", 0))) for r in rows]
        ax.loglog(N, insert, "o-", label=lang_display(lang), markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel("Insert time (ms)")
    ax.set_title(f"{title_prefix} insert (log scale)")
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(os.path.join(plots_dir, f"{prefix}_insert_log.png"), dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", os.path.join(plots_dir, f"{prefix}_insert_log.png"))

    # Plot 2: N vs get time (log-log)
    fig, ax = plt.subplots()
    for lang, rows in main_data.items():
        N = [int(r["N"]) for r in rows]
        get_t = [float(r.get("get_mean_ms", r.get("get_ms", 0))) for r in rows]
        ax.loglog(N, get_t, "o-", label=lang_display(lang), markersize=6)
    ax.set_xlabel("N (number of elements)")
    ax.set_ylabel("Get time (ms)")
    ax.set_title(f"{title_prefix} get (log scale)")
    ax.legend()
    ax.grid(True, which="both", alpha=0.3)
    fig.savefig(os.path.join(plots_dir, f"{prefix}_get_log.png"), dpi=120, bbox_inches="tight")
    plt.close(fig)
    print("Wrote", os.path.join(plots_dir, f"{prefix}_get_log.png"))

    # Plot 3: N vs memory if any language has memory_mb > 0
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
        ax.set_title(f"{title_prefix} memory (log N)")
        ax.legend()
        ax.grid(True, which="both", alpha=0.3)
        fig.savefig(os.path.join(plots_dir, f"{prefix}_memory_log.png"), dpi=120, bbox_inches="tight")
        plt.close(fig)
        print("Wrote", os.path.join(plots_dir, f"{prefix}_memory_log.png"))

    print("Plots saved to", plots_dir)

if __name__ == "__main__":
    main()
