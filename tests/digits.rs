//! Integration tests for `digits::convert`.

use std::borrow::Cow;

use persian_normalizer::{digits, DigitStyle};

const LATIN_ZERO: u32 = 0x0030;
const ARABIC_INDIC_ZERO: u32 = 0x0660;
const PERSIAN_ZERO: u32 = 0x06F0;

/// The ten digits of the numeral system whose zero is `zero`.
fn all_digits(zero: u32) -> String {
    (0..10).map(|i| digit(zero, i)).collect()
}

/// A single digit of the numeral system whose zero is `zero`.
fn digit(zero: u32, value: u32) -> char {
    char::from_u32(zero + value).unwrap()
}

const SETS: [(DigitStyle, u32); 3] = [
    (DigitStyle::Latin, LATIN_ZERO),
    (DigitStyle::Arabic, ARABIC_INDIC_ZERO),
    (DigitStyle::Persian, PERSIAN_ZERO),
];

#[test]
fn converts_between_every_pair_of_styles() {
    for (_, from_zero) in SETS {
        for (style, to_zero) in SETS {
            let input = all_digits(from_zero);
            let expected = all_digits(to_zero);
            assert_eq!(digits::convert(&input, style), expected.as_str());
        }
    }
}

#[test]
fn input_already_in_target_style_is_borrowed() {
    for (style, zero) in SETS {
        let input = all_digits(zero);
        assert!(
            matches!(digits::convert(&input, style), Cow::Borrowed(s) if s == input),
            "style: {style:?}"
        );
    }
}

#[test]
fn keep_returns_input_unchanged_and_borrowed() {
    for (_, zero) in SETS {
        let input = all_digits(zero);
        assert!(matches!(
            digits::convert(&input, DigitStyle::Keep),
            Cow::Borrowed(s) if s == input
        ));
    }
}

#[test]
fn mixed_numeral_systems_end_up_in_one_style() {
    let input: String = [
        digit(LATIN_ZERO, 1),
        digit(ARABIC_INDIC_ZERO, 2),
        digit(PERSIAN_ZERO, 3),
    ]
    .iter()
    .collect();
    assert_eq!(digits::convert(&input, DigitStyle::Latin), "123");
}

#[test]
fn separators_and_other_text_are_untouched() {
    // ASCII comma and dot stay as they are; letters are not digits.
    let input = "abc 1,2.3 def";
    let expected = format!(
        "abc {},{}.{} def",
        digit(PERSIAN_ZERO, 1),
        digit(PERSIAN_ZERO, 2),
        digit(PERSIAN_ZERO, 3),
    );
    assert_eq!(
        digits::convert(input, DigitStyle::Persian),
        expected.as_str()
    );
}

#[test]
fn neighbouring_code_points_are_not_digits() {
    // U+066A is the Arabic percent sign and U+06FA is a letter: neither is a digit.
    for input in ["\u{066A}", "\u{06FA}", "\u{065F}", "\u{06EF}"] {
        assert!(
            matches!(digits::convert(input, DigitStyle::Latin), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn empty_input_is_borrowed() {
    assert!(matches!(
        digits::convert("", DigitStyle::Persian),
        Cow::Borrowed("")
    ));
}
