#!/usr/bin/env python3
"""LRU cache benchmark: put_miss, put_hit, get_hit, get_miss, eviction (same as Rust)."""

import sys
import os
import random
import time

_script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _script_dir)
sys.path.insert(0, os.path.join(_script_dir, ".."))
from src.lru_cache import LRUCache
from bench_common import (
    SCALES,
    NUM_RUNS,
    get_memory_mb,
    mean_std,
    write_csv,
    get_results_dir,
)


def main():
    results_dir = get_results_dir()
    os.makedirs(results_dir, exist_ok=True)

    rows = [
        [
            "N",
            "put_miss_mean_ms",
            "put_miss_std_ms",
            "put_hit_mean_ms",
            "put_hit_std_ms",
            "get_hit_mean_ms",
            "get_hit_std_ms",
            "get_miss_mean_ms",
            "get_miss_std_ms",
            "eviction_mean_ms",
            "eviction_std_ms",
            "memory_mb",
        ]
    ]
    for N in SCALES:
        keys = list(range(N))
        capacity = max(16, N)

        # Warm-up: build and use cache once at this scale
        cache = LRUCache(capacity)
        for k in keys:
            cache.put(k, k)
        for k in keys:
            _ = cache.get(k)

        put_miss_times = []
        put_hit_times = []
        get_hit_times = []
        get_miss_times = []
        eviction_times = []

        for _ in range(NUM_RUNS):
            # put_miss: cache empty, then (capacity-1) puts of new keys (no eviction)
            cache = LRUCache(capacity)
            start = time.perf_counter()
            for i in range(capacity - 1):
                cache.put(i, i)
            put_miss_times.append((time.perf_counter() - start) * 1000)

            # put_hit: full cache, N updates of existing keys
            cache = LRUCache(capacity)
            for k in keys:
                cache.put(k, k)
            start = time.perf_counter()
            for i in range(N):
                cache.put(i % capacity, i)
            put_hit_times.append((time.perf_counter() - start) * 1000)

            # get_hit: full cache, N lookups of existing keys
            cache = LRUCache(capacity)
            for k in keys:
                cache.put(k, k)
            start = time.perf_counter()
            for i in range(N):
                _ = cache.get(i % capacity)
            get_hit_times.append((time.perf_counter() - start) * 1000)

            # get_miss: full cache, N lookups of a key not in cache
            cache = LRUCache(capacity)
            for k in keys:
                cache.put(k, k)
            missing = N  # not in 0..N
            start = time.perf_counter()
            for _ in range(N):
                _ = cache.get(missing)
            get_miss_times.append((time.perf_counter() - start) * 1000)

            # eviction: full cache, N puts of new keys so each put evicts LRU
            cache = LRUCache(capacity)
            for k in keys:
                cache.put(k, k)
            start = time.perf_counter()
            for i in range(N, 2 * N):
                cache.put(i, i)
            eviction_times.append((time.perf_counter() - start) * 1000)

        pm_mean, pm_std = mean_std(put_miss_times)
        ph_mean, ph_std = mean_std(put_hit_times)
        gh_mean, gh_std = mean_std(get_hit_times)
        gm_mean, gm_std = mean_std(get_miss_times)
        ev_mean, ev_std = mean_std(eviction_times)
        memory_mb = get_memory_mb()
        rows.append(
            [
                N,
                f"{pm_mean:.6f}",
                f"{pm_std:.6f}",
                f"{ph_mean:.6f}",
                f"{ph_std:.6f}",
                f"{gh_mean:.6f}",
                f"{gh_std:.6f}",
                f"{gm_mean:.6f}",
                f"{gm_std:.6f}",
                f"{ev_mean:.6f}",
                f"{ev_std:.6f}",
                f"{memory_mb:.4f}",
            ]
        )
        print(
            f"N={N}: put_miss {pm_mean:.6f}±{pm_std:.6f} ms, put_hit {ph_mean:.6f}±{ph_std:.6f} ms, "
            f"get_hit {gh_mean:.6f}±{gh_std:.6f} ms, get_miss {gm_mean:.6f}±{gm_std:.6f} ms, "
            f"eviction {ev_mean:.6f}±{ev_std:.6f} ms, memory={memory_mb:.4f} MB"
        )
    write_csv(rows, results_dir, "python_lru_cache.csv")


if __name__ == "__main__":
    main()
