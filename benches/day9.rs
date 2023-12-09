use aoc2023::day9;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let report = day9::Report::parse(day9::REAL);
    c.bench_function("day9::part2", |b| b.iter(|| report.predict_prior_total()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
