//! Integration tests for the full pipeline.

use std::borrow::Cow;

use persian_normalizer::{normalize, DigitStyle, Normalizer};

const Z: char = '\u{200C}'; // ZWNJ

#[test]
fn default_pipeline_unifies_letters_and_digits() {
    // Arabic kaf + teh + alef + beh, a space, then Arabic-Indic digits 1 2 3.
    let input = "\u{0643}\u{062A}\u{0627}\u{0628} \u{0661}\u{0662}\u{0663}";
    // Persian kaf + teh + alef + beh, a space, then Persian digits 1 2 3.
    let expected = "\u{06A9}\u{062A}\u{0627}\u{0628} \u{06F1}\u{06F2}\u{06F3}";
    assert_eq!(normalize(input), expected);
}

#[test]
fn default_pipeline_removes_diacritics_and_cleans_spacing() {
    // Kaf, kasra, teh, alef, beh, then a tatweel standing alone between spaces,
    // then a doubled ZWNJ.
    let input = format!("\u{0643}\u{0650}\u{062A}\u{0627}\u{0628} \u{0640} {Z}{Z}x");
    // The kasra and tatweel are removed, the spaces collapse, and the ZWNJ
    // after a space is dropped.
    let expected = "\u{06A9}\u{062A}\u{0627}\u{0628} x";
    assert_eq!(normalize(&input), expected);
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

#[test]
fn disabling_diacritics_keeps_them() {
    let normalizer = Normalizer::builder().remove_diacritics(false).build();
    let input = "\u{06A9}\u{0650}";
    assert!(matches!(normalizer.normalize(input), Cow::Borrowed(s) if s == input));
}

#[test]
fn disabling_spacing_keeps_extra_spaces() {
    let normalizer = Normalizer::builder().fix_spacing(false).build();
    let input = "a   b";
    assert!(matches!(normalizer.normalize(input), Cow::Borrowed(s) if s == input));
}

#[test]
fn smart_zwnj_runs_after_spacing_cleanup() {
    let normalizer = Normalizer::builder().smart_zwnj(true).build();
    // "می" + two spaces + "روم": cleanup first collapses the spaces, and only
    // then can the single remaining space become a ZWNJ.
    let input = "\u{0645}\u{06CC}  \u{0631}\u{0648}\u{0645}";
    let expected = format!("\u{0645}\u{06CC}{Z}\u{0631}\u{0648}\u{0645}");
    assert_eq!(normalizer.normalize(input), expected);
}

#[test]
fn punctuation_runs_when_enabled() {
    let normalizer = Normalizer::builder().punctuation(true).build();
    let input = "\u{0633}\u{06CC},";
    assert_eq!(normalizer.normalize(input), "\u{0633}\u{06CC}\u{060C}");
}

#[test]
fn punctuation_sees_text_after_spacing_cleanup() {
    // Regression test (ARCHITECTURE.md, D-29): a letter, a space, a ZWNJ, a
    // space and a comma. Cleanup removes the ZWNJ, so the comma must be
    // converted in the *first* pass, and a second pass must change nothing.
    let normalizer = Normalizer::builder().punctuation(true).build();
    let input = format!("\u{0628} {Z} ,");
    let once = normalizer.normalize(&input).into_owned();
    assert_eq!(once, "\u{0628} \u{060C}");
    assert!(matches!(normalizer.normalize(&once), Cow::Borrowed(_)));
}

#[test]
fn punctuation_is_off_by_default() {
    let input = "\u{0633}\u{06CC},";
    assert!(matches!(normalize(input), Cow::Borrowed(s) if s == input));
}

#[test]
fn everything_enabled_produces_a_clean_result() {
    let normalizer = Normalizer::builder()
        .smart_zwnj(true)
        .punctuation(true)
        .build();
    // Arabic kaf-based "کتاب" + space + "ها", followed by an ASCII comma, then
    // Arabic-Indic digits.
    let input = "\u{0643}\u{062A}\u{0627}\u{0628} \u{0647}\u{0627}, \u{0661}\u{0662}";
    let expected =
        format!("\u{06A9}\u{062A}\u{0627}\u{0628}{Z}\u{0647}\u{0627}\u{060C} \u{06F1}\u{06F2}");
    assert_eq!(normalizer.normalize(input), expected);
}
