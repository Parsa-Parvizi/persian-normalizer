#![no_main]

use libfuzzer_sys::fuzz_target;
use persian_normalizer::Normalizer;

fuzz_target!(|text: &str| {
    // The most aggressive configuration exercises every step.
    let normalizer = Normalizer::builder()
        .smart_zwnj(true)
        .punctuation(true)
        .build();

    let once = normalizer.normalize(text).into_owned();

    // Idempotence.
    let twice = normalizer.normalize(&once);
    assert_eq!(once, twice);

    // Bounded output size (ARCHITECTURE.md, D-20).
    assert!(once.len() <= text.len().saturating_mul(3));
});
