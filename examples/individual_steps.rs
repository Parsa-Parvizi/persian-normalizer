//! Using each normalization step on its own.
//!
//! Every step is a pure function returning `Cow<str>`, so you can apply only
//! what you need.
//!
//! Run with: `cargo run --example individual_steps`

use std::borrow::Cow;

use persian_normalizer::{DigitStyle, digits, letters};

/// Describes whether a result reused the input or allocated a new string.
fn kind(text: &Cow<'_, str>) -> &'static str {
    match text {
        Cow::Borrowed(_) => "borrowed (no allocation)",
        Cow::Owned(_) => "owned (changed)",
    }
}

fn main() {
    // Letters: Arabic yeh (ي) and kaf (ك) become Persian yeh (ی) and kaf (ک).
    let out = letters::unify("علي كتاب");
    println!("letters::unify        : {out}  [{}]", kind(&out));
    assert_eq!(out, "علی کتاب");

    // Nothing to change: the input comes back borrowed.
    let out = letters::unify("علی کتاب");
    println!("letters::unify (clean): {out}  [{}]", kind(&out));
    assert!(matches!(out, Cow::Borrowed(_)));

    // Digits: any numeral system can be converted to any other.
    let text = "1 ٢ ۳";
    for (name, style, expected) in [
        ("persian", DigitStyle::Persian, "۱ ۲ ۳"),
        ("arabic ", DigitStyle::Arabic, "١ ٢ ٣"),
        ("latin  ", DigitStyle::Latin, "1 2 3"),
        ("keep   ", DigitStyle::Keep, "1 ٢ ۳"),
    ] {
        let out = digits::convert(text, style);
        println!("digits::convert {name}: {out}  [{}]", kind(&out));
        assert_eq!(out, expected);
    }
}