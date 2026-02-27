# Findings

Findings will be updated after running benchmarks and analyzing outputs in `results/raw/`.

### Rust Custom HashMap Benchmark Results (first try)

| Operation | N=1e5 | Notes |
|-----------|-------|------|
| Insert    | 7.88–8.01 ms | Resizing triggered at 75% load |
| Get       | 504–509 µs  | Consistent, low variance |

Observations:

- Outliers minor (7% insert, 2% get)
- Performance scales roughly linearly
- Could optimize with pre-allocated buckets