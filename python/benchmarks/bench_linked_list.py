#!/usr/bin/env python3
"""Linked list benchmark: push (insert) and get by index. Same schema as hashmap."""
import sys
import os
import random
import time

_script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _script_dir)
sys.path.insert(0, os.path.join(_script_dir, ".."))
from src.linked_list import LinkedList
from bench_common import SCALES, NUM_RUNS, get_memory_mb, mean_std, write_csv, get_results_dir


def main():
    results_dir = get_results_dir()
    os.makedirs(results_dir, exist_ok=True)

    rows = [["N", "insert_mean_ms", "insert_std_ms", "get_mean_ms", "get_std_ms", "delete_mean_ms", "delete_std_ms", "memory_mb"]]
    for N in SCALES:
        keys = list(range(N))
        random.shuffle(keys)

        # warm-up
        lst = LinkedList()
        for k in keys:
            lst.push_back(k)
        sum_val = 0
        def add_to_total(x):
            nonlocal sum_val
            sum_val += x
        lst.traverse(add_to_total)
        lst.delete(lst.size() - 1)

        insert_times = []
        get_times = []
        delete_times = []
        for _ in range(NUM_RUNS):
            random.shuffle(keys)
            lst = LinkedList()
            start = time.perf_counter()
            for k in keys:
                lst.push_back(k)
            insert_times.append((time.perf_counter() - start) * 1000)
            start = time.perf_counter()
            sum_val = 0
            def add_to_total(x):
                nonlocal sum_val
                sum_val += x
            lst.traverse(add_to_total)
            get_times.append((time.perf_counter() - start) * 1000)
            start = time.perf_counter()
            lst.delete(lst.size() - 1)
            delete_times.append((time.perf_counter() - start) * 1000)

        i_mean, i_std = mean_std(insert_times)
        g_mean, g_std = mean_std(get_times)
        d_mean, d_std = mean_std(delete_times)
        memory_mb = get_memory_mb()
        rows.append([N, f"{i_mean:.6f}", f"{i_std:.6f}", f"{g_mean:.6f}", f"{g_std:.6f}", f"{d_mean:.6f}", f"{d_std:.6f}", f"{memory_mb:.4f}"])
        print(f"N={N}: Insert {i_mean:.6f} ± {i_std:.6f} ms, Get {g_mean:.6f} ± {g_std:.6f} ms, Delete {d_mean:.6f} ± {d_std:.6f} ms, memory={memory_mb:.4f} MB")
    write_csv(rows, results_dir, "python_linked_list.csv")


if __name__ == "__main__":
    main()
