//! Integration tests for `punctuation::normalize`.

use std::borrow::Cow;

use persian_normalizer::punctuation;

const COMMA: char = '\u{060C}'; // Arabic comma
const SEMICOLON: char = '\u{061B}'; // Arabic semicolon
const QUESTION: char = '\u{061F}'; // Arabic question mark

/// A short Persian word ("سی"), two letters.
const WORD: &str = "\u{0633}\u{06CC}";

#[test]
fn converts_each_mark_after_a_persian_letter() {
    assert_eq!(
        punctuation::normalize(&format!("{WORD},")),
        format!("{WORD}{COMMA}")
    );
    assert_eq!(
        punctuation::normalize(&format!("{WORD};")),
        format!("{WORD}{SEMICOLON}")
    );
    assert_eq!(
        punctuation::normalize(&format!("{WORD}?")),
        format!("{WORD}{QUESTION}")
    );
}

#[test]
fn converts_marks_in_running_text() {
    let input = format!("{WORD}, {WORD}; {WORD}?");
    let expected = format!("{WORD}{COMMA} {WORD}{SEMICOLON} {WORD}{QUESTION}");
    assert_eq!(punctuation::normalize(&input), expected);
}

#[test]
fn spaces_between_letter_and_mark_do_not_matter() {
    assert_eq!(
        punctuation::normalize(&format!("{WORD}  ,")),
        format!("{WORD}  {COMMA}")
    );
}

#[test]
fn leaves_numbers_untouched() {
    for input in [
        "1,234",
        "1,234.5",
        "12;34",
        "\u{06F1},\u{06F2}",
        "\u{0661},\u{0662}",
    ] {
        assert!(
            matches!(punctuation::normalize(input), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn leaves_latin_text_code_and_urls_untouched() {
    for input in [
        "hello, world?",
        "let a = [1, 2, 3];",
        "https://example.com/?q=1",
        "a?b:c",
    ] {
        assert!(
            matches!(punctuation::normalize(input), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn marks_without_a_preceding_letter_are_untouched() {
    for input in [",", ";", "?", " ,", "\n,", "(,", "\u{200C},"] {
        assert!(
            matches!(punctuation::normalize(input), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn only_the_first_of_several_marks_is_converted() {
    // After the first mark is converted, the second one follows a non-letter.
    assert_eq!(
        punctuation::normalize(&format!("{WORD},,")),
        format!("{WORD}{COMMA},")
    );
}

#[test]
fn existing_persian_marks_are_left_alone_and_borrowed() {
    let input = format!("{WORD}{COMMA} {WORD}{SEMICOLON} {WORD}{QUESTION}");
    assert!(matches!(punctuation::normalize(&input), Cow::Borrowed(s) if s == input));
}

#[test]
fn mixed_scripts_use_the_closest_preceding_character() {
    // A Latin word before the mark means no conversion, even in Persian text.
    let input = format!("{WORD} abc,");
    assert!(matches!(punctuation::normalize(&input), Cow::Borrowed(s) if s == input));
}

#[test]
fn multibyte_text_before_the_first_conversion_is_preserved() {
    assert_eq!(
        punctuation::normalize(&format!("\u{1F600} x {WORD}?")),
        format!("\u{1F600} x {WORD}{QUESTION}")
    );
}
