#!/usr/bin/env python3
"""LRU cache benchmark: put (insert) and get. Same schema as hashmap."""
import sys
import os
import random
import time

_script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _script_dir)
sys.path.insert(0, os.path.join(_script_dir, ".."))
from src.lru_cache import LRUCache
from bench_common import SCALES, NUM_RUNS, get_memory_mb, mean_std, write_csv, get_results_dir


def main():
    results_dir = get_results_dir()
    os.makedirs(results_dir, exist_ok=True)

    rows = [["N", "insert_mean_ms", "insert_std_ms", "get_mean_ms", "get_std_ms", "memory_mb"]]
    for N in SCALES:
        keys = list(range(N))
        random.shuffle(keys)
        capacity = max(16, N)

        # warm-up
        cache = LRUCache(capacity)
        for k in keys:
            cache.put(k, k)
        for k in keys:
            _ = cache.get(k)

        insert_times = []
        get_times = []
        for _ in range(NUM_RUNS):
            random.shuffle(keys)
            cache = LRUCache(capacity)
            start = time.perf_counter()
            for k in keys:
                cache.put(k, k)
            insert_times.append((time.perf_counter() - start) * 1000)
            start = time.perf_counter()
            for k in keys:
                _ = cache.get(k)
            get_times.append((time.perf_counter() - start) * 1000)

        i_mean, i_std = mean_std(insert_times)
        g_mean, g_std = mean_std(get_times)
        memory_mb = get_memory_mb()
        rows.append([N, f"{i_mean:.6f}", f"{i_std:.6f}", f"{g_mean:.6f}", f"{g_std:.6f}", f"{memory_mb:.4f}"])
        print(f"N={N}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Get {g_mean:.6f} ± {g_std:.6f} ms, memory={memory_mb:.4f} MB")
    write_csv(rows, results_dir, "python_lru_cache.csv")


if __name__ == "__main__":
    main()
