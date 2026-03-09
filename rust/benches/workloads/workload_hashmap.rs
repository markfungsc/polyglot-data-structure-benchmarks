use rustc_hash::FxHasher;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};

// Benchmark: compare HashMap (with FxHasher for deterministic hashing), BTreeMap, and VecMap on the same tick data
// across four ops (update, lookup, scan, range) and memory at scales 1k–1M. Writes CSV to RESULTS_DIR.

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
type VecMap = Vec<Option<Agg>>;

#[derive(Clone, Copy)]
pub struct Tick {
    pub symbol: u32,
    pub price: f64,
    pub volume: f64,
}

#[derive(Clone, Copy, Default)]
pub struct Agg {
    pub last_price: f64,
    pub total_volume: f64,
    pub tick_count: u64,
}

fn update(agg: &mut Agg, tick: &Tick) {
    agg.last_price = tick.price;
    agg.total_volume += tick.volume;
    agg.tick_count += 1;
}

fn generate_ticks(n: usize, symbols: u32) -> Vec<Tick> {
    let mut ticks = Vec::with_capacity(n);

    for i in 0..n {
        ticks.push(Tick {
            symbol: (i as u32) % symbols,
            price: (i % 1000) as f64,
            volume: 1.0,
        });
    }

    ticks
}

fn generate_queries(n: usize, symbols: u32) -> Vec<u32> {
    let mut q = Vec::with_capacity(n);

    for i in 0..n {
        q.push((i as u32) % symbols);
    }

    q
}

fn data_to_hashmap(ticks: &[Tick]) -> FxHashMap<u32, Agg> {
    let mut map: FxHashMap<u32, Agg> = FxHashMap::default();

    for tick in ticks {
        map.entry(tick.symbol)
            .and_modify(|agg| update(agg, tick))
            .or_insert(Agg {
                last_price: tick.price,
                total_volume: tick.volume,
                tick_count: 1,
            });
    }

    map
}

fn data_to_btreemap(ticks: &[Tick]) -> BTreeMap<u32, Agg> {
    let mut map: BTreeMap<u32, Agg> = BTreeMap::new();

    for tick in ticks {
        map.entry(tick.symbol)
            .and_modify(|agg| update(agg, tick))
            .or_insert(Agg {
                last_price: tick.price,
                total_volume: tick.volume,
                tick_count: 1,
            });
    }

    map
}

fn data_to_vecmap(ticks: &[Tick], symbols: usize) -> VecMap {
    let mut vecmap: VecMap = vec![None; symbols];
    for tick in ticks {
        let agg = vecmap[tick.symbol as usize].get_or_insert(Agg::default());
        update(agg, tick);
    }
    vecmap
}

// ---- Update Agg (update the agg for a given tick) ----

fn hashmap_update(map: &mut FxHashMap<u32, Agg>, ticks: &[Tick]) {
    for tick in ticks {
        map.entry(tick.symbol)
            .and_modify(|agg| update(agg, tick))
            .or_insert(Agg {
                last_price: tick.price,
                total_volume: tick.volume,
                tick_count: 1,
            });
    }
}

fn btreemap_update(map: &mut BTreeMap<u32, Agg>, ticks: &[Tick]) {
    for tick in ticks {
        map.entry(tick.symbol)
            .and_modify(|agg| update(agg, tick))
            .or_insert(Agg {
                last_price: tick.price,
                total_volume: tick.volume,
                tick_count: 1,
            });
    }
}

fn vecmap_update(vecmap: &mut VecMap, ticks: &[Tick]) {
    for tick in ticks {
        let agg = vecmap[tick.symbol as usize].get_or_insert(Agg::default());
        update(agg, tick);
    }
}

// ---- Lookup (query lastest price) ----

fn hashmap_lookup(map: &FxHashMap<u32, Agg>, queries: &[u32]) -> f64 {
    let mut sum = 0.0;
    for q in queries {
        if let Some(agg) = map.get(q) {
            sum += agg.last_price;
        }
    }
    sum
}

fn btreemap_lookup(map: &BTreeMap<u32, Agg>, queries: &[u32]) -> f64 {
    let mut sum = 0.0;
    for q in queries {
        if let Some(agg) = map.get(q) {
            sum += agg.last_price;
        }
    }
    sum
}

fn vecmap_lookup(vecmap: &VecMap, queries: &[u32]) -> f64 {
    let mut sum = 0.0;
    for &q in queries {
        if let Some(agg) = &vecmap[q as usize] {
            sum += agg.last_price;
        }
    }
    sum
}

// ---- Scan (simulate analytics query) ----

fn hashmap_scan(map: &FxHashMap<u32, Agg>) -> f64 {
    map.values().map(|a| a.total_volume).sum()
}

fn btreemap_scan(map: &BTreeMap<u32, Agg>) -> f64 {
    map.values().map(|a| a.total_volume).sum()
}

fn vecmap_scan(vecmap: &VecMap) -> f64 {
    vecmap
        .iter()
        .filter_map(|x| x.as_ref())
        .map(|a| a.total_volume)
        .sum()
}

// ---- Range query 100–200 (for ordered structure) ----

fn btreemap_range(map: &BTreeMap<u32, Agg>) -> f64 {
    map.range(100..200).map(|(_, a)| a.total_volume).sum()
}

fn vecmap_range(vecmap: &VecMap) -> f64 {
    vecmap
        .iter()
        .skip(100)
        .take(100)
        .filter_map(|x| x.as_ref())
        .map(|a| a.total_volume)
        .sum()
}

// ---- Benchmark helpers ----

fn sample_four_ops<F1, F2, F3, F4>(
    update_fn: F1,
    lookup_fn: F2,
    scan_fn: F3,
    range_fn: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn() -> f64,
    F4: Fn() -> f64,
{
    let mut update_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut lookup_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut scan_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut range_s = Vec::with_capacity(NUM_RUNS as usize);
    for _ in 0..NUM_RUNS {
        let t = Instant::now();
        update_fn();
        black_box(());
        update_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(lookup_fn());
        lookup_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(scan_fn());
        scan_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(range_fn());
        range_s.push(t.elapsed().as_secs_f64() * 1000.0);
    }
    (update_s, lookup_s, scan_s, range_s)
}

fn warm_up_four_ops<F1, F2, F3, F4>(update_fn: F1, lookup_fn: F2, scan_fn: F3, range_fn: F4)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn() -> f64,
    F4: Fn() -> f64,
{
    update_fn();
    lookup_fn();
    scan_fn();
    range_fn();
}

fn run_structure_ops<F1, F2, F3, F4>(
    update: F1,
    lookup: F2,
    scan: F3,
    range: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> f64,
    F3: Fn() -> f64,
    F4: Fn() -> f64,
{
    warm_up_four_ops(&update, &lookup, &scan, &range);
    sample_four_ops(update, lookup, scan, range)
}

struct OpMetrics {
    update_m: f64,
    update_s: f64,
    lookup_m: f64,
    lookup_s: f64,
    scan_m: f64,
    scan_s: f64,
    range_m: f64,
    range_s: f64,
}

fn op_metrics_from_samples(
    update_s: &[f64],
    lookup_s: &[f64],
    scan_s: &[f64],
    range_s: &[f64],
) -> OpMetrics {
    let (update_m, update_s_val) = mean_std(update_s);
    let (lookup_m, lookup_s_val) = mean_std(lookup_s);
    let (scan_m, scan_s_val) = mean_std(scan_s);
    let (range_m, range_s_val) = mean_std(range_s);
    OpMetrics {
        update_m,
        update_s: update_s_val,
        lookup_m,
        lookup_s: lookup_s_val,
        scan_m,
        scan_s: scan_s_val,
        range_m,
        range_s: range_s_val,
    }
}

struct RowMetrics {
    n: usize,
    hashmap: OpMetrics,
    btreemap: OpMetrics,
    vecmap: OpMetrics,
    hashmap_mem: f64,
    btreemap_mem: f64,
    vecmap_mem: f64,
}

fn compute_row(
    hashmap_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    btreemap_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    vecmap_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    mem: (f64, f64, f64, f64),
    n: usize,
) -> RowMetrics {
    RowMetrics {
        n,
        hashmap: op_metrics_from_samples(
            &hashmap_samples.0,
            &hashmap_samples.1,
            &hashmap_samples.2,
            &hashmap_samples.3,
        ),
        btreemap: op_metrics_from_samples(
            &btreemap_samples.0,
            &btreemap_samples.1,
            &btreemap_samples.2,
            &btreemap_samples.3,
        ),
        vecmap: op_metrics_from_samples(
            &vecmap_samples.0,
            &vecmap_samples.1,
            &vecmap_samples.2,
            &vecmap_samples.3,
        ),
        hashmap_mem: mem.0,
        btreemap_mem: mem.1,
        vecmap_mem: mem.2,
    }
}

fn write_csv_row(file: &mut File, row: &RowMetrics) {
    let hashmap = &row.hashmap;
    let btreemap = &row.btreemap;
    let vecmap = &row.vecmap;
    writeln!(
        file,
        "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.4},{:.4},{:.4}",
        row.n,
        hashmap.update_m,
        hashmap.update_s,
        hashmap.lookup_m,
        hashmap.lookup_s,
        hashmap.scan_m,
        hashmap.scan_s,
        hashmap.range_m,
        hashmap.range_s,
        btreemap.update_m,
        btreemap.update_s,
        btreemap.lookup_m,
        btreemap.lookup_s,
        btreemap.scan_m,
        btreemap.scan_s,
        btreemap.range_m,
        btreemap.range_s,
        vecmap.update_m,
        vecmap.update_s,
        vecmap.lookup_m,
        vecmap.lookup_s,
        vecmap.scan_m,
        vecmap.scan_s,
        vecmap.range_m,
        vecmap.range_s,
        row.hashmap_mem,
        row.btreemap_mem,
        row.vecmap_mem,
    )
    .expect("write row");
}

fn print_row(row: &RowMetrics) {
    let hashmap = &row.hashmap;
    let btreemap = &row.btreemap;
    let vecmap = &row.vecmap;
    println!(
        "N={}: update HashMap={:.4} BTreeMap={:.4} VecMap={:.4} ms",
        row.n, hashmap.update_m, btreemap.update_m, vecmap.update_m
    );
    println!(
        "      lookup HashMap={:.4} BTreeMap={:.4} VecMap={:.4} ms",
        hashmap.lookup_m, btreemap.lookup_m, vecmap.lookup_m
    );
    println!(
        "      scan HashMap={:.4} BTreeMap={:.4} VecMap={:.4} ms",
        hashmap.scan_m, btreemap.scan_m, vecmap.scan_m
    );
    println!(
        "      range HashMap={:.4} BTreeMap={:.4} VecMap={:.4} ms",
        hashmap.range_m, btreemap.range_m, vecmap.range_m
    );
    println!(
        "      memory HashMap={:.4} BTreeMap={:.4} VecMap={:.4} MB",
        row.hashmap_mem, row.btreemap_mem, row.vecmap_mem
    );
}

fn measure_memory<F, T>(build: F) -> f64
where
    F: FnOnce() -> T,
{
    let _x = build();
    memory_mb()
}

fn run() {
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_workload_hashmap.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    // N + 4 structs × 4 ops × 2 (mean, std) + 4 memory = 37 columns
    writeln!(
        file,
        "N,hashmap_update_mean_ms,hashmap_update_std_ms,hashmap_lookup_mean_ms,hashmap_lookup_std_ms,hashmap_scan_mean_ms,hashmap_scan_std_ms,hashmap_range_mean_ms,hashmap_range_std_ms,\
btreemap_update_mean_ms,btreemap_update_std_ms,btreemap_lookup_mean_ms,btreemap_lookup_std_ms,btreemap_scan_mean_ms,btreemap_scan_std_ms,btreemap_range_mean_ms,btreemap_range_std_ms,\
vecmap_update_mean_ms,vecmap_update_std_ms,vecmap_lookup_mean_ms,vecmap_lookup_std_ms,vecmap_scan_mean_ms,vecmap_scan_std_ms,vecmap_range_mean_ms,vecmap_range_std_ms,\
hashmap_memory_mb,btreemap_memory_mb,vecmap_memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        let symbols = n;
        let data = generate_ticks(n, symbols as u32);
        let queries = generate_queries(n, symbols as u32);

        let hashmap = data_to_hashmap(&data);
        let btreemap = data_to_btreemap(&data);
        let vecmap = data_to_vecmap(&data, symbols);

        let hashmap_samples = run_structure_ops(
            || {
                let mut map = hashmap.clone();
                hashmap_update(&mut map, &data);
            },
            || hashmap_lookup(&hashmap, &queries),
            || hashmap_scan(&hashmap),
            || 0.0,
        );

        let btreemap_samples = run_structure_ops(
            || {
                let mut map = btreemap.clone();
                btreemap_update(&mut map, &data);
            },
            || btreemap_lookup(&btreemap, &queries),
            || btreemap_scan(&btreemap),
            || btreemap_range(&btreemap),
        );

        let vecmap_samples = run_structure_ops(
            || {
                let mut vec = vecmap.clone();
                vecmap_update(&mut vec, &data);
            },
            || vecmap_lookup(&vecmap, &queries),
            || vecmap_scan(&vecmap),
            || vecmap_range(&vecmap),
        );

        let hashmap_mem = measure_memory(|| data_to_hashmap(&data));
        let btreemap_mem = measure_memory(|| data_to_btreemap(&data));
        let vecmap_mem = measure_memory(|| data_to_vecmap(&data, symbols));

        let row = compute_row(
            hashmap_samples,
            btreemap_samples,
            vecmap_samples,
            (hashmap_mem, btreemap_mem, vecmap_mem, 0.0),
            n,
        );

        write_csv_row(&mut file, &row);
        print_row(&row);
    }

    println!("Wrote {}", csv_path);
}

fn main() {
    run();
}
