// LRU cache benchmark: put (insert) and get. Same schema as hashmap.
use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};
use polyglot_benchmarks::lru_cache::LRUCache;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_lru_cache.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb").expect("write header");

    for &n in &SCALES {
        let capacity = n.max(16);
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);

        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut cache = LRUCache::new(capacity);
            for &k in &keys { cache.put(k, k); }
            for &k in &keys { let _ = cache.get(k); }
        }

        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut cache = LRUCache::new(capacity);
            let start = Instant::now();
            for &k in &keys { cache.put(k, k); }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            for &k in &keys { let _ = cache.get(k); }
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        let mem = memory_mb();
        writeln!(file, "{},{:.6},{:.6},{:.6},{:.6},{:.4}", n, i_mean, i_std, g_mean, g_std, mem).expect("write row");
        println!("N={}: Insert {:.6} ± {:.6} ms, Get {:.6} ± {:.6} ms, memory={:.4} MB", n, i_mean, i_std, g_mean, g_std, mem);
    }
    println!("Wrote {}", csv_path);
}
