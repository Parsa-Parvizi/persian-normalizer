//! Property-based tests for the guarantees in `docs/ARCHITECTURE.md` (D-20):
//! idempotence, totality, the zero-copy fast path and bounded output size.
//!
//! Note on `prop_assert!`: the macro turns the asserted expression into a
//! *format string* for its failure message. Expressions containing braces, such
//! as `'\u{200C}'`, would be misread as format placeholders. Conditions that
//! need such literals therefore live in the helper functions below, and every
//! assertion carries an explicit message.

use std::borrow::Cow;

use persian_normalizer::{
    diacritics, digits, letters, normalize, punctuation, spacing, DigitStyle, Normalizer,
};
use proptest::prelude::*;

const ZWNJ: char = '\u{200C}';

/// Strings dominated by Arabic-script characters (including marks, digits and
/// letters), ZWNJ, whitespace, ASCII and the punctuation the steps care about.
fn persian_like() -> impl Strategy<Value = String> {
    "[\u{0600}-\u{06FF}\u{200C} \n\r a-z0-9,;?]{0,64}"
}

fn all_digit_styles() -> [DigitStyle; 4] {
    [
        DigitStyle::Persian,
        DigitStyle::Arabic,
        DigitStyle::Latin,
        DigitStyle::Keep,
    ]
}

/// `true` if `text` still contains a diacritic or tatweel that `strip` removes.
fn contains_diacritic(text: &str) -> bool {
    text.chars()
        .any(|c| matches!(c, '\u{064B}'..='\u{0652}' | '\u{0670}' | '\u{0640}'))
}

/// `true` if `text` contains an Arabic-Indic or Persian digit.
fn contains_non_latin_digit(text: &str) -> bool {
    text.chars()
        .any(|c| matches!(c, '\u{0660}'..='\u{0669}' | '\u{06F0}'..='\u{06F9}'))
}

/// `true` if `text` breaks any of the `spacing::clean` rules.
fn has_spacing_noise(text: &str) -> bool {
    text.contains("  ")
        || text.contains("\u{200C}\u{200C}")
        || [
            "\u{200C} ",
            " \u{200C}",
            "\u{200C}\n",
            "\n\u{200C}",
            "\u{200C}\r",
            "\r\u{200C}",
        ]
        .iter()
        .any(|pattern| text.contains(pattern))
        || text.starts_with(ZWNJ)
        || text.ends_with(ZWNJ)
}

/// Applying a step twice must equal applying it once, and the second
/// application must take the zero-copy path.
fn assert_stable(step: impl Fn(&str) -> Cow<'_, str>, input: &str) -> Result<(), TestCaseError> {
    let once = step(input).into_owned();
    let twice = step(&once);
    prop_assert!(
        matches!(&twice, Cow::Borrowed(s) if *s == once),
        "second pass changed {once:?} into {twice:?}"
    );
    Ok(())
}

proptest! {
    #[test]
    fn letters_unify_is_stable(s in persian_like()) {
        assert_stable(letters::unify, &s)?;
    }

    #[test]
    fn diacritics_strip_is_stable_and_leaves_none(s in persian_like()) {
        assert_stable(diacritics::strip, &s)?;
        let out = diacritics::strip(&s);
        prop_assert!(!contains_diacritic(&out), "diacritics left in {out:?}");
        prop_assert!(out.len() <= s.len(), "output grew: {s:?} -> {out:?}");
    }

    #[test]
    fn spacing_clean_is_stable_and_leaves_no_noise(s in persian_like()) {
        assert_stable(spacing::clean, &s)?;
        let out = spacing::clean(&s);
        prop_assert!(!has_spacing_noise(&out), "noise left in {out:?} (input {s:?})");
        prop_assert!(out.len() <= s.len(), "output grew: {s:?} -> {out:?}");
    }

    #[test]
    fn spacing_smart_zwnj_is_stable_and_bounded(s in persian_like()) {
        assert_stable(spacing::smart_zwnj, &s)?;
        let out = spacing::smart_zwnj(&s);
        prop_assert!(
            out.len() <= s.len().saturating_mul(3),
            "output too large: {s:?} -> {out:?}"
        );
    }

    #[test]
    fn punctuation_normalize_is_stable(s in persian_like()) {
        assert_stable(punctuation::normalize, &s)?;
    }

    #[test]
    fn digits_convert_is_stable_for_every_style(s in persian_like()) {
        for style in all_digit_styles() {
            assert_stable(|text| digits::convert(text, style), &s)?;
        }
    }

    #[test]
    fn digits_convert_leaves_only_latin_digits_when_asked(s in persian_like()) {
        let out = digits::convert(&s, DigitStyle::Latin);
        prop_assert!(!contains_non_latin_digit(&out), "non-Latin digits left in {out:?}");
    }

    #[test]
    fn default_pipeline_is_stable_on_persian_like_text(s in persian_like()) {
        assert_stable(normalize, &s)?;
    }

    #[test]
    fn default_pipeline_is_total_and_stable_on_arbitrary_text(s in any::<String>()) {
        assert_stable(normalize, &s)?;
    }

    #[test]
    fn fully_enabled_pipeline_is_idempotent_and_bounded(s in persian_like()) {
        let normalizer = Normalizer::builder()
            .smart_zwnj(true)
            .punctuation(true)
            .build();
        let once = normalizer.normalize(&s).into_owned();
        let twice = normalizer.normalize(&once).into_owned();
        prop_assert_eq!(&once, &twice);
        prop_assert!(
            once.len() <= s.len().saturating_mul(3),
            "output too large: {s:?} -> {once:?}"
        );
    }

    #[test]
    fn fully_enabled_pipeline_is_idempotent_on_arbitrary_text(s in any::<String>()) {
        let normalizer = Normalizer::builder()
            .smart_zwnj(true)
            .punctuation(true)
            .build();
        let once = normalizer.normalize(&s).into_owned();
        let twice = normalizer.normalize(&once).into_owned();
        prop_assert_eq!(once, twice);
    }
}
