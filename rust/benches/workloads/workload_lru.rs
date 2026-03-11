use hashlink::LinkedHashMap;
use lru::LruCache;
use std::collections::HashMap;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::num::NonZeroUsize;
use std::time::Instant;

use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};

// Benchmark: compare HashMap, NaiveLRU (Vec + HashMap<Key, index> for O(1) recency update), LruCache and LinkedHashMap on the same values
// across four ops (put, get, mostly_get, balanced) and memory at scales 1k–1M, full, high locality and eviction key space. Writes CSV to RESULTS_DIR.
// NaiveLRU uses Vec + HashMap<Key, index> for O(1) recency update (move to back via swap).

/// Key-space scenario: determines key_space and capacity from n.
#[derive(Clone, Copy)]
enum KeySpaceScenario {
    /// key_space=n, capacity=n — no eviction, all keys fit
    Full,
    /// key_space=n/10, capacity=key_space — high locality, repeated keys
    HighLocality,
    /// key_space=n, capacity=n/10 — working set > cache, eviction stress
    Eviction,
}

impl KeySpaceScenario {
    fn key_space_and_capacity(self, n: usize) -> (usize, usize) {
        match self {
            KeySpaceScenario::Full => (n, n.max(16)),
            KeySpaceScenario::HighLocality => {
                let ks = (n / 10).max(1);
                (ks, ks.max(16))
            }
            KeySpaceScenario::Eviction => (n, (n / 10).max(16)),
        }
    }
    fn as_str(self) -> &'static str {
        match self {
            KeySpaceScenario::Full => "full",
            KeySpaceScenario::HighLocality => "high_locality",
            KeySpaceScenario::Eviction => "eviction",
        }
    }
}

const SCENARIOS: [KeySpaceScenario; 3] = [
    KeySpaceScenario::Full,
    KeySpaceScenario::HighLocality,
    KeySpaceScenario::Eviction,
];

fn generate_requests(n: usize, key_space: usize) -> Vec<i32> {
    let mut reqs = Vec::with_capacity(n);
    for i in 0..n {
        reqs.push((i % key_space) as i32);
    }
    reqs
}

// ---- PUT (insert) ----

fn hashmap_put(reqs: &[i32]) -> HashMap<i32, i32> {
    let mut map = HashMap::with_capacity(reqs.len());
    for &k in reqs {
        map.insert(k, k);
    }
    map
}

fn naive_lru_put(
    reqs: &[i32],
    capacity: usize,
) -> (HashMap<i32, i32>, Vec<i32>, HashMap<i32, usize>) {
    let mut map = HashMap::with_capacity(capacity);
    let mut order = Vec::with_capacity(capacity);
    let mut index = HashMap::with_capacity(capacity);

    for &k in reqs {
        if map.contains_key(&k) {
            continue;
        }
        if map.len() == capacity {
            let old = order[0];
            let last_idx = order.len() - 1;
            order.swap(0, last_idx);
            let moved = order[0];
            index.insert(moved, 0);
            order.pop();
            map.remove(&old);
            index.remove(&old);
        }
        let pos = order.len();
        order.push(k);
        map.insert(k, k);
        index.insert(k, pos);
    }

    (map, order, index)
}

fn lru_put(reqs: &[i32], capacity: usize) -> LruCache<i32, i32> {
    let mut cache = LruCache::new(NonZeroUsize::new(capacity).unwrap());

    for &k in reqs {
        cache.put(k, k);
    }

    cache
}

fn linked_put(keys: &[i32], capacity: usize) -> LinkedHashMap<i32, i32> {
    let mut map = LinkedHashMap::new();
    for &k in keys {
        map.insert(k, k);
        if map.len() > capacity {
            map.pop_front();
        }
    }
    map
}

// ---- GET ----

fn hashmap_get(map: &HashMap<i32, i32>, reqs: &[i32]) -> i32 {
    let mut sum = 0;

    for &k in reqs {
        if let Some(v) = map.get(&k) {
            sum += *v;
        }
    }

    sum
}

// update recency on get in O(1) via swap-to-back (fair comparison with LruCache / LinkedHashMap)
fn naive_lru_get(
    map: &HashMap<i32, i32>,
    order: &mut [i32],
    index: &mut HashMap<i32, usize>,
    reqs: &[i32],
) -> i32 {
    let mut sum = 0;
    for &k in reqs {
        if let Some(&v) = map.get(&k) {
            sum += v;
            let j = order.len() - 1;
            let i = *index.get(&k).unwrap();
            if i != j {
                let replaced = order[j];
                order.swap(i, j);
                index.insert(replaced, i);
                index.insert(k, j);
            }
        }
    }
    sum
}

// has update recency
fn lru_get(cache: &mut LruCache<i32, i32>, reqs: &[i32]) -> i32 {
    let mut sum = 0;

    for &k in reqs {
        if let Some(v) = cache.get(&k) {
            sum += *v;
        }
    }

    sum
}

// update recency on get via to_back (fair comparison with LruCache / NaiveLRU)
fn linked_get(map: &mut LinkedHashMap<i32, i32>, keys: &[i32]) -> f64 {
    let mut hits = 0;

    for &k in keys {
        if map.contains_key(&k) {
            hits += 1;
            map.to_back(&k);
        }
    }

    hits as f64
}

// ---- MOSTLY GET 90% GET, 10% PUT ----

fn hashmap_mostly_get(keys: &[i32]) -> f64 {
    let mut map = HashMap::new();
    let mut hits = 0;

    for (i, &k) in keys.iter().enumerate() {
        if i % 10 == 0 {
            map.insert(k, k);
        } else if map.contains_key(&k) {
            hits += 1;
        }
    }

    hits as f64
}

// Naive LRU (HashMap + Vec + index): 90% get / 10% put, O(1) recency on get
fn naive_lru_mostly_get(keys: &[i32], capacity: usize) -> f64 {
    let mut map = HashMap::with_capacity(capacity);
    let mut order = Vec::with_capacity(capacity);
    let mut index = HashMap::with_capacity(capacity);
    let mut hits = 0;
    for (i, &k) in keys.iter().enumerate() {
        if i % 10 == 0 {
            if !map.contains_key(&k) {
                if map.len() == capacity {
                    let old = order[0];
                    let last_idx = order.len() - 1;
                    order.swap(0, last_idx);
                    let moved = order[0];
                    index.insert(moved, 0);
                    order.pop();
                    map.remove(&old);
                    index.remove(&old);
                }
                let pos = order.len();
                order.push(k);
                map.insert(k, k);
                index.insert(k, pos);
            }
        } else if map.contains_key(&k) {
            hits += 1;
            let j = order.len() - 1;
            let i = *index.get(&k).unwrap();
            if i != j {
                let replaced = order[j];
                order.swap(i, j);
                index.insert(replaced, i);
                index.insert(k, j);
            }
        }
    }
    hits as f64
}

fn lru_mostly_get(keys: &[i32], capacity: usize) -> f64 {
    let mut cache = LruCache::new(NonZeroUsize::new(capacity).unwrap());
    let mut hits = 0;

    for (i, &k) in keys.iter().enumerate() {
        if i % 10 == 0 {
            cache.put(k, k);
        } else if cache.get(&k).is_some() {
            hits += 1;
        }
    }

    hits as f64
}

fn linked_mostly_get(keys: &[i32], cap: usize) -> f64 {
    let mut map = LinkedHashMap::new();
    let mut hits = 0;

    for (i, k) in keys.iter().enumerate() {
        if i % 10 == 0 {
            map.insert(*k, *k);
            if map.len() > cap {
                map.pop_front();
            }
        } else if map.contains_key(k) {
            hits += 1;
            map.to_back(k);
        }
    }

    hits as f64
}

// ---- Balanced 50% GET, 50% PUT ----

fn hashmap_balanced(keys: &[i32]) -> (usize, f64) {
    let mut map = HashMap::new();
    let mut hits = 0;
    for (i, &k) in keys.iter().enumerate() {
        if i % 2 == 0 {
            if map.contains_key(&k) {
                hits += 1;
            }
        } else {
            map.insert(k, k);
        }
    }

    (hits, map.len() as f64)
}

fn lru_balanced(keys: &[i32], capacity: usize) -> (usize, f64) {
    let mut cache = LruCache::new(NonZeroUsize::new(capacity).unwrap());
    let mut hits = 0;

    for (i, &k) in keys.iter().enumerate() {
        if i % 2 == 0 {
            if cache.get(&k).is_some() {
                hits += 1;
            }
        } else {
            cache.put(k, k);
        }
    }

    (hits, cache.len() as f64)
}

// Naive LRU: 50% get / 50% put, O(1) recency on put (move to back)
fn naive_lru_balanced(keys: &[i32], capacity: usize) -> (usize, f64) {
    let mut map = HashMap::with_capacity(capacity);
    let mut order = Vec::with_capacity(capacity);
    let mut index = HashMap::with_capacity(capacity);
    let mut hits = 0;
    for (i, &k) in keys.iter().enumerate() {
        if i % 2 == 0 {
            if map.contains_key(&k) {
                hits += 1;
            }
        } else {
            let is_new = !map.contains_key(&k);
            if is_new && map.len() == capacity {
                let old = order[0];
                let last_idx = order.len() - 1;
                order.swap(0, last_idx);
                let moved = order[0];
                index.insert(moved, 0);
                order.pop();
                map.remove(&old);
                index.remove(&old);
            }
            map.insert(k, k);
            if is_new {
                let pos = order.len();
                order.push(k);
                index.insert(k, pos);
            } else {
                let j = order.len() - 1;
                let i = *index.get(&k).unwrap();
                if i != j {
                    let replaced = order[j];
                    order.swap(i, j);
                    index.insert(replaced, i);
                    index.insert(k, j);
                }
            }
        }
    }
    (hits, map.len() as f64)
}

fn linked_balanced(keys: &[i32], cap: usize) -> (usize, f64) {
    let mut map = LinkedHashMap::new();
    let mut hits = 0;
    for (i, k) in keys.iter().enumerate() {
        if i % 2 == 0 {
            if map.contains_key(k) {
                hits += 1;
                map.to_back(k);
            }
        } else {
            map.insert(*k, *k);
            if map.len() > cap {
                map.pop_front();
            }
        }
    }

    (hits, map.len() as f64)
}

fn sample_four_ops<F1, F2, F3, F4>(
    put_fn: F1,
    get_fn: F2,
    mostly_get_fn: F3,
    balanced_fn: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn(),
    F4: Fn(),
{
    let mut put_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut get_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut mostly_get_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut balanced_s = Vec::with_capacity(NUM_RUNS as usize);

    for _ in 0..NUM_RUNS {
        let t = Instant::now();
        put_fn();
        black_box(());
        put_s.push(t.elapsed().as_secs_f64() * 1000.0);

        get_s.push(get_fn());

        let t = Instant::now();
        mostly_get_fn();
        black_box(());
        mostly_get_s.push(t.elapsed().as_secs_f64() * 1000.0);

        let t = Instant::now();
        balanced_fn();
        black_box(());
        balanced_s.push(t.elapsed().as_secs_f64() * 1000.0);
    }

    (put_s, get_s, mostly_get_s, balanced_s)
}

fn warm_up_four_ops<F1, F2, F3, F4>(put_fn: F1, get_fn: F2, mostly_get_fn: F3, balanced_fn: F4)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn(),
    F4: Fn(),
{
    put_fn();
    let _ = get_fn();
    mostly_get_fn();
    balanced_fn();
}

fn run_structure_ops<F1, F2, F3, F4>(
    put: F1,
    get: F2,
    mostly_get: F3,
    balanced: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn(),
    F4: Fn(),
{
    warm_up_four_ops(&put, &get, &mostly_get, &balanced);
    sample_four_ops(put, get, mostly_get, balanced)
}

struct OpMetrics {
    put_m: f64,
    put_s: f64,
    get_m: f64,
    get_s: f64,
    mostly_get_m: f64,
    mostly_get_s: f64,
    balanced_m: f64,
    balanced_s: f64,
}

fn op_metrics_from_samples(
    put_s: &[f64],
    get_s: &[f64],
    mostly_get_s: &[f64],
    balanced_s: &[f64],
) -> OpMetrics {
    let (put_m, put_s_val) = mean_std(put_s);
    let (get_m, get_s_val) = mean_std(get_s);
    let (mostly_get_m, mostly_get_s_val) = mean_std(mostly_get_s);
    let (balanced_m, balanced_s_val) = mean_std(balanced_s);
    OpMetrics {
        put_m,
        put_s: put_s_val,
        get_m,
        get_s: get_s_val,
        mostly_get_m,
        mostly_get_s: mostly_get_s_val,
        balanced_m,
        balanced_s: balanced_s_val,
    }
}

struct RowMetrics {
    n: usize,
    scenario: &'static str,
    hashmap: OpMetrics,
    naive_lru: OpMetrics,
    lru: OpMetrics,
    linked: OpMetrics,
    hashmap_mem: f64,
    naive_lru_mem: f64,
    lru_mem: f64,
    linked_mem: f64,
}

fn compute_row(
    hashmap_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    naive_lru_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    lru_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    linked_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    mem: (f64, f64, f64, f64),
    n: usize,
    scenario: &'static str,
) -> RowMetrics {
    RowMetrics {
        n,
        scenario,
        hashmap: op_metrics_from_samples(
            &hashmap_samples.0,
            &hashmap_samples.1,
            &hashmap_samples.2,
            &hashmap_samples.3,
        ),
        naive_lru: op_metrics_from_samples(
            &naive_lru_samples.0,
            &naive_lru_samples.1,
            &naive_lru_samples.2,
            &naive_lru_samples.3,
        ),
        lru: op_metrics_from_samples(
            &lru_samples.0,
            &lru_samples.1,
            &lru_samples.2,
            &lru_samples.3,
        ),
        linked: op_metrics_from_samples(
            &linked_samples.0,
            &linked_samples.1,
            &linked_samples.2,
            &linked_samples.3,
        ),
        hashmap_mem: mem.0,
        naive_lru_mem: mem.1,
        lru_mem: mem.2,
        linked_mem: mem.3,
    }
}

fn write_csv_row(file: &mut File, row: &RowMetrics) {
    let h = &row.hashmap;
    let n = &row.naive_lru;
    let l = &row.lru;
    let ld = &row.linked;
    writeln!(
        file,
        "{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.4},{:.4},{:.4},{:.4}",
        row.n,
        row.scenario,
        h.put_m,
        h.put_s,
        h.get_m,
        h.get_s,
        h.mostly_get_m,
        h.mostly_get_s,
        h.balanced_m,
        h.balanced_s,
        n.put_m,
        n.put_s,
        n.get_m,
        n.get_s,
        n.mostly_get_m,
        n.mostly_get_s,
        n.balanced_m,
        n.balanced_s,
        l.put_m,
        l.put_s,
        l.get_m,
        l.get_s,
        l.mostly_get_m,
        l.mostly_get_s,
        l.balanced_m,
        l.balanced_s,
        ld.put_m,
        ld.put_s,
        ld.get_m,
        ld.get_s,
        ld.mostly_get_m,
        ld.mostly_get_s,
        ld.balanced_m,
        ld.balanced_s,
        row.hashmap_mem,
        row.naive_lru_mem,
        row.lru_mem,
        row.linked_mem,
    )
    .expect("write row");
}

fn print_row(row: &RowMetrics) {
    let h = &row.hashmap;
    let n = &row.naive_lru;
    let l = &row.lru;
    let ld = &row.linked;
    println!(
        "N={} {}: put HashMap={:.4} NaiveLRU={:.4} LruCache={:.4} LinkedHashMap={:.4} ms",
        row.n, row.scenario, h.put_m, n.put_m, l.put_m, ld.put_m
    );
    println!(
        "      get HashMap={:.4} NaiveLRU={:.4} LruCache={:.4} LinkedHashMap={:.4} ms",
        h.get_m, n.get_m, l.get_m, ld.get_m
    );
    println!(
        "      mostly_get HashMap={:.4} NaiveLRU={:.4} LruCache={:.4} LinkedHashMap={:.4} ms",
        h.mostly_get_m, n.mostly_get_m, l.mostly_get_m, ld.mostly_get_m
    );
    println!(
        "      balanced HashMap={:.4} NaiveLRU={:.4} LruCache={:.4} LinkedHashMap={:.4} ms",
        h.balanced_m, n.balanced_m, l.balanced_m, ld.balanced_m
    );
    println!(
        "      memory HashMap={:.4} NaiveLRU={:.4} LruCache={:.4} LinkedHashMap={:.4} MB",
        row.hashmap_mem, row.naive_lru_mem, row.lru_mem, row.linked_mem
    );
}

fn measure_memory<F, T>(build: F) -> f64
where
    F: FnOnce() -> T,
{
    let x = build();
    black_box(x);
    memory_mb()
}

fn run() {
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_workload_lru.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(
        file,
        "N,scenario,hashmap_put_mean_ms,hashmap_put_std_ms,hashmap_get_mean_ms,hashmap_get_std_ms,hashmap_mostly_get_mean_ms,hashmap_mostly_get_std_ms,hashmap_balanced_mean_ms,hashmap_balanced_std_ms,\
naive_lru_put_mean_ms,naive_lru_put_std_ms,naive_lru_get_mean_ms,naive_lru_get_std_ms,naive_lru_mostly_get_mean_ms,naive_lru_mostly_get_std_ms,naive_lru_balanced_mean_ms,naive_lru_balanced_std_ms,\
lru_put_mean_ms,lru_put_std_ms,lru_get_mean_ms,lru_get_std_ms,lru_mostly_get_mean_ms,lru_mostly_get_std_ms,lru_balanced_mean_ms,lru_balanced_std_ms,\
linked_put_mean_ms,linked_put_std_ms,linked_get_mean_ms,linked_get_std_ms,linked_mostly_get_mean_ms,linked_mostly_get_std_ms,linked_balanced_mean_ms,linked_balanced_std_ms,\
hashmap_memory_mb,naive_lru_memory_mb,lru_memory_mb,linked_memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        for &scenario in &SCENARIOS {
            let (key_space, capacity) = scenario.key_space_and_capacity(n);
            let reqs = generate_requests(n, key_space);

            let hashmap = hashmap_put(&reqs);
            let (naive_map, naive_order, naive_index) = naive_lru_put(&reqs, capacity);
            let lru_cache = lru_put(&reqs, capacity);
            let linked_map = linked_put(&reqs, capacity);

            let hashmap_samples = run_structure_ops(
                || {
                    hashmap_put(&reqs);
                },
                || {
                    let t = Instant::now();
                    let _ = black_box(hashmap_get(&hashmap, &reqs));
                    t.elapsed().as_secs_f64() * 1000.0
                },
                || {
                    let _ = black_box(hashmap_mostly_get(&reqs));
                },
                || {
                    let (hits, len) = hashmap_balanced(&reqs);
                    black_box((hits, len));
                },
            );

            let naive_lru_samples = run_structure_ops(
                || {
                    naive_lru_put(&reqs, capacity);
                },
                || {
                    let mut order = naive_order.clone();
                    let mut index = naive_index.clone();
                    let t = Instant::now();
                    let _ = black_box(naive_lru_get(&naive_map, &mut order, &mut index, &reqs));
                    t.elapsed().as_secs_f64() * 1000.0
                },
                || {
                    let _ = black_box(naive_lru_mostly_get(&reqs, capacity));
                },
                || {
                    let (hits, len) = naive_lru_balanced(&reqs, capacity);
                    black_box((hits, len));
                },
            );

            let lru_samples = run_structure_ops(
                || {
                    lru_put(&reqs, capacity);
                },
                || {
                    let mut cache = lru_cache.clone();
                    let t = Instant::now();
                    let _ = black_box(lru_get(&mut cache, &reqs));
                    t.elapsed().as_secs_f64() * 1000.0
                },
                || {
                    let _ = black_box(lru_mostly_get(&reqs, capacity));
                },
                || {
                    let (hits, len) = lru_balanced(&reqs, capacity);
                    black_box((hits, len));
                },
            );

            let linked_samples = run_structure_ops(
                || {
                    linked_put(&reqs, capacity);
                },
                || {
                    let mut map = linked_map.clone();
                    let t = Instant::now();
                    let _ = black_box(linked_get(&mut map, &reqs));
                    t.elapsed().as_secs_f64() * 1000.0
                },
                || {
                    let _ = black_box(linked_mostly_get(&reqs, capacity));
                },
                || {
                    let (hits, len) = linked_balanced(&reqs, capacity);
                    black_box((hits, len));
                },
            );

            let hashmap_mem = measure_memory(|| hashmap_put(&reqs));
            let naive_lru_mem = measure_memory(|| naive_lru_put(&reqs, capacity));
            let lru_mem = measure_memory(|| lru_put(&reqs, capacity));
            let linked_mem = measure_memory(|| linked_put(&reqs, capacity));

            let row = compute_row(
                hashmap_samples,
                naive_lru_samples,
                lru_samples,
                linked_samples,
                (hashmap_mem, naive_lru_mem, lru_mem, linked_mem),
                n,
                scenario.as_str(),
            );

            write_csv_row(&mut file, &row);
            print_row(&row);
        }
    }

    println!("Wrote {}", csv_path);
}

fn main() {
    run();
}
