//! Space and zero-width non-joiner (ZWNJ, U+200C) handling.
//!
//! Two independent steps live here:
//!
//! * [`clean`] removes whitespace and ZWNJ noise (on by default in the pipeline).
//! * [`smart_zwnj`] inserts ZWNJ where Persian orthography expects it
//!   (off by default, heuristic).

use alloc::borrow::Cow;
use alloc::string::String;

use crate::chars;

/// Cleans up spaces and misplaced ZWNJ.
///
/// Rules:
///
/// 1. Runs of the space character (U+0020) collapse into one space. Tabs,
///    newlines and no-break spaces are left unchanged, and the text is not
///    trimmed.
/// 2. Consecutive ZWNJ collapse into one.
/// 3. A ZWNJ next to a space or a newline (`\n`, `\r`) is removed.
/// 4. A ZWNJ at the start or end of the text is removed.
///
/// ZWNJ anywhere else is preserved.
///
/// Returns [`Cow::Borrowed`] when the input needs no changes. The function is
/// idempotent and never lengthens the text.
///
/// # Examples
///
/// ```
/// use persian_normalizer::spacing;
///
/// assert_eq!(spacing::clean("a   b  c"), "a b c");
///
/// // Repeated, leading and trailing ZWNJ are removed; a single one between
/// // letters is kept.
/// assert_eq!(
///     spacing::clean("\u{200C}x\u{200C}\u{200C}y\u{200C}"),
///     "x\u{200C}y",
/// );
/// ```
#[must_use]
pub fn clean(text: &str) -> Cow<'_, str> {
    if needs_cleaning(text) {
        Cow::Owned(clean_owned(text))
    } else {
        Cow::Borrowed(text)
    }
}

/// Returns `true` if `clean_owned` would change `text`.
///
/// A text is a fixed point of the cleaning rules exactly when none of the
/// patterns below occurs, so this check keeps the zero-copy fast path exact.
fn needs_cleaning(text: &str) -> bool {
    let mut previous: Option<char> = None;
    for c in text.chars() {
        let bad = matches!(
            (previous, c),
            // Two spaces in a row.
            (Some(' '), ' ')
            // A ZWNJ at the start, after a space or newline, or after a ZWNJ.
            | (None | Some(' ' | '\n' | '\r' | chars::ZWNJ), chars::ZWNJ)
            // A space or newline right after a ZWNJ.
            | (Some(chars::ZWNJ), ' ' | '\n' | '\r')
        );
        if bad {
            return true;
        }
        previous = Some(c);
    }
    // A trailing ZWNJ.
    previous == Some(chars::ZWNJ)
}

fn clean_owned(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    // The last character written to `out`.
    let mut last: Option<char> = None;
    // A ZWNJ that has been seen but not written yet. It is written only if a
    // regular character follows it.
    let mut pending_zwnj = false;

    for c in text.chars() {
        match c {
            chars::ZWNJ => {
                let after_boundary = matches!(last, None | Some(' ' | '\n' | '\r'));
                if !pending_zwnj && !after_boundary {
                    pending_zwnj = true;
                }
            }
            ' ' | '\n' | '\r' => {
                // A pending ZWNJ next to a space or newline is dropped.
                pending_zwnj = false;
                if c == ' ' && last == Some(' ') {
                    continue;
                }
                out.push(c);
                last = Some(c);
            }
            _ => {
                if pending_zwnj {
                    out.push(chars::ZWNJ);
                    pending_zwnj = false;
                }
                out.push(c);
                last = Some(c);
            }
        }
    }

    out
}

/// Inserts ZWNJ using a small, dictionary-free set of rules.
///
/// A single space (U+0020) is replaced by ZWNJ when it separates two words
/// (maximal runs of Persian letters) and one of these holds:
///
/// * the word before it is the verb prefix `می` or `نمی` and the word after it
///   has at least two letters (`می روم` becomes `می‌روم`),
/// * the word after it is the plural suffix `ها` or `های` and the word before
///   it has at least two letters (`کتاب ها` becomes `کتاب‌ها`).
///
/// The rules are heuristic and can produce false positives, which is why the
/// step is off by default in the pipeline. They are stable: each decision
/// depends only on the two words next to a space, which the step itself never
/// changes, so applying the function twice gives the same result as once.
///
/// Returns [`Cow::Borrowed`] when the input needs no changes.
///
/// # Examples
///
/// ```
/// use persian_normalizer::spacing;
///
/// // "می روم" (م ی, space, ر و م) becomes "می‌روم".
/// assert_eq!(
///     spacing::smart_zwnj("\u{0645}\u{06CC} \u{0631}\u{0648}\u{0645}"),
///     "\u{0645}\u{06CC}\u{200C}\u{0631}\u{0648}\u{0645}",
/// );
///
/// // "کتاب ها" (ک ت ا ب, space, ه ا) becomes "کتاب‌ها".
/// assert_eq!(
///     spacing::smart_zwnj("\u{06A9}\u{062A}\u{0627}\u{0628} \u{0647}\u{0627}"),
///     "\u{06A9}\u{062A}\u{0627}\u{0628}\u{200C}\u{0647}\u{0627}",
/// );
/// ```
#[must_use]
pub fn smart_zwnj(text: &str) -> Cow<'_, str> {
    if !text
        .char_indices()
        .any(|(index, c)| c == ' ' && joins_at(text, index))
    {
        return Cow::Borrowed(text);
    }

    let mut out = String::with_capacity(text.len());
    for (index, c) in text.char_indices() {
        if c == ' ' && joins_at(text, index) {
            out.push(chars::ZWNJ);
        } else {
            out.push(c);
        }
    }
    Cow::Owned(out)
}

// Word forms recognized by `smart_zwnj`. Both the Persian and the Arabic yeh
// are accepted so the step also works when letter unification is disabled.
const MI: &str = "\u{0645}\u{06CC}";
const MI_ARABIC: &str = "\u{0645}\u{064A}";
const NAMI: &str = "\u{0646}\u{0645}\u{06CC}";
const NAMI_ARABIC: &str = "\u{0646}\u{0645}\u{064A}";
const HA: &str = "\u{0647}\u{0627}";
const HAYE: &str = "\u{0647}\u{0627}\u{06CC}";
const HAYE_ARABIC: &str = "\u{0647}\u{0627}\u{064A}";

/// Returns `true` if the space at byte offset `space_index` should become a ZWNJ.
fn joins_at(text: &str, space_index: usize) -> bool {
    let (before, from_space) = text.split_at(space_index);
    // The space is a single byte, so everything after it starts at offset 1.
    let after = from_space.get(1..).unwrap_or_default();
    should_join(left_run(before), right_run(after))
}

fn should_join(left: &str, right: &str) -> bool {
    let verb_prefix =
        matches!(left, MI | MI_ARABIC | NAMI | NAMI_ARABIC) && right.chars().count() >= 2;
    let plural_suffix = matches!(right, HA | HAYE | HAYE_ARABIC) && left.chars().count() >= 2;
    verb_prefix || plural_suffix
}

/// The maximal run of letters at the end of `before`.
fn left_run(before: &str) -> &str {
    let start = before
        .char_indices()
        .rev()
        .take_while(|&(_, c)| chars::is_letter(c))
        .last()
        .map_or(before.len(), |(index, _)| index);
    before.get(start..).unwrap_or_default()
}

/// The maximal run of letters at the start of `after`.
fn right_run(after: &str) -> &str {
    let end = after
        .char_indices()
        .find(|&(_, c)| !chars::is_letter(c))
        .map_or(after.len(), |(index, _)| index);
    after.get(..end).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    const Z: char = chars::ZWNJ;

    #[test]
    fn clean_collapses_spaces_but_keeps_other_whitespace() {
        assert_eq!(clean("a  b"), "a b");
        assert_eq!(clean("a \t b"), "a \t b");
        assert_eq!(clean("a\u{00A0}\u{00A0}b"), "a\u{00A0}\u{00A0}b");
    }

    #[test]
    fn clean_does_not_trim() {
        assert!(matches!(clean(" a "), Cow::Borrowed(" a ")));
    }

    #[test]
    fn clean_removes_zwnj_at_edges_and_next_to_whitespace() {
        assert_eq!(clean(&format!("{Z}a")), "a");
        assert_eq!(clean(&format!("a{Z}")), "a");
        assert_eq!(clean(&format!("a{Z} b")), "a b");
        assert_eq!(clean(&format!("a {Z}b")), "a b");
        assert_eq!(clean(&format!("a{Z}\nb")), "a\nb");
        assert_eq!(clean(&format!("a\n{Z}b")), "a\nb");
    }

    #[test]
    fn clean_keeps_a_single_zwnj_between_characters() {
        let input = format!("a{Z}b");
        assert!(matches!(clean(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn clean_handles_zwnj_between_spaces() {
        assert_eq!(clean(&format!("a {Z} b")), "a b");
    }

    #[test]
    fn clean_collapses_repeated_zwnj() {
        assert_eq!(clean(&format!("a{Z}{Z}{Z}b")), format!("a{Z}b"));
    }

    #[test]
    fn clean_is_idempotent_on_awkward_input() {
        for input in [
            format!("{Z}{Z} a {Z}{Z} b {Z}"),
            format!("a{Z} {Z} {Z}b"),
            format!(" {Z} "),
            format!("{Z}"),
            String::new(),
        ] {
            let once = clean(&input).into_owned();
            assert!(matches!(clean(&once), Cow::Borrowed(_)), "{input:?}");
        }
    }

    #[test]
    fn smart_zwnj_joins_verb_prefixes() {
        assert_eq!(
            smart_zwnj(&format!("{MI} \u{0631}\u{0648}\u{0645}")),
            format!("{MI}{Z}\u{0631}\u{0648}\u{0645}")
        );
        assert_eq!(
            smart_zwnj(&format!("{NAMI} \u{0631}\u{0648}\u{0645}")),
            format!("{NAMI}{Z}\u{0631}\u{0648}\u{0645}")
        );
    }

    #[test]
    fn smart_zwnj_does_not_join_a_one_letter_following_word() {
        // "می و" ("wine and") must stay as it is.
        let input = format!("{MI} \u{0648}");
        assert!(matches!(smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn smart_zwnj_joins_plural_suffixes() {
        let word = "\u{06A9}\u{062A}\u{0627}\u{0628}";
        assert_eq!(
            smart_zwnj(&format!("{word} {HA}")),
            format!("{word}{Z}{HA}")
        );
        assert_eq!(
            smart_zwnj(&format!("{word} {HAYE}")),
            format!("{word}{Z}{HAYE}")
        );
    }

    #[test]
    fn smart_zwnj_requires_whole_words() {
        // "ها" is the start of a longer word: not a plural suffix.
        let word = "\u{06A9}\u{062A}\u{0627}\u{0628}";
        let longer = format!("{word} {HA}\u{0634}\u{0645}");
        assert!(matches!(smart_zwnj(&longer), Cow::Borrowed(_)));
    }

    #[test]
    fn smart_zwnj_ignores_non_letter_neighbours() {
        assert!(matches!(smart_zwnj("1 2"), Cow::Borrowed("1 2")));
        assert!(matches!(smart_zwnj(&format!("{MI} 123")), Cow::Borrowed(_)));
    }

    #[test]
    fn smart_zwnj_is_stable_on_its_own_output() {
        let input = format!("{MI} {MI} \u{0631}\u{0648}\u{0645}");
        let once = smart_zwnj(&input).into_owned();
        assert!(matches!(smart_zwnj(&once), Cow::Borrowed(_)));
    }

    #[test]
    fn runs_stop_at_non_letters() {
        assert_eq!(left_run("12 \u{0628}\u{0627}"), "\u{0628}\u{0627}");
        assert_eq!(left_run("\u{0628}\u{0627} 1"), "");
        assert_eq!(right_run("\u{0628}\u{0627}\u{200C}x"), "\u{0628}\u{0627}");
        assert_eq!(right_run(""), "");
    }
}
