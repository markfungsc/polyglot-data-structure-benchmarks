use std::collections::BinaryHeap;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;
use std::{cmp::Reverse, collections::BTreeSet};

use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};

// Benchmark: compare BinaryHeap, BTreeSet, SortedVec and Vec + Sort on the same values
// across four ops (push, pop, peek, topk) and memory at scales 1k–1M. Writes CSV to RESULTS_DIR.

fn generate_values(n: usize) -> Vec<i32> {
    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        values.push(i as i32);
    }
    values
}

// ---- PUSH (insert) ----

fn heap_push(values: &[i32]) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::with_capacity(values.len());

    for &v in values {
        heap.push(v);
    }

    heap
}

fn btree_push(values: &[i32]) -> BTreeSet<i32> {
    let mut tree = BTreeSet::new();

    for &v in values {
        tree.insert(v);
    }

    tree
}

fn sorted_vec_push(values: &[i32]) -> Vec<i32> {
    let mut vec = Vec::with_capacity(values.len());

    for &v in values {
        let pos = vec.binary_search(&v).unwrap_or_else(|p| p);
        vec.insert(pos, v);
    }

    vec
}

fn vec_full_sort_push(values: &[i32]) -> Vec<i32> {
    let mut vec = values.to_vec();
    vec.sort_unstable();
    vec
}

// ---- POP (remove the minimum) ----

fn heap_pop(heap: &mut BinaryHeap<i32>) -> i32 {
    let mut sum = 0;

    while let Some(v) = heap.pop() {
        sum += v;
    }

    sum
}

fn btree_pop(tree: &mut BTreeSet<i32>) -> i32 {
    let mut sum = 0;

    while let Some(v) = tree.pop_last() {
        sum += v;
    }

    sum
}

fn sorted_vec_pop(vec: &mut Vec<i32>) -> i32 {
    let mut sum = 0;

    while let Some(v) = vec.pop() {
        sum += v;
    }

    sum
}

fn vec_full_sort_pop(values: &[i32]) -> i32 {
    let mut vec = values.to_vec();

    vec.sort_unstable();

    let mut sum = 0;

    while let Some(v) = vec.pop() {
        sum += v;
    }

    sum
}

// ---- PEEK (get the minimum) ----

fn heap_peek(heap: &BinaryHeap<i32>, k: usize) -> i32 {
    let mut sum = 0;

    for _ in 0..k {
        if let Some(v) = black_box(&heap).peek() {
            sum += black_box(*v);
        }
    }

    sum
}

fn btree_peek(tree: &BTreeSet<i32>, k: usize) -> i32 {
    let mut sum = 0;

    for _ in 0..k {
        if let Some(v) = black_box(&tree).last() {
            sum += black_box(*v);
        }
    }

    sum
}

fn sorted_vec_peek(vec: &[i32], k: usize) -> i32 {
    let mut sum = 0;

    for _ in 0..k {
        if let Some(v) = black_box(&vec).last() {
            sum += black_box(*v);
        }
    }

    sum
}

fn vec_full_sort_peek(values: &[i32], k: usize) -> i32 {
    let mut vec = values.to_vec();

    vec.sort_unstable();

    let mut sum = 0;

    for _ in 0..k {
        if let Some(v) = black_box(&vec).last() {
            sum += black_box(*v);
        }
    }

    sum
}

// ---- TOPK (get the top k elements) ----

fn heap_topk(values: &[i32], k: usize) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k);

    for &v in values {
        if heap.len() < k {
            heap.push(Reverse(v));
        } else if let Some(&Reverse(min)) = heap.peek() {
            if v > min {
                heap.pop();
                heap.push(Reverse(v));
            }
        }
    }

    heap.into_iter().map(|Reverse(x)| x).sum()
}

fn btree_topk(values: &[i32], k: usize) -> i32 {
    let mut tree = BTreeSet::new();

    for &v in values {
        tree.insert(v);

        if tree.len() > k {
            tree.pop_first();
        }
    }

    tree.iter().sum()
}

fn vec_topk(values: &[i32], k: usize) -> i32 {
    let mut vec: Vec<i32> = Vec::with_capacity(k);

    for &v in values {
        // search for the position to insert the value
        let pos = vec.binary_search(&v).unwrap_or_else(|p| p);
        // insert the value at the position
        vec.insert(pos, v);

        // if the vector is longer than k, remove the first element
        if vec.len() > k {
            vec.remove(0);
        }
    }

    vec.iter().sum()
}

fn vec_full_sort_topk(values: &[i32], k: usize) -> i32 {
    let mut vec = values.to_vec();

    vec.sort_unstable_by(|a, b| b.cmp(a));

    vec[..k].iter().sum()
}

fn sample_four_ops<F1, F2, F3, F4>(
    push_fn: F1,
    pop_fn: F2,
    peek_fn: F3,
    topk_fn: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> i32,
    F3: Fn() -> i32,
    F4: Fn() -> i32,
{
    let mut push_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut pop_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut peek_s = Vec::with_capacity(NUM_RUNS as usize);
    let mut topk_s = Vec::with_capacity(NUM_RUNS as usize);

    for _ in 0..NUM_RUNS {
        let t = Instant::now();
        push_fn();
        black_box(());
        push_s.push(t.elapsed().as_secs_f64() * 1000.0);

        let t = Instant::now();
        black_box(pop_fn());
        pop_s.push(t.elapsed().as_secs_f64() * 1000.0);

        let t = Instant::now();
        black_box(peek_fn());
        peek_s.push(t.elapsed().as_secs_f64() * 1000.0);

        let t = Instant::now();
        black_box(topk_fn());
        topk_s.push(t.elapsed().as_secs_f64() * 1000.0);
    }

    (push_s, pop_s, peek_s, topk_s)
}

fn warm_up_four_ops<F1, F2, F3, F4>(push_fn: F1, pop_fn: F2, peek_fn: F3, topk_fn: F4)
where
    F1: Fn(),
    F2: Fn() -> i32,
    F3: Fn() -> i32,
    F4: Fn() -> i32,
{
    push_fn();
    pop_fn();
    peek_fn();
    topk_fn();
}

fn run_structure_ops<F1, F2, F3, F4>(
    push: F1,
    pop: F2,
    peek: F3,
    topk: F4,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    F1: Fn(),
    F2: Fn() -> i32,
    F3: Fn() -> i32,
    F4: Fn() -> i32,
{
    warm_up_four_ops(&push, &pop, &peek, &topk);
    sample_four_ops(push, pop, peek, topk)
}

struct OpMetrics {
    push_m: f64,
    push_s: f64,
    pop_m: f64,
    pop_s: f64,
    peek_m: f64,
    peek_s: f64,
    topk_m: f64,
    topk_s: f64,
}

fn op_metrics_from_samples(
    push_s: &[f64],
    pop_s: &[f64],
    peek_s: &[f64],
    topk_s: &[f64],
) -> OpMetrics {
    let (push_m, push_s_val) = mean_std(push_s);
    let (pop_m, pop_s_val) = mean_std(pop_s);
    let (peek_m, peek_s_val) = mean_std(peek_s);
    let (topk_m, topk_s_val) = mean_std(topk_s);
    OpMetrics {
        push_m,
        push_s: push_s_val,
        pop_m,
        pop_s: pop_s_val,
        peek_m,
        peek_s: peek_s_val,
        topk_m,
        topk_s: topk_s_val,
    }
}

struct RowMetrics {
    n: usize,
    binary_heap: OpMetrics,
    b_tree_set: OpMetrics,
    sorted_vec: OpMetrics,
    vec: OpMetrics,
    binary_heap_mem: f64,
    b_tree_set_mem: f64,
    sorted_vec_mem: f64,
    vec_mem: f64,
}

fn compute_row(
    binary_heap_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    b_tree_set_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    sorted_vec_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    vec_samples: (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
    mem: (f64, f64, f64, f64),
    n: usize,
) -> RowMetrics {
    RowMetrics {
        n,
        binary_heap: op_metrics_from_samples(
            &binary_heap_samples.0,
            &binary_heap_samples.1,
            &binary_heap_samples.2,
            &binary_heap_samples.3,
        ),
        b_tree_set: op_metrics_from_samples(
            &b_tree_set_samples.0,
            &b_tree_set_samples.1,
            &b_tree_set_samples.2,
            &b_tree_set_samples.3,
        ),
        sorted_vec: op_metrics_from_samples(
            &sorted_vec_samples.0,
            &sorted_vec_samples.1,
            &sorted_vec_samples.2,
            &sorted_vec_samples.3,
        ),
        vec: op_metrics_from_samples(
            &vec_samples.0,
            &vec_samples.1,
            &vec_samples.2,
            &vec_samples.3,
        ),
        binary_heap_mem: mem.0,
        b_tree_set_mem: mem.1,
        sorted_vec_mem: mem.2,
        vec_mem: mem.3,
    }
}

fn write_csv_row(file: &mut File, row: &RowMetrics) {
    let binary_heap = &row.binary_heap;
    let b_tree_set = &row.b_tree_set;
    let sorted_vec = &row.sorted_vec;
    let vec = &row.vec;
    writeln!(
        file,
        "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},\
{:.4},{:.4},{:.4},{:.4}",
        row.n,
        binary_heap.push_m,
        binary_heap.push_s,
        binary_heap.pop_m,
        binary_heap.pop_s,
        binary_heap.peek_m,
        binary_heap.peek_s,
        binary_heap.topk_m,
        binary_heap.topk_s,
        b_tree_set.push_m,
        b_tree_set.push_s,
        b_tree_set.pop_m,
        b_tree_set.pop_s,
        b_tree_set.peek_m,
        b_tree_set.peek_s,
        b_tree_set.topk_m,
        b_tree_set.topk_s,
        sorted_vec.push_m,
        sorted_vec.push_s,
        sorted_vec.pop_m,
        sorted_vec.pop_s,
        sorted_vec.peek_m,
        sorted_vec.peek_s,
        sorted_vec.topk_m,
        sorted_vec.topk_s,
        vec.push_m,
        vec.push_s,
        vec.pop_m,
        vec.pop_s,
        vec.peek_m,
        vec.peek_s,
        vec.topk_m,
        vec.topk_s,
        row.binary_heap_mem,
        row.b_tree_set_mem,
        row.sorted_vec_mem,
        row.vec_mem,
    )
    .expect("write row");
}

fn print_row(row: &RowMetrics) {
    let binary_heap = &row.binary_heap;
    let b_tree_set = &row.b_tree_set;
    let sorted_vec = &row.sorted_vec;
    let vec = &row.vec;
    println!(
        "N={}: update BinaryHeap={:.4} BTreeSet={:.4} SortedVec={:.4} Vec={:.4} ms",
        row.n, binary_heap.push_m, b_tree_set.push_m, sorted_vec.push_m, vec.push_m
    );
    println!(
        "      pop BinaryHeap={:.4} BTreeSet={:.4} SortedVec={:.4} Vec={:.4} ms",
        binary_heap.pop_m, b_tree_set.pop_m, sorted_vec.pop_m, vec.pop_m
    );
    println!(
        "      peek BinaryHeap={:.4} BTreeSet={:.4} SortedVec={:.4} Vec={:.4} ms",
        binary_heap.peek_m, b_tree_set.peek_m, sorted_vec.peek_m, vec.peek_m
    );
    println!(
        "      topk BinaryHeap={:.4} BTreeSet={:.4} SortedVec={:.4} Vec={:.4} ms",
        binary_heap.topk_m, b_tree_set.topk_m, sorted_vec.topk_m, vec.topk_m
    );
    println!(
        "      memory BinaryHeap={:.4} BTreeSet={:.4} SortedVec={:.4} Vec={:.4} MB",
        row.binary_heap_mem, row.b_tree_set_mem, row.sorted_vec_mem, row.vec_mem
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

    let csv_path = format!("{}/rust_workload_heap.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(
        file,
        "N,binary_heap_push_mean_ms,binary_heap_push_std_ms,binary_heap_pop_mean_ms,binary_heap_pop_std_ms,binary_heap_peek_mean_ms,binary_heap_peek_std_ms,binary_heap_topk_mean_ms,binary_heap_topk_std_ms,\
b_tree_set_push_mean_ms,b_tree_set_push_std_ms,b_tree_set_pop_mean_ms,b_tree_set_pop_std_ms,b_tree_set_peek_mean_ms,b_tree_set_peek_std_ms,b_tree_set_topk_mean_ms,b_tree_set_topk_std_ms,\
sorted_vec_push_mean_ms,sorted_vec_push_std_ms,sorted_vec_pop_mean_ms,sorted_vec_pop_std_ms,sorted_vec_peek_mean_ms,sorted_vec_peek_std_ms,sorted_vec_topk_mean_ms,sorted_vec_topk_std_ms,\
vec_push_mean_ms,vec_push_std_ms,vec_pop_mean_ms,vec_pop_std_ms,vec_peek_mean_ms,vec_peek_std_ms,vec_topk_mean_ms,vec_topk_std_ms,\
binary_heap_memory_mb,b_tree_set_memory_mb,sorted_vec_memory_mb,vec_memory_mb"
    )
    .expect("write header");

    for &n in &SCALES {
        let values = generate_values(n);
        let k = n / 10;

        let binary_heap = heap_push(&values);
        let b_tree_set = btree_push(&values);
        let sorted_vec = sorted_vec_push(&values);

        let binary_heap_samples = run_structure_ops(
            || {
                heap_push(&values);
            },
            || {
                let mut heap = binary_heap.clone();
                heap_pop(&mut heap)
            },
            || heap_peek(&binary_heap, k),
            || heap_topk(&values, k),
        );

        let b_tree_set_samples = run_structure_ops(
            || {
                btree_push(&values);
            },
            || {
                let mut tree = b_tree_set.clone();
                btree_pop(&mut tree)
            },
            || btree_peek(&b_tree_set, k),
            || btree_topk(&values, k),
        );

        let sorted_vec_samples = run_structure_ops(
            || {
                sorted_vec_push(&values);
            },
            || {
                let mut vec = sorted_vec.clone();
                sorted_vec_pop(&mut vec)
            },
            || sorted_vec_peek(&sorted_vec, k),
            || vec_topk(&values, k),
        );

        let vec_samples = run_structure_ops(
            || {
                vec_full_sort_push(&values);
            },
            || vec_full_sort_pop(&values),
            || vec_full_sort_peek(&values, k),
            || vec_full_sort_topk(&values, k),
        );

        let binary_heap_mem = measure_memory(|| heap_push(&values));
        let b_tree_set_mem = measure_memory(|| btree_push(&values));
        let sorted_vec_mem = measure_memory(|| sorted_vec_push(&values));
        let vec_mem = measure_memory(|| vec_full_sort_push(&values));

        let row = compute_row(
            binary_heap_samples,
            b_tree_set_samples,
            sorted_vec_samples,
            vec_samples,
            (binary_heap_mem, b_tree_set_mem, sorted_vec_mem, vec_mem),
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
