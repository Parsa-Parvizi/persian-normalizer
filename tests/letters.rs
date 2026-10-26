//! Integration tests for `letters::unify`.
//!
//! Characters are written as escapes so each test is unambiguous about which
//! code point it exercises.

use std::borrow::Cow;

use persian_normalizer::letters;

#[test]
fn maps_each_arabic_letter_to_its_persian_form() {
    let cases = [
        ("\u{064A}", "\u{06CC}"), // Arabic yeh        -> Persian yeh
        ("\u{0649}", "\u{06CC}"), // Alef maksura      -> Persian yeh
        ("\u{0643}", "\u{06A9}"), // Arabic kaf        -> Persian kaf
        ("\u{0629}", "\u{0647}"), // Teh marbuta       -> Heh
    ];
    for (input, expected) in cases {
        assert_eq!(letters::unify(input), expected, "input: {input:?}");
    }
}

#[test]
fn persian_letters_are_left_alone_and_borrowed() {
    let input = "\u{06CC}\u{06A9}\u{0647}";
    assert!(matches!(letters::unify(input), Cow::Borrowed(s) if s == input));
}

#[test]
fn alef_and_hamza_variants_are_not_unified() {
    for input in ["\u{0623}", "\u{0625}", "\u{0671}", "\u{0624}", "\u{0626}"] {
        assert!(
            matches!(letters::unify(input), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn surrounding_text_is_preserved() {
    // Latin text, a digit, ZWNJ and an emoji around an Arabic kaf.
    let input = "ab1\u{200C}\u{0643}\u{1F600}z";
    let expected = "ab1\u{200C}\u{06A9}\u{1F600}z";
    assert_eq!(letters::unify(input), expected);
}

#[test]
fn replaces_every_occurrence() {
    let input = "\u{0643}\u{064A}\u{0643}\u{064A}";
    let expected = "\u{06A9}\u{06CC}\u{06A9}\u{06CC}";
    assert_eq!(letters::unify(input), expected);
}

#[test]
fn empty_and_ascii_inputs_are_borrowed() {
    assert!(matches!(letters::unify(""), Cow::Borrowed("")));
    assert!(matches!(letters::unify("hello"), Cow::Borrowed("hello")));
}

#[test]
fn changed_input_is_owned() {
    assert!(matches!(letters::unify("\u{0643}"), Cow::Owned(_)));
}
