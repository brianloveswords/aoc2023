use aoc2023::day4;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day4::part2", |b| {
        b.iter(|| day4::part2(black_box(day4::REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
