//! Configuring a `Normalizer` with the builder.
//!
//! Build a normalizer once, then reuse it for many strings.
//!
//! Run with: `cargo run --example builder`

use persian_normalizer::{DigitStyle, Normalizer};

fn main() {
    // Arabic kaf (ك) followed by Arabic-Indic digits (١٢٣).
    let text = "كتاب ١٢٣";

    // The default configuration: Persian letters and Persian digits.
    let default = Normalizer::default();
    let out = default.normalize(text);
    println!("default          : {out}");
    assert_eq!(out, "کتاب ۱۲۳");

    // Convert digits to Latin instead.
    let latin_digits = Normalizer::builder().digits(DigitStyle::Latin).build();
    let out = latin_digits.normalize(text);
    println!("latin digits     : {out}");
    assert_eq!(out, "کتاب 123");

    // Leave digits exactly as they are.
    let keep_digits = Normalizer::builder().digits(DigitStyle::Keep).build();
    let out = keep_digits.normalize(text);
    println!("keep digits      : {out}");
    assert_eq!(out, "کتاب ١٢٣");

    // Turn letter unification off (the Arabic kaf stays).
    let keep_letters = Normalizer::builder().unify_letters(false).build();
    let out = keep_letters.normalize(text);
    println!("keep letters     : {out}");
    assert_eq!(out, "كتاب ۱۲۳");

    // Diacritics are removed by default. Disable that for text where they
    // matter. Here: "ک" + kasra (U+0650) + "ت".
    let marked = "ک\u{0650}ت";
    assert_eq!(default.normalize(marked), "کت");
    let keep_marks = Normalizer::builder().remove_diacritics(false).build();
    assert_eq!(keep_marks.normalize(marked), marked);

    // Opt-in steps: ZWNJ insertion and Persian punctuation.
    let full = Normalizer::builder()
        .smart_zwnj(true)
        .punctuation(true)
        .build();
    let out = full.normalize("می روم, کتاب ها?");
    println!("opt-in steps     : {out}");
    assert_eq!(out, "می\u{200C}روم، کتاب\u{200C}ها؟");

    // A normalizer is reusable: build once, apply to many inputs.
    let inputs = ["علي", "ايران", "کتاب"];
    let normalized: Vec<_> = inputs
        .iter()
        .map(|input| default.normalize(input))
        .collect();
    println!("batch            : {normalized:?}");
    assert_eq!(normalized, ["علی", "ایران", "کتاب"]);
}
