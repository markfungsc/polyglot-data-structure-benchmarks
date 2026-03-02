#ifndef BENCH_COMMON_HPP
#define BENCH_COMMON_HPP

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <random>
#include <string>
#include <vector>

#if defined(__linux__) || defined(__APPLE__)
#include <sys/resource.h>
#endif

template <typename T>
inline void do_not_optimize(const T& value) {
    asm volatile("" : : "g"(value) : "memory");
}

namespace fs = std::filesystem;

static constexpr int SCALES[] = {1000, 10'000, 100'000, 1'000'000};
static constexpr int NUM_RUNS = 5;

inline double memory_mb() {
#if defined(__linux__) || defined(__APPLE__)
    struct rusage u;
    if (getrusage(RUSAGE_SELF, &u) == 0)
        return u.ru_maxrss / 1024.0;  // KB -> MB (Linux); macOS is bytes, adjust if needed
#endif
    return 0.0;
}

inline void mean_std(const std::vector<double>& v, double& mean, double& stddev) {
    mean = 0;
    for (double x : v) mean += x;
    mean /= v.size();
    stddev = 0;
    if (v.size() >= 2) {
        for (double x : v) stddev += (x - mean) * (x - mean);
        stddev = std::sqrt(stddev / (v.size() - 1));
    }
}

inline std::string get_results_dir() {
    const char* results_env = std::getenv("RESULTS_DIR");
    return results_env ? results_env : "../results/raw";
}

#endif
