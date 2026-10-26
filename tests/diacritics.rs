//! Integration tests for `diacritics::strip`.

use std::borrow::Cow;

use persian_normalizer::diacritics;

const KAF: char = '\u{06A9}';
const TEH: char = '\u{062A}';
const ALEF: char = '\u{0627}';
const BEH: char = '\u{0628}';

/// "kitab" written with the four letters above.
fn word() -> String {
    [KAF, TEH, ALEF, BEH].iter().collect()
}

#[test]
fn removes_every_documented_diacritic() {
    for code in 0x064B..=0x0652u32 {
        let mark = char::from_u32(code).unwrap();
        let input = format!("{KAF}{mark}{TEH}");
        assert_eq!(
            diacritics::strip(&input),
            format!("{KAF}{TEH}"),
            "U+{code:04X}"
        );
    }
}

#[test]
fn removes_superscript_alef_and_tatweel() {
    assert_eq!(diacritics::strip("a\u{0670}b"), "ab");
    assert_eq!(diacritics::strip("a\u{0640}\u{0640}b"), "ab");
}

#[test]
fn removes_marks_from_a_whole_word() {
    let marked = format!("{KAF}\u{0650}{TEH}\u{064E}{ALEF}{BEH}\u{064F}");
    assert_eq!(diacritics::strip(&marked), word());
}

#[test]
fn text_without_marks_is_borrowed() {
    let input = word();
    assert!(matches!(diacritics::strip(&input), Cow::Borrowed(s) if s == input));
    assert!(matches!(diacritics::strip(""), Cow::Borrowed("")));
}

#[test]
fn text_with_marks_is_owned() {
    assert!(matches!(diacritics::strip("a\u{064E}"), Cow::Owned(_)));
}

#[test]
fn other_combining_marks_are_left_alone() {
    // U+0653 (maddah above) and U+065F are outside the documented set.
    for input in ["a\u{0653}", "a\u{065F}", "a\u{06D6}"] {
        assert!(
            matches!(diacritics::strip(input), Cow::Borrowed(s) if s == input),
            "input: {input:?}"
        );
    }
}

#[test]
fn letters_next_to_the_removed_range_are_kept() {
    // U+064A (Arabic yeh) sits right before the range, U+0671 right after U+0670.
    let input = "\u{064A}\u{0671}";
    assert!(matches!(diacritics::strip(input), Cow::Borrowed(s) if s == input));
}

#[test]
fn surrounding_text_is_preserved() {
    assert_eq!(
        diacritics::strip("\u{1F600}a\u{064E}\u{200C}b\u{0651} 1"),
        "\u{1F600}a\u{200C}b 1"
    );
}

#[test]
fn removing_everything_yields_an_empty_string() {
    assert_eq!(diacritics::strip("\u{064E}\u{0640}\u{0651}"), "");
}
