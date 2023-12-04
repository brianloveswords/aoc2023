use aoc2023::util::Range;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_overlapping", |b| {
        b.iter(|| {
            let a = Range::new(3, 10);
            let b = Range::new(1, 7);
            black_box(a).is_overlapping(&black_box(b));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
