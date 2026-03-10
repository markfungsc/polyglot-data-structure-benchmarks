#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/results/raw"
export RESULTS_DIR
mkdir -p "${RESULTS_DIR}"

mode="${1:-all}"
bench="${2:-}"

BENCHMARKS="array hashmap heap linked_list lru_cache native_lru_cache concurrency workload_dynamic_array workload_hashmap workload_heap"

valid_bench() {
  local b
  for b in $BENCHMARKS; do
    if [[ "$b" == "$1" ]]; then return 0; fi
  done
  return 1
}

if [[ -n "$bench" ]] && ! valid_bench "$bench"; then
  echo "Usage: $0 [python|java|cpp|rust|all] [array|hashmap|heap|linked_list|lru_cache|native_lru_cache|concurrency|workload_dynamic_array|workload_hashmap|workload_heap]" >&2
  exit 1
fi

run_python_one() {
  local name="$1"
  case "$name" in
    array)             python3 -O "${ROOT_DIR}/python/benchmarks/bench_array.py" ;;
    hashmap)           python3 -O "${ROOT_DIR}/python/benchmarks/bench_hashmap.py" ;;
    heap)              python3 -O "${ROOT_DIR}/python/benchmarks/bench_heap.py" ;;
    linked_list)       python3 -O "${ROOT_DIR}/python/benchmarks/bench_linked_list.py" ;;
    lru_cache)         python3 -O "${ROOT_DIR}/python/benchmarks/bench_lru_cache.py" ;;
    native_lru_cache)  ;;  # Rust only
    concurrency)       python3 -O "${ROOT_DIR}/python/benchmarks/bench_concurrency.py" ;;
    workload_dynamic_array)  ;;  # Rust only
    workload_hashmap)  ;;  # Rust only
    workload_heap)     ;;  # Rust only
    *) echo "Unknown benchmark: $name" >&2; return 1 ;;
  esac
}

run_python() {
  echo "== Python benchmarks =="
  if [[ -z "$bench" ]]; then
    for b in $BENCHMARKS; do run_python_one "$b" || true; done
  else
    run_python_one "$bench" || echo "Python $bench failed."
  fi
}

run_java_one() {
  local name="$1"
  local class
  case "$name" in
    array)             class="com.polyglot.benchmarks.ArrayBenchmark" ;;
    hashmap)           class="com.polyglot.benchmarks.HashMapBenchmark" ;;
    heap)              class="com.polyglot.benchmarks.HeapBenchmark" ;;
    linked_list)       class="com.polyglot.benchmarks.LinkedListBenchmark" ;;
    lru_cache)         class="com.polyglot.benchmarks.LRUCacheBenchmark" ;;
    native_lru_cache)  return 0 ;;  # Rust only
    concurrency)       class="com.polyglot.benchmarks.ConcurrencyBenchmark" ;;
    workload_dynamic_array)  return 0 ;;  # Rust only
    workload_hashmap)  return 0 ;;  # Rust only
    workload_heap)     return 0 ;;  # Rust only
    *) echo "Unknown benchmark: $name" >&2; return 1 ;;
  esac
  (cd "${ROOT_DIR}/java" && mvn -q compile -DskipTests exec:java -Dexec.mainClass="$class") || echo "Java $name failed."
}

run_java() {
  echo "== Java benchmarks =="
  if [[ -z "$bench" ]]; then
    for b in $BENCHMARKS; do run_java_one "$b" || true; done
  else
    run_java_one "$bench"
  fi
}

run_cpp_one() {
  local name="$1"
  local bin
  case "$name" in
    array)             bin="bench_array" ;;
    hashmap)           bin="bench_hashmap" ;;
    heap)              bin="bench_heap" ;;
    linked_list)       bin="bench_linked_list" ;;
    lru_cache)         bin="bench_lru_cache" ;;
    native_lru_cache)  return 0 ;;  # Rust only
    concurrency)       bin="bench_concurrency" ;;
    workload_dynamic_array)  return 0 ;;  # Rust only
    workload_hashmap)  return 0 ;;  # Rust only
    workload_heap)     return 0 ;;  # Rust only
    *) echo "Unknown benchmark: $name" >&2; return 1 ;;
  esac
  (cd "${ROOT_DIR}/cpp" && ./build/"$bin") || echo "C++ $name failed."
}

run_cpp() {
  echo "== C++ benchmarks =="
  (cd "${ROOT_DIR}/cpp" && cmake -S . -B build -DCMAKE_BUILD_TYPE=Release >/dev/null && cmake --build build >/dev/null) || true
  if [[ -z "$bench" ]]; then
    for b in $BENCHMARKS; do run_cpp_one "$b" || true; done
  else
    run_cpp_one "$bench"
  fi
}

run_rust_one() {
  local name="$1"
  local target
  case "$name" in
    array)             target="bench_array" ;;
    hashmap)           target="bench_hashmap" ;;
    heap)              target="bench_heap" ;;
    linked_list)       target="bench_linked_list" ;;
    lru_cache)         target="bench_lru_cache" ;;
    native_lru_cache)  target="bench_native_lru_cache" ;;
    concurrency)       target="bench_concurrency" ;;
    workload_dynamic_array)  target="workload_dynamic_array" ;;
    workload_hashmap)  target="workload_hashmap" ;;
    workload_heap)     target="workload_heap" ;;
    *) echo "Unknown benchmark: $name" >&2; return 1 ;;
  esac
  (cd "${ROOT_DIR}/rust" && cargo bench -q --bench "$target") || echo "Rust $name failed."
}

run_rust() {
  echo "== Rust benchmarks =="
  if [[ -z "$bench" ]]; then
    (cd "${ROOT_DIR}/rust" && cargo bench -q) || echo "Rust benches - some may have failed."
  else
    run_rust_one "$bench"
  fi
}

case "${mode}" in
  python) run_python ;;
  java) run_java ;;
  cpp) run_cpp ;;
  rust) run_rust ;;
  all)
    run_python
    run_java
    run_cpp
    run_rust
    ;;
  *)
    echo "Usage: $0 [python|java|cpp|rust|all] [array|hashmap|heap|linked_list|lru_cache|native_lru_cache|concurrency|workload_dynamic_array|workload_hashmap|workload_heap]" >&2
    exit 1
    ;;
esac
