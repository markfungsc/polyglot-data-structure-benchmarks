// Heap benchmark: push (insert) and peek (get). Same schema as hashmap.
use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};
use polyglot_benchmarks::heap::Heap;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_heap.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb").expect("write header");

    for &n in &SCALES {
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);

        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut h = Heap::new();
            for &k in &keys { h.push(k); }
            for _ in 0..n { let _ = h.peek(); h.pop(); }
        }

        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut h = Heap::new();
            let start = Instant::now();
            for &k in &keys { h.push(k); }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            for _ in 0..n { let _ = h.peek(); }
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        let mem = memory_mb();
        writeln!(file, "{},{:.6},{:.6},{:.6},{:.6},{:.4}", n, i_mean, i_std, g_mean, g_std, mem).expect("write row");
        println!("N={}: Insert {:.6} ± {:.6} ms, Get(peek) {:.6} ± {:.6} ms, memory={:.4} MB", n, i_mean, i_std, g_mean, g_std, mem);
    }
    println!("Wrote {}", csv_path);
}
