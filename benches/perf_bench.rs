use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn add(i: i32) -> i32 {
    i + i
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| b.iter(|| add(black_box(5))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
