use aoc2023::day06;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day06::part1", |b| {
        b.iter(|| day06::part1(black_box(&day06::REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
