use aoc2023::day03;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SCHEMATIC: &str = day03::REAL;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day03::part1", |b| {
        b.iter(|| day03::part1(black_box(SCHEMATIC)))
    });
    c.bench_function("day03::part2", |b| {
        b.iter(|| day03::part2(black_box(SCHEMATIC)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
