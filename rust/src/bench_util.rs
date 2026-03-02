//! Shared constants and helpers for benchmark binaries (same schema and methodology).

use std::fs::File;
use std::io::{BufRead, BufReader};

pub const SCALES: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];
pub const NUM_RUNS: u32 = 5;

pub fn memory_mb() -> f64 {
    let Ok(f) = File::open("/proc/self/status") else { return 0.0 };
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

pub fn mean_std(samples: &[f64]) -> (f64, f64) {
    let n = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let variance = if samples.len() >= 2 {
        samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
    } else {
        0.0
    };
    (mean, variance.sqrt())
}
