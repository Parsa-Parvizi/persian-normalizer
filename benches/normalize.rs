#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use persian_normalizer::{normalize, Normalizer};
use std::hint::black_box;

/// Repeats `unit` until the result is at least `bytes` long.
fn repeat_to(unit: &str, bytes: usize) -> String {
    let mut text = String::with_capacity(bytes + unit.len());
    while text.len() < bytes {
        text.push_str(unit);
    }
    text
}

fn bench_normalize(c: &mut Criterion) {
    // Already normalized: the zero-copy path.
    let clean_unit = "این یک متن فارسی تمیز است. ";
    // Needs work in every step: Arabic letters, Arabic-Indic digits,
    // diacritics, doubled spaces and ZWNJ noise.
    let messy_unit = "كتاب  علي\u{0650} ١٢٣ \u{200C}\u{200C} می  روم, کتاب ها? ";

    let default = Normalizer::default();
    let full = Normalizer::builder()
        .smart_zwnj(true)
        .punctuation(true)
        .build();

    let mut group = c.benchmark_group("normalize");
    for size in [64usize, 4 * 1024, 256 * 1024] {
        let clean = repeat_to(clean_unit, size);
        let messy = repeat_to(messy_unit, size);

        group.throughput(Throughput::Bytes(clean.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("clean/default", size),
            &clean,
            |b, text| {
                b.iter(|| normalize(black_box(text)));
            },
        );

        group.throughput(Throughput::Bytes(messy.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("messy/default", size),
            &messy,
            |b, text| {
                b.iter(|| default.normalize(black_box(text)));
            },
        );
        group.bench_with_input(BenchmarkId::new("messy/full", size), &messy, |b, text| {
            b.iter(|| full.normalize(black_box(text)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_normalize);
criterion_main!(benches);
