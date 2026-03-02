#include "bench_common.hpp"
#include "heap.hpp"
#include <chrono>
#include <random>

int main() {
    std::string out_dir = get_results_dir();
    fs::create_directories(out_dir);

    std::random_device rd;
    std::mt19937 g(rd());

    std::string csv_path = out_dir + "/cpp_heap.csv";
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

        {  // warm-up: insert = push, get = peek
            heap::Heap h;
            for (int k : keys) h.push(k);
            while (h.size() > 0) do_not_optimize(h.peek()), h.pop();
        }

        std::vector<double> insert_ms(NUM_RUNS), get_ms(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; run++) {
            std::shuffle(keys.begin(), keys.end(), g);
            heap::Heap h;
            auto start = std::chrono::high_resolution_clock::now();
            for (int k : keys) h.push(k);
            insert_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
            start = std::chrono::high_resolution_clock::now();
            for (size_t i = 0; i < static_cast<size_t>(n); i++) do_not_optimize(h.peek());
            get_ms[run] = std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start).count();
        }
        double i_mean, i_std, g_mean, g_std;
        mean_std(insert_ms, i_mean, i_std);
        mean_std(get_ms, g_mean, g_std);
        double mem = memory_mb();
        file << n << "," << i_mean << "," << i_std << "," << g_mean << "," << g_std << "," << std::setprecision(4) << mem << "\n";
        file << std::setprecision(6);
        std::cout << "N=" << n << ": Insert " << i_mean << " ± " << i_std << " ms, Get(peek) " << g_mean << " ± " << g_std << " ms, memory=" << mem << " MB\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    return 0;
}
