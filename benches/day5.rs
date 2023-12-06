use aoc2023::day5;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day5::part2", |b| {
        let almanac = day5::Almanac::parse(black_box(day5::EXAMPLE));
        b.iter(|| almanac.process_all_seeds())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
