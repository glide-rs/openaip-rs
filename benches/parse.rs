use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openaip::parse;

pub fn criterion_benchmark(c: &mut Criterion) {
    let str = include_str!("../tests/data/de_asp.aip");

    c.bench_function("parse", |b| b.iter(|| parse(black_box(str))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
