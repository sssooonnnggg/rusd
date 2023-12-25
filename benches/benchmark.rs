use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};
use rusd::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse large usda file", |b| {
        b.iter(|| {
            let usda = read_to_string("benches/Workspace.usda").unwrap();
            let _ = UsdParser::parse(Rule::usd, &usda).unwrap();
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
