use aoc2023::day6;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day6::part1", |b| {
        b.iter(|| day6::part1(black_box(&day6::REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
