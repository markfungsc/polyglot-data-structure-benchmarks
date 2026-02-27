#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_DIR="${ROOT_DIR}/results/raw"
export RESULTS_DIR
mkdir -p "${RESULTS_DIR}"

mode="${1:-all}"

run_python() {
  echo "== Python benchmarks (stub) =="
  python3 "${ROOT_DIR}/python/benchmarks/bench_array.py"
  python3 "${ROOT_DIR}/python/benchmarks/bench_hashmap.py"
  python3 "${ROOT_DIR}/python/benchmarks/bench_concurrency.py"
}

run_java() {
  echo "== Java benchmarks =="
  (cd "${ROOT_DIR}/java" && mvn -q compile -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.ArrayBenchmark) || echo "Java ArrayBenchmark failed. Run: cd java && mvn -e exec:java -Dexec.mainClass=com.polyglot.benchmarks.ArrayBenchmark"
  (cd "${ROOT_DIR}/java" && mvn -q compile -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.HashMapBenchmark) || echo "Java HashMapBenchmark failed."
  (cd "${ROOT_DIR}/java" && mvn -q compile -DskipTests exec:java -Dexec.mainClass=com.polyglot.benchmarks.ConcurrencyBenchmark) || echo "Java ConcurrencyBenchmark failed."
}

run_cpp() {
  echo "== C++ benchmarks (stub) =="
  (cd "${ROOT_DIR}/cpp" && cmake -S . -B build -DCMAKE_BUILD_TYPE=Release >/dev/null && cmake --build build >/dev/null && \
    ./build/bench_array || echo "C++ bench_array missing or failed.")
  (cd "${ROOT_DIR}/cpp" && ./build/bench_hashmap || echo "C++ bench_hashmap missing or failed.")
  (cd "${ROOT_DIR}/cpp" && ./build/bench_concurrency || echo "C++ bench_concurrency missing or failed.")
}

run_rust() {
  echo "== Rust benchmarks (stub) =="
  (cd "${ROOT_DIR}/rust" && cargo bench -q || echo "Rust benches (stub) - may be empty.")
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

