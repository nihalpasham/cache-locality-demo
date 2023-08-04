use cache_demo::{sum_cached, sum_uncached};
use criterion::{criterion_group, criterion_main, Criterion};

fn sum_cached_benchmark(c: &mut Criterion) {
    c.bench_function("sum_cached", |b| b.iter(|| sum_cached()));
}

fn sum_uncached_benchmark(c: &mut Criterion) {
    c.bench_function("sum_poorly_cached", |b| b.iter(|| sum_uncached()));
}
criterion_group!(benches, sum_cached_benchmark, sum_uncached_benchmark);
criterion_main!(benches);
