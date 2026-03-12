// Bounded blocking queue producer-consumer benchmark.
// Uses Mutex<VecDeque<i32>> + Condvar (same semantics as C++/Java).

use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Instant;

const TOTAL_ITEMS: i32 = 100_000;
const QUEUE_CAPACITY: usize = 4096;
const CONFIGS: [(usize, usize); 6] = [(1, 1), (2, 2), (4, 4), (8, 8), (4, 1), (1, 4)];

/// Queue and done flag under one lock (matches C++ and reduces contention).
struct BoundedQueue {
    inner: Mutex<(VecDeque<i32>, bool)>,
    capacity: usize,
    not_full: Condvar,
    not_empty: Condvar,
}

impl BoundedQueue {
    fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new((VecDeque::new(), false)),
            capacity,
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    fn push(&self, value: i32) {
        let mut g = self.inner.lock().unwrap();
        while g.0.len() >= self.capacity {
            g = self.not_full.wait(g).unwrap();
        }
        g.0.push_back(value);
        drop(g); // release the lock
        self.not_empty.notify_one();
    }

    fn pop(&self) -> Option<i32> {
        let mut g = self.inner.lock().unwrap();
        while g.0.is_empty() && !g.1 {
            g = self.not_empty.wait(g).unwrap();
        }
        let v = g.0.pop_front();
        drop(g); // release the lock
        if v.is_some() {
            self.not_full.notify_one();
        }
        v
    }

    fn set_done(&self) {
        let mut g = self.inner.lock().unwrap();
        g.1 = true;
        self.not_empty.notify_all();
    }
}

fn run_one(num_producers: usize, num_consumers: usize, total_items: i32, capacity: usize) -> f64 {
    // Create a new bounded queue
    let queue = std::sync::Arc::new(BoundedQueue::new(capacity));
    // Create a new atomic integer to track the number of items consumed
    let consumed = std::sync::Arc::new(AtomicI32::new(0));
    // Create a new target to track the number of items to consume
    let target = total_items;

    let start = Instant::now();

    let per_producer = total_items as usize / num_producers;
    let mut producers = Vec::with_capacity(num_producers);
    for p in 0..num_producers {
        let begin = p * per_producer;
        let end = if p == num_producers - 1 {
            total_items as usize
        } else {
            (p + 1) * per_producer
        };
        let q = std::sync::Arc::clone(&queue);
        producers.push(thread::spawn(move || {
            for i in begin..end {
                q.push(i as i32);
            }
        }));
    }

    let mut consumers = Vec::with_capacity(num_consumers);
    for _ in 0..num_consumers {
        let q = std::sync::Arc::clone(&queue);
        let c = std::sync::Arc::clone(&consumed);
        consumers.push(thread::spawn(move || {
            while c.load(Ordering::Relaxed) < target {
                if q.pop().is_some() {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    for t in producers {
        t.join().unwrap();
    }
    queue.set_done();
    for t in consumers {
        t.join().unwrap();
    }

    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_concurrency.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(
        file,
        "num_producers,num_consumers,capacity,total_items,elapsed_mean_ms,elapsed_std_ms,throughput_per_sec_mean,memory_mb"
    )
    .expect("write header");

    for (p, c) in CONFIGS {
        let mut samples = Vec::with_capacity(NUM_RUNS as usize);
        for _ in 0..NUM_RUNS {
            samples.push(run_one(p, c, TOTAL_ITEMS, QUEUE_CAPACITY));
        }
        let (e_mean, e_std) = mean_std(&samples);
        let throughput = TOTAL_ITEMS as f64 / (e_mean / 1000.0);
        let mem = memory_mb();
        writeln!(
            file,
            "{},{},{},{},{:.6},{:.6},{:.6},{:.4}",
            p, c, QUEUE_CAPACITY, TOTAL_ITEMS, e_mean, e_std, throughput, mem
        )
        .expect("write row");
        println!(
            "P={} C={}: elapsed {:.6} ± {:.6} ms, throughput {:.0}/s, memory {:.4} MB",
            p, c, e_mean, e_std, throughput, mem
        );
    }
    println!("Wrote {}", csv_path);
}
