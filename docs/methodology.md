# Methodology

Benchmarks are run in isolation on the same machine (or the same Docker image) so that results are comparable across languages.

- **Warmup**: One untimed run per scale (or per scenario) before timed iterations.
- **Fixed N**: Scenario sizes (e.g. 1k, 10k, 100k, 1M) are fixed so that runs are comparable.
- **Iterations**: Each scenario runs 5 times; we report mean and standard deviation (insert_mean_ms, insert_std_ms, get_mean_ms, get_std_ms).
- **Memory**: Best-effort per language: Python uses `resource.getrusage` (Linux) peak RSS; Java uses `Runtime.totalMemory() - freeMemory()` (heap used); C++ uses `getrusage(RUSAGE_SELF).ru_maxrss` (Linux); Rust reads `/proc/self/status` VmRSS (Linux). Reported in MB. Not comparable across languages due to GC and allocator differences.
