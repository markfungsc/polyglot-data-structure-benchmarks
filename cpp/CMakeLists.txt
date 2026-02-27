cmake_minimum_required(VERSION 3.14)
project(polyglot_benchmarks CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(ds src/dynamic_array.cpp src/linked_list.cpp src/hashmap.cpp src/heap.cpp src/lru_cache.cpp)

add_executable(bench_array benchmarks/bench_array.cpp)
target_link_libraries(bench_array ds)

add_executable(bench_hashmap benchmarks/bench_hashmap.cpp)
target_link_libraries(bench_hashmap ds)

add_executable(bench_concurrency benchmarks/bench_concurrency.cpp)
target_link_libraries(bench_concurrency ds)
