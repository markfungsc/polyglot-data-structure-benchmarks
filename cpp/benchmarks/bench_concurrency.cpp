#include <atomic>
#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>
#include <vector>

#include "bench_common.hpp"

// Bounded blocking queue: mutex + queue + two condition variables.
class BoundedQueue {
   public:
    explicit BoundedQueue(size_t cap) : capacity_(cap) {}

    void push(int value) {
        std::unique_lock<std::mutex> lock(mutex_);
        // Wait until the queue is not full
        not_full_.wait(lock, [this] { return queue_.size() < capacity_; });
        queue_.push(value);
        not_empty_.notify_one();
    }

    bool pop(int& value) {
        std::unique_lock<std::mutex> lock(mutex_);
        // Wait until the queue is not empty or the done flag is set
        not_empty_.wait(lock, [this] { return !queue_.empty() || done_; });
        if (queue_.empty()) return false;
        value = queue_.front();
        queue_.pop();
        not_full_.notify_one();
        return true;
    }

    void set_done() {
        std::lock_guard<std::mutex> lock(mutex_);
        done_ = true;
        not_empty_.notify_all();
    }

   private:
    std::mutex mutex_;
    std::condition_variable not_full_;
    std::condition_variable not_empty_;
    std::queue<int> queue_;
    size_t capacity_;
    bool done_ = false;
};

static constexpr int TOTAL_ITEMS = 100000;
static constexpr int QUEUE_CAPACITY = 4096;
static constexpr int CONFIGS[][2] = {{1, 1}, {2, 2}, {4, 4}, {8, 8}, {4, 1}, {1, 4}};
static constexpr size_t NUM_CONFIGS = sizeof(CONFIGS) / sizeof(CONFIGS[0]);

void run_one(int num_producers, int num_consumers, int total_items, int capacity,
             double& elapsed_ms) {
    BoundedQueue queue(static_cast<size_t>(capacity));
    std::atomic<int> consumed{0};
    const int target = total_items;

    auto start = std::chrono::high_resolution_clock::now();

    std::vector<std::thread> producers;
    int per_producer = total_items / num_producers;
    for (int p = 0; p < num_producers; ++p) {
        int begin = p * per_producer;
        int end = (p == num_producers - 1) ? total_items : (p + 1) * per_producer;
        producers.emplace_back([&queue, begin, end]() {
            for (int i = begin; i < end; ++i) queue.push(i);
        });
    }

    std::vector<std::thread> consumers;
    for (int c = 0; c < num_consumers; ++c) {
        consumers.emplace_back([&queue, &consumed, target]() {
            int v;
            while (consumed.load(std::memory_order_relaxed) < target) {
                if (queue.pop(v)) {
                    consumed.fetch_add(1, std::memory_order_relaxed);
                }
            }
        });
    }

    for (auto& t : producers) t.join();
    queue.set_done();
    for (auto& t : consumers) t.join();

    elapsed_ms =
        std::chrono::duration<double, std::milli>(std::chrono::high_resolution_clock::now() - start)
            .count();
}

int main() {
    std::string out_dir = get_results_dir();
    fs::create_directories(out_dir);

    std::string csv_path = out_dir + "/cpp_concurrency.csv";
    std::ofstream file(csv_path);
    if (!file.is_open()) {
        std::cerr << "Failed to open " << csv_path << "\n";
        return 1;
    }
    file << "num_producers,num_consumers,capacity,total_items,elapsed_mean_ms,"
            "elapsed_std_ms,throughput_per_sec_mean,memory_mb\n";
    file << std::fixed << std::setprecision(6);

    for (size_t cfg = 0; cfg < NUM_CONFIGS; ++cfg) {
        int P = CONFIGS[cfg][0];
        int C = CONFIGS[cfg][1];

        std::vector<double> samples(NUM_RUNS);
        for (int run = 0; run < NUM_RUNS; ++run) {
            run_one(P, C, TOTAL_ITEMS, QUEUE_CAPACITY, samples[run]);
        }

        double e_mean, e_std;
        mean_std(samples, e_mean, e_std);
        double throughput = TOTAL_ITEMS / (e_mean / 1000.0);
        double mem = memory_mb();

        file << P << "," << C << "," << QUEUE_CAPACITY << "," << TOTAL_ITEMS << "," << e_mean << ","
             << e_std << "," << throughput << "," << std::setprecision(4) << mem << "\n";
        file << std::setprecision(6);

        std::cout << "P=" << P << " C=" << C << ": elapsed " << e_mean << " ± " << e_std
                  << " ms, throughput " << (static_cast<long>(throughput)) << "/s, memory " << mem
                  << " MB\n";
    }

    std::cout << "Wrote " << csv_path << "\n";
    return 0;
}
