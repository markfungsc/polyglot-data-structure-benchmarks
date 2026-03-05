#!/usr/bin/env python3
"""Heap benchmark: push (insert) and pop (remove). Same schema as hashmap."""

import sys
import os
import random
import time

_script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _script_dir)
sys.path.insert(0, os.path.join(_script_dir, ".."))
from src.heap import MinHeap
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
            "insert_mean_ms",
            "insert_std_ms",
            "get_mean_ms",
            "get_std_ms",
            "memory_mb",
        ]
    ]
    for N in SCALES:
        keys = list(range(N))
        random.shuffle(keys)

        # warm-up
        h = MinHeap()
        for k in keys:
            h.insert(k)
        while h.size() > 0:
            _ = h.pop()

        insert_times = []
        pop_times = []
        for _ in range(NUM_RUNS):
            random.shuffle(keys)
            h = MinHeap()
            start = time.perf_counter()
            for k in keys:
                h.insert(k)
            insert_times.append((time.perf_counter() - start) * 1000)
            start = time.perf_counter()
            while h.size() > 0:
                _ = h.pop()
            pop_times.append((time.perf_counter() - start) * 1000)

        i_mean, i_std = mean_std(insert_times)
        p_mean, p_std = mean_std(pop_times)
        memory_mb = get_memory_mb()
        rows.append(
            [
                N,
                f"{i_mean:.6f}",
                f"{i_std:.6f}",
                f"{p_mean:.6f}",
                f"{p_std:.6f}",
                f"{memory_mb:.4f}",
            ]
        )
        print(
            f"N={N}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Pop {p_mean:.6f} ± {p_std:.6f} ms, memory={memory_mb:.4f} MB"
        )
    write_csv(rows, results_dir, "python_heap.csv")


if __name__ == "__main__":
    main()
