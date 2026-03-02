cmake_minimum_required(VERSION 3.14)
project(polyglot_benchmarks CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(ds src/dynamic_array.cpp src/linked_list.cpp src/hashmap.cpp src/heap.cpp src/lru_cache.cpp)

add_executable(bench_array benchmarks/bench_array.cpp)
target_include_directories(bench_array PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_SOURCE_DIR}/benchmarks)
target_link_libraries(bench_array ds)

add_executable(bench_hashmap benchmarks/bench_hashmap.cpp)
target_include_directories(bench_hashmap PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_SOURCE_DIR}/benchmarks)
target_link_libraries(bench_hashmap ds)

add_executable(bench_heap benchmarks/bench_heap.cpp)
target_include_directories(bench_heap PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_SOURCE_DIR}/benchmarks)
target_link_libraries(bench_heap ds)

add_executable(bench_linked_list benchmarks/bench_linked_list.cpp)
target_include_directories(bench_linked_list PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_SOURCE_DIR}/benchmarks)
target_link_libraries(bench_linked_list ds)

add_executable(bench_lru_cache benchmarks/bench_lru_cache.cpp)
target_include_directories(bench_lru_cache PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_SOURCE_DIR}/benchmarks)
target_link_libraries(bench_lru_cache ds)

add_executable(bench_concurrency benchmarks/bench_concurrency.cpp)
target_link_libraries(bench_concurrency ds)
