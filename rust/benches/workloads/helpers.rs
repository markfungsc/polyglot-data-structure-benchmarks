use polyglot_benchmarks::bench_util::memory_mb;

pub fn measure_memory<F, T>(build: F) -> (T, f64)
where
    F: FnOnce() -> T,
{
    let before = memory_mb();
    let x = build();
    let after = memory_mb();
    (x, after - before)
}