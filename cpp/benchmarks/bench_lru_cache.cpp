#include <chrono>
#include <random>

#include "bench_common.hpp"
#include "lru_cache.hpp"

int main() {
    std::string out_dir = get_results_dir();
    fs::create_directories(out_dir);

    std::random_device rd;
    std::mt19937 g(rd());

    std::string csv_path = out_dir + "/cpp_lru_cache.csv";
    std::ofstream file(csv_path);
    if (!file.is_open()) {
        std::cerr << "Failed to open " << csv_path << "\n";
        return 1;
    }
    file << "N,put_miss_mean_ms,put_miss_std_ms,put_hit_mean_ms,put_hit_std_ms,get_hit_mean_ms,"
            "get_hit_std_ms,get_miss_mean_ms,get_miss_std_ms,eviction_mean_ms,eviction_std_ms,"
            "memory_mb\n";
    file << std::fixed << std::setprecision(6);

    for (int n : SCALES) {
        std::vector<int> keys(n);
        std::iota(keys.begin(), keys.end(), 0);
        std::shuffle(keys.begin(), keys.end(), g);

        size_t capacity = std::max(size_t(16), static_cast<size_t>(n));
        {  // Warm-up: build and use cache once at this scale
            lru_cache::LRUCache cache(capacity);
            for (int k : keys) cache.put(k, k);
            for (int k : keys) do_not_optimize(cache.get(k));
        }

        std::vector<double> put_miss_ms(NUM_RUNS), put_hit_ms(NUM_RUNS), get_hit_ms(NUM_RUNS),
            get_miss_ms(NUM_RUNS), eviction_ms(NUM_RUNS);

        for (int run = 0; run < NUM_RUNS; run++) {
            // put_miss: cache empty, then (capacity-1) puts of new keys (no eviction)
            {
                lru_cache::LRUCache cache(capacity);
                auto start = std::chrono::high_resolution_clock::now();
                for (size_t i = 0; i < capacity - 1; i++)
                    cache.put(static_cast<int>(i), static_cast<int>(i));
                put_miss_ms[run] = std::chrono::duration<double, std::milli>(
                                       std::chrono::high_resolution_clock::now() - start)
                                       .count();
            }

            // put_hit: full cache, n updates of existing keys
            {
                lru_cache::LRUCache cache(capacity);
                for (int k : keys) cache.put(k, k);
                auto start = std::chrono::high_resolution_clock::now();
                for (int i = 0; i < n; i++) cache.put(i % static_cast<int>(capacity), i);
                put_hit_ms[run] = std::chrono::duration<double, std::milli>(
                                      std::chrono::high_resolution_clock::now() - start)
                                      .count();
            }

            // get_hit: full cache, n lookups of existing keys
            {
                lru_cache::LRUCache cache(capacity);
                for (int k : keys) cache.put(k, k);
                auto start = std::chrono::high_resolution_clock::now();
                for (int i = 0; i < n; i++)
                    do_not_optimize(cache.get(i % static_cast<int>(capacity)));
                get_hit_ms[run] = std::chrono::duration<double, std::milli>(
                                      std::chrono::high_resolution_clock::now() - start)
                                      .count();
            }

            // get_miss: full cache, n lookups of a key not in cache
            {
                lru_cache::LRUCache cache(capacity);
                for (int k : keys) cache.put(k, k);
                const int missing = n;
                auto start = std::chrono::high_resolution_clock::now();
                for (int i = 0; i < n; i++) do_not_optimize(cache.get(missing));
                get_miss_ms[run] = std::chrono::duration<double, std::milli>(
                                       std::chrono::high_resolution_clock::now() - start)
                                       .count();
            }

            // eviction: full cache, n puts of new keys so each put evicts LRU
            {
                lru_cache::LRUCache cache(capacity);
                for (int k : keys) cache.put(k, k);
                auto start = std::chrono::high_resolution_clock::now();
                for (int i = n; i < 2 * n; i++) cache.put(i, i);
                eviction_ms[run] = std::chrono::duration<double, std::milli>(
                                       std::chrono::high_resolution_clock::now() - start)
                                       .count();
            }
        }

        double pm_mean, pm_std, ph_mean, ph_std, gh_mean, gh_std, gm_mean, gm_std, ev_mean, ev_std;
        mean_std(put_miss_ms, pm_mean, pm_std);
        mean_std(put_hit_ms, ph_mean, ph_std);
        mean_std(get_hit_ms, gh_mean, gh_std);
        mean_std(get_miss_ms, gm_mean, gm_std);
        mean_std(eviction_ms, ev_mean, ev_std);
        double mem = memory_mb();
        file << n << "," << pm_mean << "," << pm_std << "," << ph_mean << "," << ph_std << ","
             << gh_mean << "," << gh_std << "," << gm_mean << "," << gm_std << "," << ev_mean << ","
             << ev_std << "," << std::setprecision(4) << mem << "\n";
        file << std::setprecision(6);
        std::cout << "N=" << n << ": put_miss " << pm_mean << "±" << pm_std << " ms, put_hit "
                  << ph_mean << "±" << ph_std << " ms, get_hit " << gh_mean << "±" << gh_std
                  << " ms, get_miss " << gm_mean << "±" << gm_std << " ms, eviction " << ev_mean
                  << "±" << ev_std << " ms, memory=" << mem << " MB\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    return 0;
}
