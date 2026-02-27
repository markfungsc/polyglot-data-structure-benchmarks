#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/results/raw"
mkdir -p "${RESULTS_DIR}"

mode="${1:-all}"
timestamp="$(date +%Y%m%d_%H%M%S)"

run_python() {
  echo "== Python benchmarks (stub) =="
  python3 "${ROOT_DIR}/python/benchmarks/bench_array.py" | tee "${RESULTS_DIR}/python_array_${timestamp}.log"
  python3 "${ROOT_DIR}/python/benchmarks/bench_hashmap.py" | tee "${RESULTS_DIR}/python_hashmap_${timestamp}.log"
  python3 "${ROOT_DIR}/python/benchmarks/bench_concurrency.py" | tee "${RESULTS_DIR}/python_concurrency_${timestamp}.log"
}

run_java() {
  echo "== Java benchmarks (stub) =="
  (cd "${ROOT_DIR}/java" && mvn -q -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.ArrayBenchmark || echo "Java ArrayBenchmark stub failed (ensure Maven/Java and exec plugin are configured).") | tee "${RESULTS_DIR}/java_array_${timestamp}.log"
  (cd "${ROOT_DIR}/java" && mvn -q -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.HashMapBenchmark || echo "Java HashMapBenchmark stub failed.") | tee "${RESULTS_DIR}/java_hashmap_${timestamp}.log"
  (cd "${ROOT_DIR}/java" && mvn -q -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.ConcurrencyBenchmark || echo "Java ConcurrencyBenchmark stub failed.") | tee "${RESULTS_DIR}/java_concurrency_${timestamp}.log"
}

run_cpp() {
  echo "== C++ benchmarks (stub) =="
  (cd "${ROOT_DIR}/cpp" && cmake -S . -B build >/dev/null && cmake --build build >/dev/null && \
    ./build/bench_array || echo "C++ bench_array missing or failed.") | tee "${RESULTS_DIR}/cpp_array_${timestamp}.log"
  (cd "${ROOT_DIR}/cpp" && ./build/bench_hashmap || echo "C++ bench_hashmap missing or failed.") | tee "${RESULTS_DIR}/cpp_hashmap_${timestamp}.log"
  (cd "${ROOT_DIR}/cpp" && ./build/bench_concurrency || echo "C++ bench_concurrency missing or failed.") | tee "${RESULTS_DIR}/cpp_concurrency_${timestamp}.log"
}

run_rust() {
  echo "== Rust benchmarks (stub) =="
  (cd "${ROOT_DIR}/rust" && cargo bench -q || echo "Rust benches (stub) - may be empty.") | tee "${RESULTS_DIR}/rust_${timestamp}.log"
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
    echo "Usage: $0 [python|java|cpp|rust|all]" >&2
    exit 1
    ;;
esac

