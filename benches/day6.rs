use aoc2023::day6;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count_record_beaters", |b| {
        b.iter(|| day6::EXAMPLE_PART2.count_record_beaters())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
