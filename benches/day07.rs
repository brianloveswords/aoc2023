use aoc2023::day07;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day07::part2", |b| {
        b.iter(|| day07::part2(black_box(&day07::REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
