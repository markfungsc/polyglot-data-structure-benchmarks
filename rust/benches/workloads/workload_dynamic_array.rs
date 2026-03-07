use std::collections::{LinkedList, VecDeque};
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};

#[derive(Clone, Copy)]
struct Tick {
    price: f64,
    volume: f64,
}

struct TickColumns {
    prices: Vec<f64>,
    volumes: Vec<f64>,
}

fn generate_ticks(n: usize) -> Vec<Tick> {
    let mut ticks = Vec::with_capacity(n);
    for i in 0..n {
        ticks.push(Tick {
            price: i as f64,
            volume: 1.0,
        });
    }
    ticks
}

fn convert_to_columns(data: &[Tick]) -> TickColumns {
    let mut prices = Vec::with_capacity(data.len());
    let mut volumes = Vec::with_capacity(data.len());
    for t in data {
        prices.push(t.price);
        volumes.push(t.volume);
    }
    TickColumns { prices, volumes }
}

// ---- Sum (sequential iteration, 10 passes) ----

fn vec_sum(vec: &[Tick]) -> f64 {
    let mut sum = 0.0;
    for _ in 0..10 {
        for t in vec {
            sum += t.price * t.volume;
        }
    }
    sum
}

fn vecdeque_sum(deque: &VecDeque<Tick>) -> f64 {
    let mut sum = 0.0;
    for _ in 0..10 {
        for t in deque {
            sum += t.price * t.volume;
        }
    }
    sum
}

fn linkedlist_sum(list: &LinkedList<Tick>) -> f64 {
    let mut sum = 0.0;
    for _ in 0..10 {
        for t in list {
            sum += t.price * t.volume;
        }
    }
    sum
}

fn columnar_sum(cols: &TickColumns) -> f64 {
    let mut sum = 0.0;
    for _ in 0..10 {
        for (p, v) in cols.prices.iter().zip(&cols.volumes) {
            sum += p * v;
        }
    }
    sum
}

// ---- VWAP (sliding window: sum(price*vol)/sum(vol) per window) ----
// Window size W = min(1000, max(1, n/10)). Return sum of all window VWAPs.

fn vec_vwap(vec: &[Tick], window: usize) -> f64 {
    if vec.len() < window {
        return 0.0;
    }
    // Sliding window: maintain sum(price*vol) and sum(vol), O(1) update per step
    let mut pv: f64 = (0..window).map(|i| vec[i].price * vec[i].volume).sum();
    let mut v: f64 = (0..window).map(|i| vec[i].volume).sum();
    let mut total = if v > 0.0 { pv / v } else { 0.0 };
    for i in window..vec.len() {
        pv += vec[i].price * vec[i].volume - vec[i - window].price * vec[i - window].volume;
        v += vec[i].volume - vec[i - window].volume;
        total += if v > 0.0 { pv / v } else { 0.0 };
    }
    total
}

fn vecdeque_vwap(deque: &VecDeque<Tick>, window: usize) -> f64 {
    let n = deque.len();
    if n < window {
        return 0.0;
    }
    let mut pv = 0.0;
    let mut v = 0.0;
    for t in deque.iter().take(window) {
        pv += t.price * t.volume;
        v += t.volume;
    }
    let mut total = if v > 0.0 { pv / v } else { 0.0 };
    for i in window..n {
        pv += deque[i].price * deque[i].volume - deque[i - window].price * deque[i - window].volume;
        v += deque[i].volume - deque[i - window].volume;
        total += if v > 0.0 { pv / v } else { 0.0 };
    }
    total
}

fn linkedlist_vwap(list: &LinkedList<Tick>, window: usize) -> f64 {
    let vec: Vec<Tick> = list.iter().copied().collect();
    vec_vwap(&vec, window)
}

fn columnar_vwap(cols: &TickColumns, window: usize) -> f64 {
    let n = cols.prices.len();
    if n < window {
        return 0.0;
    }
    let mut pv: f64 = cols.prices[..window]
        .iter()
        .zip(&cols.volumes[..window])
        .map(|(p, v)| p * v)
        .sum();
    let mut v: f64 = cols.volumes[..window].iter().sum();
    let mut total = if v > 0.0 { pv / v } else { 0.0 };
    for i in window..n {
        pv += cols.prices[i] * cols.volumes[i] - cols.prices[i - window] * cols.volumes[i - window];
        v += cols.volumes[i] - cols.volumes[i - window];
        total += if v > 0.0 { pv / v } else { 0.0 };
    }
    total
}

// ---- Moving average (sliding window average of price) ----

fn vec_ma(vec: &[Tick], window: usize) -> f64 {
    if vec.len() < window {
        return 0.0;
    }
    let mut sum: f64 = vec[0..window].iter().map(|t| t.price).sum();
    let mut total = sum / window as f64;
    for i in window..vec.len() {
        sum += vec[i].price - vec[i - window].price;
        total += sum / window as f64;
    }
    total
}

fn vecdeque_ma(deque: &VecDeque<Tick>, window: usize) -> f64 {
    let n = deque.len();
    if n < window {
        return 0.0;
    }
    let mut sum: f64 = deque.iter().take(window).map(|t| t.price).sum();
    let mut total = sum / window as f64;
    for i in window..n {
        sum += deque[i].price - deque[i - window].price;
        total += sum / window as f64;
    }
    total
}

fn linkedlist_ma(list: &LinkedList<Tick>, window: usize) -> f64 {
    let vec: Vec<Tick> = list.iter().copied().collect();
    vec_ma(&vec, window)
}

fn columnar_ma(cols: &TickColumns, window: usize) -> f64 {
    let n = cols.prices.len();
    if n < window {
        return 0.0;
    }
    let mut sum: f64 = cols.prices[0..window].iter().sum();
    let mut total = sum / window as f64;
    for i in window..n {
        sum += cols.prices[i] - cols.prices[i - window];
        total += sum / window as f64;
    }
    total
}

// ---- Filter (price > threshold, collect into new structure) ----

fn vec_filter(vec: &[Tick], threshold: f64) -> (usize, f64) {
    let filtered: Vec<Tick> = vec
        .iter()
        .copied()
        .filter(|t| t.price > threshold)
        .collect();
    let sum: f64 = filtered.iter().map(|t| t.price * t.volume).sum();
    (filtered.len(), sum)
}

fn vecdeque_filter(deque: &VecDeque<Tick>, threshold: f64) -> (usize, f64) {
    let filtered: VecDeque<Tick> = deque
        .iter()
        .copied()
        .filter(|t| t.price > threshold)
        .collect();
    let sum: f64 = filtered.iter().map(|t| t.price * t.volume).sum();
    (filtered.len(), sum)
}

fn linkedlist_filter(list: &LinkedList<Tick>, threshold: f64) -> (usize, f64) {
    let filtered: LinkedList<Tick> = list
        .iter()
        .copied()
        .filter(|t| t.price > threshold)
        .collect();
    let sum: f64 = filtered.iter().map(|t| t.price * t.volume).sum();
    (filtered.len(), sum)
}

fn columnar_filter(cols: &TickColumns, threshold: f64) -> (usize, f64) {
    let mut count = 0;
    let mut sum = 0.0;
    for (p, v) in cols.prices.iter().zip(&cols.volumes) {
        if *p > threshold {
            count += 1;
            sum += p * v;
        }
    }
    (count, sum)
}

// ---- Benchmark helpers ----

fn sample_four_ops<F1, F2, F3, F4>(
    sum_fn: F1,
    vwap_fn: F2,
    ma_fn: F3,
    filter_fn: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn() -> f64,
    F2: Fn() -> f64,
    F3: Fn() -> f64,
    F4: Fn() -> (usize, f64),
{
    let mut sum_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut vwap_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut ma_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut filter_s = Vec::with_capacity(NUM_RUNS as usize);
    for _ in 0..NUM_RUNS {
        let t = Instant::now();
        black_box(sum_fn());
        sum_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(vwap_fn());
        vwap_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(ma_fn());
        ma_s.push(t.elapsed().as_secs_f64() * 1000.0);
        let t = Instant::now();
        black_box(filter_fn());
        filter_s.push(t.elapsed().as_secs_f64() * 1000.0);
    }
    (sum_s, vwap_s, ma_s, filter_s)
}

fn warm_up_four_ops<F1, F2, F3, F4>(sum_fn: F1, vwap_fn: F2, ma_fn: F3, filter_fn: F4)
where
    F1: Fn() -> f64,
    F2: Fn() -> f64,
    F3: Fn() -> f64,
    F4: Fn() -> (usize, f64),
{
    black_box(sum_fn());
    black_box(vwap_fn());
    black_box(ma_fn());
    black_box(filter_fn());
}

struct OpMetrics {
    sum_m: f64,
    sum_s: f64,
    vwap_m: f64,
    vwap_s: f64,
    ma_m: f64,
    ma_s: f64,
    filter_m: f64,
    filter_s: f64,
}

fn op_metrics_from_samples(
    sum_s: &[f64],
    vwap_s: &[f64],
    ma_s: &[f64],
    filter_s: &[f64],
) -> OpMetrics {
    let (sum_m, sum_s_val) = mean_std(sum_s);
    let (vwap_m, vwap_s_val) = mean_std(vwap_s);
    let (ma_m, ma_s_val) = mean_std(ma_s);
    let (filter_m, filter_s_val) = mean_std(filter_s);
    OpMetrics {
        sum_m,
        sum_s: sum_s_val,
        vwap_m,
        vwap_s: vwap_s_val,
        ma_m,
        ma_s: ma_s_val,
        filter_m,
        filter_s: filter_s_val,
    }
}

struct RowMetrics {
    n: usize,
    vec: OpMetrics,
    vecdeque: OpMetrics,
    linkedlist: OpMetrics,
    columnar: OpMetrics,
    vec_mem: f64,
    vecdeque_mem: f64,
    linkedlist_mem: f64,
    columnar_mem: f64,
}

fn compute_row(
    vec_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    vd_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    ll_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    col_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    mem: (f64, f64, f64, f64),
    n: usize,
) -> RowMetrics {
    RowMetrics {
        n,
        vec: op_metrics_from_samples(
            &vec_samples.0,
            &vec_samples.1,
            &vec_samples.2,
            &vec_samples.3,
        ),
        vecdeque: op_metrics_from_samples(
            &vd_samples.0,
            &vd_samples.1,
            &vd_samples.2,
            &vd_samples.3,
        ),
        linkedlist: op_metrics_from_samples(
            &ll_samples.0,
            &ll_samples.1,
            &ll_samples.2,
            &ll_samples.3,
        ),
        columnar: op_metrics_from_samples(
            &col_samples.0,
            &col_samples.1,
            &col_samples.2,
            &col_samples.3,
        ),
        vec_mem: mem.0,
        vecdeque_mem: mem.1,
        linkedlist_mem: mem.2,
        columnar_mem: mem.3,
    }
}

fn write_csv_row(file: &mut File, row: &RowMetrics) {
    let v = &row.vec;
    let vd = &row.vecdeque;
    let ll = &row.linkedlist;
    let col = &row.columnar;
    writeln!(
        file,
        "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.4},{:.4},{:.4},{:.4}",
        row.n,
        v.sum_m,
        v.sum_s,
        v.vwap_m,
        v.vwap_s,
        v.ma_m,
        v.ma_s,
        v.filter_m,
        v.filter_s,
        vd.sum_m,
        vd.sum_s,
        vd.vwap_m,
        vd.vwap_s,
        vd.ma_m,
        vd.ma_s,
        vd.filter_m,
        vd.filter_s,
        ll.sum_m,
        ll.sum_s,
        ll.vwap_m,
        ll.vwap_s,
        ll.ma_m,
        ll.ma_s,
        ll.filter_m,
        ll.filter_s,
        col.sum_m,
        col.sum_s,
        col.vwap_m,
        col.vwap_s,
        col.ma_m,
        col.ma_s,
        col.filter_m,
        col.filter_s,
        row.vec_mem,
        row.vecdeque_mem,
        row.linkedlist_mem,
        row.columnar_mem
    )
    .expect("write row");
}

fn print_row(row: &RowMetrics) {
    let v = &row.vec;
    let vd = &row.vecdeque;
    let ll = &row.linkedlist;
    let col = &row.columnar;
    println!(
        "N={}: sum Vec={:.4} VecDeque={:.4} LinkedList={:.4} Columnar={:.4} ms",
        row.n, v.sum_m, vd.sum_m, ll.sum_m, col.sum_m
    );
    println!(
        "      vwap Vec={:.4} VecDeque={:.4} LinkedList={:.4} Columnar={:.4} ms",
        v.vwap_m, vd.vwap_m, ll.vwap_m, col.vwap_m
    );
    println!(
        "      ma   Vec={:.4} VecDeque={:.4} LinkedList={:.4} Columnar={:.4} ms",
        v.ma_m, vd.ma_m, ll.ma_m, col.ma_m
    );
    println!(
        "      filter Vec={:.4} VecDeque={:.4} LinkedList={:.4} Columnar={:.4} ms",
        v.filter_m, vd.filter_m, ll.filter_m, col.filter_m
    );
    println!(
        "      memory Vec={:.4} VecDeque={:.4} LinkedList={:.4} Columnar={:.4} MB",
        row.vec_mem, row.vecdeque_mem, row.linkedlist_mem, row.columnar_mem
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

    let csv_path = format!("{}/rust_workload_dynamic_array.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    // N + 4 structs × 4 ops × 2 (mean, std) + 4 memory = 37 columns
    writeln!(
        file,
        "N,vec_sum_mean_ms,vec_sum_std_ms,vec_vwap_mean_ms,vec_vwap_std_ms,vec_ma_mean_ms,vec_ma_std_ms,vec_filter_mean_ms,vec_filter_std_ms,\
vecdeque_sum_mean_ms,vecdeque_sum_std_ms,vecdeque_vwap_mean_ms,vecdeque_vwap_std_ms,vecdeque_ma_mean_ms,vecdeque_ma_std_ms,vecdeque_filter_mean_ms,vecdeque_filter_std_ms,\
linkedlist_sum_mean_ms,linkedlist_sum_std_ms,linkedlist_vwap_mean_ms,linkedlist_vwap_std_ms,linkedlist_ma_mean_ms,linkedlist_ma_std_ms,linkedlist_filter_mean_ms,linkedlist_filter_std_ms,\
columnar_sum_mean_ms,columnar_sum_std_ms,columnar_vwap_mean_ms,columnar_vwap_std_ms,columnar_ma_mean_ms,columnar_ma_std_ms,columnar_filter_mean_ms,columnar_filter_std_ms,\
vec_memory_mb,vecdeque_memory_mb,linkedlist_memory_mb,columnar_memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        let data = generate_ticks(n);
        let columns = convert_to_columns(&data);
        let w = (n / 10).clamp(1, 1000);
        let threshold = n as f64 / 2.0;

        let vec = data.to_vec();
        let vecdeque: VecDeque<Tick> = data.iter().copied().collect();
        let linkedlist: LinkedList<Tick> = data.iter().copied().collect();

        warm_up_four_ops(
            || vec_sum(&vec),
            || vec_vwap(&vec, w),
            || vec_ma(&vec, w),
            || vec_filter(&vec, threshold),
        );
        let vec_samples = sample_four_ops(
            || vec_sum(&vec),
            || vec_vwap(&vec, w),
            || vec_ma(&vec, w),
            || vec_filter(&vec, threshold),
        );
        warm_up_four_ops(
            || vecdeque_sum(&vecdeque),
            || vecdeque_vwap(&vecdeque, w),
            || vecdeque_ma(&vecdeque, w),
            || vecdeque_filter(&vecdeque, threshold),
        );
        let vd_samples = sample_four_ops(
            || vecdeque_sum(&vecdeque),
            || vecdeque_vwap(&vecdeque, w),
            || vecdeque_ma(&vecdeque, w),
            || vecdeque_filter(&vecdeque, threshold),
        );
        warm_up_four_ops(
            || linkedlist_sum(&linkedlist),
            || linkedlist_vwap(&linkedlist, w),
            || linkedlist_ma(&linkedlist, w),
            || linkedlist_filter(&linkedlist, threshold),
        );
        let ll_samples = sample_four_ops(
            || linkedlist_sum(&linkedlist),
            || linkedlist_vwap(&linkedlist, w),
            || linkedlist_ma(&linkedlist, w),
            || linkedlist_filter(&linkedlist, threshold),
        );
        warm_up_four_ops(
            || columnar_sum(&columns),
            || columnar_vwap(&columns, w),
            || columnar_ma(&columns, w),
            || columnar_filter(&columns, threshold),
        );
        let col_samples = sample_four_ops(
            || columnar_sum(&columns),
            || columnar_vwap(&columns, w),
            || columnar_ma(&columns, w),
            || columnar_filter(&columns, threshold),
        );

        let vec_mem = measure_memory(|| data.to_vec());
        let vd_mem = measure_memory(|| data.iter().copied().collect::<VecDeque<_>>());
        let ll_mem = measure_memory(|| data.iter().copied().collect::<LinkedList<_>>());
        let col_mem = measure_memory(|| convert_to_columns(&data));

        let row = compute_row(
            vec_samples,
            vd_samples,
            ll_samples,
            col_samples,
            (vec_mem, vd_mem, ll_mem, col_mem),
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
