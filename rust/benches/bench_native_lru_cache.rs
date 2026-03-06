// LRU cache benchmark: put_miss, put_hit, get_hit, get_miss, eviction.
use lru::LruCache;
use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::num::NonZero;
use std::time::Instant;

fn main() {
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_native_lru_cache.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(
        file,
        "N,put_miss_mean_ms,put_miss_std_ms,put_hit_mean_ms,put_hit_std_ms,get_hit_mean_ms,get_hit_std_ms,get_miss_mean_ms,get_miss_std_ms,eviction_mean_ms,eviction_std_ms,memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        let capacity = n.max(16);
        let mut put_miss_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut put_hit_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_hit_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_miss_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut eviction_samples = Vec::with_capacity(NUM_RUNS as usize);

        // Warm-up: build and use cache once at this scale
        {
            let keys: Vec<i32> = (0..n as i32).collect();
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            for &k in &keys {
                cache.put(k, k);
            }
            for &k in &keys {
                let _ = cache.get(&k);
            }
        }

        for _ in 0..NUM_RUNS {
            // put_miss: cache has capacity, N-1 elements; one put of a new key (no eviction)
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            let start = Instant::now();
            for i in 0..(capacity - 1) as i32 {
                cache.put(i, i);
            }
            put_miss_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // put_hit: full cache, N updates of existing keys
            let keys: Vec<i32> = (0..n as i32).collect();
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            for &k in &keys {
                cache.put(k, k);
            }
            let start = Instant::now();
            for i in 0..n {
                cache.put((i % capacity) as i32, i as i32);
            }
            put_hit_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // get_hit: full cache, N lookups of existing keys
            let keys: Vec<i32> = (0..n as i32).collect();
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            for &k in &keys {
                cache.put(k, k);
            }
            let start = Instant::now();
            for i in 0..n {
                black_box(cache.get(&((i % capacity) as i32)));
            }
            get_hit_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // get_miss: full cache, N lookups of a key not in cache
            let keys: Vec<i32> = (0..n as i32).collect();
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            for &k in &keys {
                cache.put(k, k);
            }
            let missing = n as i32; // not in 0..n
            let start = Instant::now();
            for _ in 0..n {
                black_box(cache.get(&missing));
            }
            get_miss_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // eviction: full cache, N puts of new keys so each put evicts LRU
            let keys: Vec<i32> = (0..n as i32).collect();
            let mut cache = LruCache::new(NonZero::new(capacity).unwrap());
            for &k in &keys {
                cache.put(k, k);
            }
            let start = Instant::now();
            for i in n..(2 * n) {
                cache.put(i as i32, i as i32);
            }
            eviction_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (pm_mean, pm_std) = mean_std(&put_miss_samples);
        let (ph_mean, ph_std) = mean_std(&put_hit_samples);
        let (gh_mean, gh_std) = mean_std(&get_hit_samples);
        let (gm_mean, gm_std) = mean_std(&get_miss_samples);
        let (ev_mean, ev_std) = mean_std(&eviction_samples);
        let mem = memory_mb();
        writeln!(
            file,
            "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.4}",
            n,
            pm_mean,
            pm_std,
            ph_mean,
            ph_std,
            gh_mean,
            gh_std,
            gm_mean,
            gm_std,
            ev_mean,
            ev_std,
            mem
        )
        .expect("write row");
        println!(
            "N={}: put_miss {:.6}±{:.6} ms, put_hit {:.6}±{:.6} ms, get_hit {:.6}±{:.6} ms, get_miss {:.6}±{:.6} ms, eviction {:.6}±{:.6} ms, memory={:.4} MB",
            n, pm_mean, pm_std, ph_mean, ph_std, gh_mean, gh_std, gm_mean, gm_std, ev_mean, ev_std, mem
        );
    }
    println!("Wrote {}", csv_path);
}
