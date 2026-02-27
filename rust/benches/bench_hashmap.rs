// Scaled benchmarks, random keys, std dev, memory, low-entropy (near-collision), load-factor; CSV output
use polyglot_benchmarks::hashmap::HashMap;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

const SCALES: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];
const NUM_RUNS: u32 = 5;
const LOW_ENTROPY_CAPACITY: usize = 64; // low-entropy / near-collision: few buckets
const LOAD_FACTOR_N: usize = 100_000;
const LOAD_FACTORS: [f64; 4] = [0.25, 0.5, 0.75, 1.0];

fn memory_mb() -> f64 {
    let Ok(f) = std::fs::File::open("/proc/self/status") else { return 0.0 };
    let r = BufReader::new(f);
    for line in r.lines().flatten() {
        if line.starts_with("VmRSS:") {
            let num: String = line.split_whitespace().nth(1).unwrap_or("0").into();
            let kb: f64 = num.parse().unwrap_or(0.0);
            return kb / 1024.0;
        }
    }
    0.0
}

fn mean_std(samples: &[f64]) -> (f64, f64) {
    let n = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let variance = if samples.len() >= 2 {
        samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
    } else {
        0.0
    };
    (mean, variance.sqrt())
}

fn main() {
    let mut rng = rand::thread_rng();
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results/raw");

    // ---- Main scenario ----
    let csv_path = format!("{}/rust_hashmap.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb").expect("write header");

    for &n in &SCALES {
        let capacity = n.max(16);
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);

        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut map = HashMap::new(capacity);
            for &k in &keys { map.insert(k, k); }
            for &k in &keys { let _ = map.get(&k); }
        }

        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let start = Instant::now();
            let mut map = HashMap::new(capacity);
            for &k in &keys { map.insert(k, k); }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            for &k in &keys { let _ = map.get(&k); }
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        let mem = memory_mb();
        writeln!(file, "{},{:.6},{:.6},{:.6},{:.6},{:.4}", n, i_mean, i_std, g_mean, g_std, mem).expect("write row");
        println!("N={}: Insert {:.6} ± {:.6} ms, Get {:.6} ± {:.6} ms, memory={:.4} MB", n, i_mean, i_std, g_mean, g_std, mem);
    }
    println!("Wrote {}", csv_path);

    // ---- Low-entropy / near-collision ----
    let csv_path = format!("{}/rust_hashmap_low_entropy.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms").expect("write header");

    for &n in &SCALES {
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);
        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut map = HashMap::new(LOW_ENTROPY_CAPACITY);
            for &k in &keys { map.insert(k, k); }
            for &k in &keys { let _ = map.get(&k); }
        }
        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let start = Instant::now();
            let mut map = HashMap::new(LOW_ENTROPY_CAPACITY);
            for &k in &keys { map.insert(k, k); }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            for &k in &keys { let _ = map.get(&k); }
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        writeln!(file, "{},{:.6},{:.6},{:.6},{:.6}", n, i_mean, i_std, g_mean, g_std).expect("write row");
        println!("Low-entropy N={}: Insert {:.6} ± {:.6} ms, Get {:.6} ± {:.6} ms", n, i_mean, i_std, g_mean, g_std);
    }
    println!("Wrote {}", csv_path);

    // ---- Load factor sensitivity ----
    let csv_path = format!("{}/rust_hashmap_loadfactor.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "load_factor,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms").expect("write header");
    let n = LOAD_FACTOR_N;
    for &lf in &LOAD_FACTORS {
        let capacity = (n as f64 / lf).ceil() as usize;
        let capacity = capacity.max(16);
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);
        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut map = HashMap::new(capacity);
            for &k in &keys { map.insert(k, k); }
            for &k in &keys { let _ = map.get(&k); }
        }
        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let start = Instant::now();
            let mut map = HashMap::new(capacity);
            for &k in &keys { map.insert(k, k); }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            for &k in &keys { let _ = map.get(&k); }
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        writeln!(file, "{},{:.6},{:.6},{:.6},{:.6}", lf, i_mean, i_std, g_mean, g_std).expect("write row");
        println!("LoadFactor={}: Insert {:.6} ± {:.6} ms, Get {:.6} ± {:.6} ms", lf, i_mean, i_std, g_mean, g_std);
    }
    println!("Wrote {}", csv_path);
}
