#include <chrono>
#include <random>

#include "bench_common.hpp"
#include "linked_list.hpp"

int main() {
    std::string out_dir = get_results_dir();
    fs::create_directories(out_dir);

    std::random_device rd;
    std::mt19937 g(rd());

    std::string csv_path = out_dir + "/cpp_linked_list.csv";
    std::ofstream file(csv_path);
    if (!file.is_open()) {
        std::cerr << "Failed to open " << csv_path << "\n";
        return 1;
    }
    file << "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,delete_mean_ms,delete_std_ms,"
            "memory_mb\n";
    file << std::fixed << std::setprecision(6);
    int sum = 0;
    std::function<void(int)> addToSum = [&sum](int value) { sum += value; };
    for (int n : SCALES) {
        std::vector<int> keys(n);
        std::iota(keys.begin(), keys.end(), 0);
        std::shuffle(keys.begin(), keys.end(), g);

        {  // warm-up
            linked_list::LinkedList list;
            for (int k : keys) list.pushBack(k);
            list.traverse(addToSum);
            list.deleteNode(list.size() - 1);
        }

        std::vector<double> insert_ms(NUM_RUNS), get_ms(NUM_RUNS), delete_ms(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; run++) {
            std::shuffle(keys.begin(), keys.end(), g);
            linked_list::LinkedList list;
            // insert
            auto start = std::chrono::high_resolution_clock::now();
            for (int k : keys) list.pushBack(k);
            insert_ms[run] = std::chrono::duration<double, std::milli>(
                                 std::chrono::high_resolution_clock::now() - start)
                                 .count();
            // get
            start = std::chrono::high_resolution_clock::now();
            list.traverse(addToSum);
            get_ms[run] = std::chrono::duration<double, std::milli>(
                              std::chrono::high_resolution_clock::now() - start)
                              .count();
            // delete
            start = std::chrono::high_resolution_clock::now();
            list.deleteNode(list.size() - 1);
            delete_ms[run] = std::chrono::duration<double, std::milli>(
                                 std::chrono::high_resolution_clock::now() - start)
                                 .count();
        }
        double i_mean, i_std, g_mean, g_std, d_mean, d_std;
        mean_std(insert_ms, i_mean, i_std);
        mean_std(get_ms, g_mean, g_std);
        mean_std(delete_ms, d_mean, d_std);
        double mem = memory_mb();
        file << n << "," << i_mean << "," << i_std << "," << g_mean << "," << g_std << "," << d_mean
             << "," << d_std << "," << std::setprecision(4) << mem << "\n";
        file << std::setprecision(6);
        std::cout << "N=" << n << ": Insert " << i_mean << " ± " << i_std << " ms, Get " << g_mean
                  << " ± " << g_std << " ms, Delete " << d_mean << " ± " << d_std
                  << " ms, memory=" << mem << " MB\n";
    }
    std::cout << "Wrote " << csv_path << "\n";
    return 0;
}
