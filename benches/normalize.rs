#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use persian_normalizer::normalize;
use std::hint::black_box;

fn bench_normalize(c: &mut Criterion) {
    c.bench_function("normalize_clean_input", |b| {
        b.iter(|| normalize(black_box("کتاب")));
    });
}

criterion_group!(benches, bench_normalize);
criterion_main!(benches);