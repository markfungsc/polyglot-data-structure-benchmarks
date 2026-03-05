// Heap benchmark: insert, peek, and pop. Same schema as other structure benchmarks.
use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};
use polyglot_benchmarks::heap::MinHeap;
use rand::prelude::*;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_heap.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(
        file,
        "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut pop_samples = Vec::with_capacity(NUM_RUNS as usize);

        // Warm-up: insert, peek, pop
        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut h = MinHeap::new();
            for &k in &keys {
                h.insert(k);
            }
            while h.size() > 0 {
                while let Some(value) = h.pop() {
                    black_box(value);
                }
            }
        }

        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut h = MinHeap::new();

            let start = Instant::now();
            for &k in &keys {
                h.insert(k); // O(log n) per insert
            }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            let start = Instant::now();
            while h.size() > 0 {
                if let Some(value) = h.pop() {
                    black_box(value);
                }
            }
            pop_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (i_mean, i_std) = mean_std(&insert_samples);
        let (o_mean, o_std) = mean_std(&pop_samples);
        let mem = memory_mb();
        writeln!(
            file,
            "{},{:.6},{:.6},{:.6},{:.6},{:.4}",
            n, i_mean, i_std, o_mean, o_std, mem
        )
        .expect("write row");
        println!(
            "N={}: Insert {:.6} ± {:.6} ms, Pop {:.6} ± {:.6} ms, memory={:.4} MB",
            n, i_mean, i_std, o_mean, o_std, mem
        );
    }
    println!("Wrote {}", csv_path);
}
