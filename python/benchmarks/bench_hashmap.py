import sys
import os
import random
import csv
import statistics
import time

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from src.hashmap import HashMap

SCALES = [1_000, 10_000, 100_000, 1_000_000]
NUM_RUNS = 5
LOW_ENTROPY_CAPACITY = 64  # low-entropy / near-collision: few buckets, many keys
LOAD_FACTOR_N = 100_000
LOAD_FACTORS = [0.25, 0.5, 0.75, 1.0]


def get_memory_mb():
    """Peak RSS in MB (Linux). Returns 0.0 if not available."""
    try:
        import resource
        return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024.0  # KB -> MB
    except Exception:
        return 0.0


def run_one_insert(N, capacity):
    keys = list(range(N))
    random.shuffle(keys)
    map1 = HashMap(capacity=max(16, capacity))
    for k in keys:
        map1.insert(k, k)


def run_one_get(N, capacity):
    keys = list(range(N))
    random.shuffle(keys)
    map1 = HashMap(capacity=max(16, capacity))
    for k in keys:
        map1.insert(k, k)
    for k in keys:
        _ = map1.get(k)


def measure_insert_times(N, capacity, warmup=True):
    if warmup:
        run_one_insert(N, capacity)
    times_ms = []
    for _ in range(NUM_RUNS):
        start = time.perf_counter()
        run_one_insert(N, capacity)
        times_ms.append((time.perf_counter() - start) * 1000)
    return times_ms


def measure_get_times(N, capacity, warmup=True):
    if warmup:
        run_one_get(N, capacity)
    times_ms = []
    for _ in range(NUM_RUNS):
        start = time.perf_counter()
        run_one_get(N, capacity)
        times_ms.append((time.perf_counter() - start) * 1000)
    return times_ms


def mean_std(times_ms):
    if len(times_ms) < 2:
        return statistics.mean(times_ms), 0.0
    return statistics.mean(times_ms), statistics.stdev(times_ms)


def write_csv(rows, results_dir, filename):
    path = os.path.join(results_dir, filename)
    with open(path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerows(rows)
    print(f"Wrote {path}")


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    results_dir = os.path.join(script_dir, "..", "..", "results", "raw")
    os.makedirs(results_dir, exist_ok=True)

    # ---- Main scenario: scaled, random keys, with std dev and memory ----
    rows_main = [["N", "insert_mean_ms", "insert_std_ms", "get_mean_ms", "get_std_ms", "memory_mb"]]
    for N in SCALES:
        capacity = max(16, N)
        mem_before = get_memory_mb()
        insert_times = measure_insert_times(N, capacity, warmup=True)
        get_times = measure_get_times(N, capacity, warmup=True)
        mem_after = get_memory_mb()
        memory_mb = max(0.0, mem_after - mem_before) if mem_after else 0.0

        i_mean, i_std = mean_std(insert_times)
        g_mean, g_std = mean_std(get_times)
        rows_main.append([N, f"{i_mean:.6f}", f"{i_std:.6f}", f"{g_mean:.6f}", f"{g_std:.6f}", f"{memory_mb:.4f}"])
        print(f"N={N}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Get {g_mean:.6f} ± {g_std:.6f} ms, memory={memory_mb:.4f} MB")
    write_csv(rows_main, results_dir, "python_hashmap.csv")

    # ---- Low-entropy / near-collision: small fixed capacity ----
    rows_low_entropy = [["N", "insert_mean_ms", "insert_std_ms", "get_mean_ms", "get_std_ms"]]
    for N in SCALES:
        insert_times = measure_insert_times(N, LOW_ENTROPY_CAPACITY, warmup=True)
        get_times = measure_get_times(N, LOW_ENTROPY_CAPACITY, warmup=True)
        i_mean, i_std = mean_std(insert_times)
        g_mean, g_std = mean_std(get_times)
        rows_low_entropy.append([N, f"{i_mean:.6f}", f"{i_std:.6f}", f"{g_mean:.6f}", f"{g_std:.6f}"])
        print(f"Low-entropy N={N}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Get {g_mean:.6f} ± {g_std:.6f} ms")
    write_csv(rows_low_entropy, results_dir, "python_hashmap_low_entropy.csv")

    # ---- Load factor sensitivity: fixed N, vary capacity ----
    rows_lf = [["load_factor", "insert_mean_ms", "insert_std_ms", "get_mean_ms", "get_std_ms"]]
    N = LOAD_FACTOR_N
    for lf in LOAD_FACTORS:
        capacity = max(16, int(N / lf))
        insert_times = measure_insert_times(N, capacity, warmup=True)
        get_times = measure_get_times(N, capacity, warmup=True)
        i_mean, i_std = mean_std(insert_times)
        g_mean, g_std = mean_std(get_times)
        rows_lf.append([lf, f"{i_mean:.6f}", f"{i_std:.6f}", f"{g_mean:.6f}", f"{g_std:.6f}"])
        print(f"LoadFactor={lf}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Get {g_mean:.6f} ± {g_std:.6f} ms")
    write_csv(rows_lf, results_dir, "python_hashmap_loadfactor.csv")


if __name__ == "__main__":
    main()
