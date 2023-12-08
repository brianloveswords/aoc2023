use aoc2023::day7;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day7::part2", |b| {
        b.iter(|| day7::part2(black_box(&day7::REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
