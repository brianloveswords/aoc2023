use aoc2023::day09;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let report = day09::Report::parse(day09::REAL);
    c.bench_function("day09::part2", |b| b.iter(|| report.predict_prior_total()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
