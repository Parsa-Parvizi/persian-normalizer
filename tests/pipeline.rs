//! Integration tests for the full default pipeline.

use std::borrow::Cow;

use persian_normalizer::{DigitStyle, Normalizer, normalize};

#[test]
fn default_pipeline_unifies_letters_and_digits() {
    // Arabic kaf + teh + alef + beh, a space, then Arabic-Indic digits 1 2 3.
    let input = "\u{0643}\u{062A}\u{0627}\u{0628} \u{0661}\u{0662}\u{0663}";
    // Persian kaf + teh + alef + beh, a space, then Persian digits 1 2 3.
    let expected = "\u{06A9}\u{062A}\u{0627}\u{0628} \u{06F1}\u{06F2}\u{06F3}";
    assert_eq!(normalize(input), expected);
}

#[test]
fn normalized_output_is_returned_borrowed_on_second_pass() {
    let input = "\u{0643}\u{062A}\u{0627}\u{0628} \u{0661}\u{0662}\u{0663}";
    let once = normalize(input).into_owned();
    assert!(matches!(normalize(&once), Cow::Borrowed(s) if s == once));
}

#[test]
fn custom_digit_style_is_respected() {
    let normalizer = Normalizer::builder().digits(DigitStyle::Latin).build();
    let input = "\u{06F1}\u{06F2}\u{06F3}";
    assert_eq!(normalizer.normalize(input), "123");
}

#[test]
fn disabling_letters_leaves_arabic_letters_in_place() {
    let normalizer = Normalizer::builder()
        .unify_letters(false)
        .digits(DigitStyle::Keep)
        .build();
    let input = "\u{0643}\u{064A}";
    assert!(matches!(normalizer.normalize(input), Cow::Borrowed(s) if s == input));
}