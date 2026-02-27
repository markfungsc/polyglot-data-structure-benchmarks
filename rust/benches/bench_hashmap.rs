// benches/bench_hashmap.rs
use criterion::{criterion_group, criterion_main, Criterion};
use polyglot_benchmarks::hashmap::HashMap;

fn bench_insert(c: &mut Criterion) {
    c.bench_function("hashmap_insert_1e5", |b| {
        b.iter(|| {
            let mut map = HashMap::new(100_000);
            for i in 0..100_000 {
                map.insert(i, i);
            }
        });
    });
}

fn bench_get(c: &mut Criterion) {
    let mut map = HashMap::new(100_000);
    for i in 0..100_000 {
        map.insert(i, i);
    }

    c.bench_function("hashmap_get_1e5", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                map.get(&i);
            }
        });
    });
}

criterion_group!(benches, bench_insert, bench_get);
criterion_main!(benches);