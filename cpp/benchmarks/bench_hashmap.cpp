#include "hashmap.hpp"
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
static constexpr size_t LOW_ENTROPY_CAPACITY = 64;  // low-entropy / near-collision: few buckets
static constexpr int LOAD_FACTOR_N = 100'000;
static constexpr double LOAD_FACTORS[] = {0.25, 0.5, 0.75, 1.0};

static double memory_mb() {
#if defined(__linux__) || defined(__APPLE__)
    struct rusage u;
    if (getrusage(RUSAGE_SELF, &u) == 0)
        return u.ru_maxrss / 1024.0;  // KB -> MB (Linux); macOS is bytes, adjust if needed
#endif
    return 0.0;
}

static void mean_std(const std::vector<double>& v, double& mean, double& stddev) {
    mean = 0;
    for (double x : v) mean += x;
    mean /= v.size();
    stddev = 0;
    if (v.size() >= 2) {
        for (double x : v) stddev += (x - mean) * (x - mean);
        stddev = std::sqrt(stddev / (v.size() - 1));
    }
}

int main() {
    const char* results_env = std::getenv("RESULTS_DIR");
    std::string out_dir = results_env ? results_env : "../results/raw";
    fs::create_directories(out_dir);

    std::random_device rd;
    std::mt19937 g(rd());

    // ---- Main scenario ----
    std::string csv_path = out_dir + "/cpp_hashmap.csv";
    std::ofstream file(csv_path);
    if (!file.is_open()) {
        std::cerr << "Failed to open " << csv_path << "\n";
        return 1;
    }
    file << "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb\n";
    file << std::fixed << std::setprecision(6);

    for (int n : SCALES) {
        std::vector<int> keys(n);
        std::iota(keys.begin(), keys.end(), 0);
        std::shuffle(keys.begin(), keys.end(), g);

        {  // warm-up
            hashmap::HashMap map(static_cast<size_t>(std::max(16, n)));
            for (int k : keys) map.insert(k, k);
            for (int k : keys) do_not_optimize(map.get(k));
        }

        std::vector<double> insert_ms(NUM_RUNS), get_ms(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; run++) {
            std::shuffle(keys.begin(), keys.end(), g);
            hashmap::HashMap map(static_cast<size_t>(std::max(16, n)));
            auto start = std::chrono::high_resolution_clock::now();
            for (int k : keys) map.insert(k, k);
            insert_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
            start = std::chrono::high_resolution_clock::now();
            for (int k : keys) do_not_optimize(map.get(k));
            get_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
        }
        double i_mean, i_std, g_mean, g_std;
        mean_std(insert_ms, i_mean, i_std);
        mean_std(get_ms, g_mean, g_std);
        double mem = memory_mb();
        file << n << "," << i_mean << "," << i_std << "," << g_mean << "," << g_std << "," << std::setprecision(4) << mem << "\n";
        file << std::setprecision(6);
        std::cout << "N=" << n << ": Insert " << i_mean << " ± " << i_std << " ms, Get " << g_mean << " ± " << g_std << " ms, memory=" << mem << " MB\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    file.close();

    // ---- Low-entropy / near-collision ----
    csv_path = out_dir + "/cpp_hashmap_low_entropy.csv";
    file.open(csv_path);
    file << "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms\n";
    file << std::fixed << std::setprecision(6);
    for (int n : SCALES) {
        std::vector<int> keys(n);
        std::iota(keys.begin(), keys.end(), 0);
        std::shuffle(keys.begin(), keys.end(), g);
        { hashmap::HashMap map(LOW_ENTROPY_CAPACITY); for (int k : keys) map.insert(k, k); for (int k : keys) do_not_optimize(map.get(k)); }
        std::vector<double> insert_ms(NUM_RUNS), get_ms(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; run++) {
            std::shuffle(keys.begin(), keys.end(), g);
            hashmap::HashMap map(LOW_ENTROPY_CAPACITY);
            auto start = std::chrono::high_resolution_clock::now();
            for (int k : keys) map.insert(k, k);
            insert_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
            start = std::chrono::high_resolution_clock::now();
            for (int k : keys) do_not_optimize(map.get(k));
            get_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
        }
        double i_mean, i_std, g_mean, g_std;
        mean_std(insert_ms, i_mean, i_std);
        mean_std(get_ms, g_mean, g_std);
        file << n << "," << i_mean << "," << i_std << "," << g_mean << "," << g_std << "\n";
        std::cout << "Low-entropy N=" << n << ": Insert " << i_mean << " ± " << i_std << " ms, Get " << g_mean << " ± " << g_std << " ms\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    file.close();

    // ---- Load factor sensitivity ----
    csv_path = out_dir + "/cpp_hashmap_loadfactor.csv";
    file.open(csv_path);
    file << "load_factor,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms\n";
    file << std::fixed << std::setprecision(6);
    int n = LOAD_FACTOR_N;
    std::vector<int> keys(n);
    std::iota(keys.begin(), keys.end(), 0);
    for (double lf : LOAD_FACTORS) {
        std::shuffle(keys.begin(), keys.end(), g);
        size_t cap = std::max(size_t(16), size_t(n / lf));
        { hashmap::HashMap map(cap); for (int k : keys) map.insert(k, k); for (int k : keys) do_not_optimize(map.get(k)); }
        std::vector<double> insert_ms(NUM_RUNS), get_ms(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; run++) {
            std::shuffle(keys.begin(), keys.end(), g);
            hashmap::HashMap map(cap);
            auto start = std::chrono::high_resolution_clock::now();
            for (int k : keys) map.insert(k, k);
            insert_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
            start = std::chrono::high_resolution_clock::now();
            for (int k : keys) do_not_optimize(map.get(k));
            get_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
        }
        double i_mean, i_std, g_mean, g_std;
        mean_std(insert_ms, i_mean, i_std);
        mean_std(get_ms, g_mean, g_std);
        file << lf << "," << i_mean << "," << i_std << "," << g_mean << "," << g_std << "\n";
        std::cout << "LoadFactor=" << lf << ": Insert " << i_mean << " ± " << i_std << " ms, Get " << g_mean << " ± " << g_std << " ms\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    return 0;
}
