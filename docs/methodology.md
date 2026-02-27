# Methodology

Benchmarks are run in isolation on the same machine (or the same Docker image) so that results are comparable across languages.

- **Warmup**: Where supported, a short warmup phase is run before timed iterations.
- **Fixed N**: Scenario sizes (e.g. 1M elements) are fixed so that runs are comparable.
- **Iterations**: Detailed iteration counts, repetition, and statistical reporting will be documented when real benchmark harnesses are in place.
- **Memory**: How memory is measured (heap snapshot, `/proc`, language-specific APIs) will be described once implemented.
