import sys
import os
import random
import time

_script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _script_dir)
sys.path.insert(0, os.path.join(_script_dir, ".."))
from src.hashmap import HashMap
from bench_common import SCALES, NUM_RUNS, get_memory_mb, mean_std, write_csv, get_results_dir

LOW_ENTROPY_CAPACITY = 64  # low-entropy / near-collision: few buckets, many keys
LOAD_FACTOR_N = 100_000
LOAD_FACTORS = [0.25, 0.5, 0.75, 1.0]


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


def main():
    results_dir = get_results_dir()
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
