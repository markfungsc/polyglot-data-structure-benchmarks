"""Shared constants and helpers for benchmark scripts (same schema and methodology)."""

import os
import csv
import statistics

SCALES = [1_000, 10_000, 100_000, 1_000_000]
NUM_RUNS = 5


def get_memory_mb():
    """Peak RSS in MB (Linux). Returns 0.0 if not available."""
    try:
        import resource

        return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024.0  # KB -> MB
    except Exception:
        return 0.0


def mean_std(times_ms):
    if len(times_ms) < 2:
        return statistics.mean(times_ms), 0.0
    return statistics.mean(times_ms), statistics.stdev(times_ms)


def get_results_dir():
    """RESULTS_DIR env or default script-relative results/raw."""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default = os.path.join(script_dir, "..", "..", "results", "raw")
    return os.environ.get("RESULTS_DIR", default)


def write_csv(rows, results_dir, filename):
    path = os.path.join(results_dir, filename)
    with open(path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerows(rows)
    print(f"Wrote {path}")
