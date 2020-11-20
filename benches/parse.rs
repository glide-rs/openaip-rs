use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openaip::parse;

pub fn criterion_benchmark(c: &mut Criterion) {
    let bytes = include_bytes!("../tests/data/de_asp.aip");

    c.bench_function("parse", |b| {
        b.iter(|| parse::<&[u8]>(black_box(bytes).as_ref()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
