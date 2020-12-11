use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day10::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let v = day10::parse_input("input.txt");
    c.bench_function("Part1", |b| b.iter(|| part1(black_box(&v))));
    c.bench_function("Part2", |b| b.iter(|| part2(black_box(&v))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
