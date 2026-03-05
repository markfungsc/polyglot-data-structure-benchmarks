// Linked list benchmark: push (insert) and get by index. Same schema as hashmap.
use polyglot_benchmarks::bench_util::{mean_std, memory_mb, NUM_RUNS, SCALES};
use polyglot_benchmarks::linked_list::LinkedList;
use rand::prelude::*;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let out_dir = std::env::var("RESULTS_DIR").unwrap_or_else(|_| "../results/raw".into());
    std::fs::create_dir_all(&out_dir).expect("create results dir");

    let csv_path = format!("{}/rust_linked_list.csv", out_dir);
    let mut file = File::create(&csv_path).expect("create csv");
    writeln!(file, "N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,delete_mean_ms,delete_std_ms,memory_mb").expect("write header");

    for &n in &SCALES {
        let mut insert_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut get_samples = Vec::with_capacity(NUM_RUNS as usize);
        let mut delete_samples = Vec::with_capacity(NUM_RUNS as usize);

        {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut list = LinkedList::new();
            // insert (in order)
            for &k in &keys {
                list.push_back(k);
            }

            // Sequential traversal (instead of get(i))
            let mut sum = 0i64;
            list.traverse(|v| sum += v as i64);
            black_box(sum);

            // Delete the last element
            list.delete(list.size() - 1);
        }

        for _ in 0..NUM_RUNS {
            let mut keys: Vec<i32> = (0..n as i32).collect();
            keys.shuffle(&mut rng);
            let mut list = LinkedList::new();
            // insert (in order)
            let start = Instant::now();
            for &k in &keys {
                list.push_back(k);
            }
            insert_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // Sequential traversal (instead of get(i))
            let start = Instant::now();
            let mut sum: i64 = 0;
            list.traverse(|v| sum += v as i64);
            black_box(sum);
            get_samples.push(start.elapsed().as_secs_f64() * 1000.0);

            // Delete the last element
            let start = Instant::now();
            list.delete(list.size() - 1);
            delete_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let (i_mean, i_std) = mean_std(&insert_samples);
        let (g_mean, g_std) = mean_std(&get_samples);
        let (d_mean, d_std) = mean_std(&delete_samples);
        let mem = memory_mb();
        writeln!(
            file,
            "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.4}",
            n, i_mean, i_std, g_mean, g_std, d_mean, d_std, mem
        )
        .expect("write row");
        println!("N={}: Insert {:.6} ± {:.6} ms, Get {:.6} ± {:.6} ms, Delete {:.6} ± {:.6} ms, memory={:.4} MB", n, i_mean, i_std, g_mean, g_std, d_mean, d_std, mem);
    }
    println!("Wrote {}", csv_path);
}
