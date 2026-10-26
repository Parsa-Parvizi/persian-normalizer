//! Conversion of Latin punctuation to Persian punctuation.
//!
//! | From | To |
//! |---|---|
//! | `,` | `،` U+060C |
//! | `;` | `؛` U+061B |
//! | `?` | `؟` U+061F |
//!
//! The conversion is context-aware: a mark is replaced only when the preceding
//! character, ignoring spaces (U+0020), is a Persian-script letter. Numbers
//! such as `1,234`, code and URLs are therefore left alone.

use alloc::borrow::Cow;
use alloc::string::String;

use crate::chars;

/// Converts `,`, `;` and `?` to their Persian forms after Persian-script letters.
///
/// Returns [`Cow::Borrowed`] when nothing needs converting. The function is
/// idempotent: the Persian marks are never converted again.
///
/// # Examples
///
/// ```
/// use persian_normalizer::punctuation;
///
/// // After a Persian letter the marks are converted ("س" is U+0633, "ی" is U+06CC).
/// assert_eq!(punctuation::normalize("\u{0633}\u{06CC},"), "\u{0633}\u{06CC}\u{060C}");
/// assert_eq!(punctuation::normalize("\u{0633}\u{06CC}?"), "\u{0633}\u{06CC}\u{061F}");
///
/// // After digits or Latin text they are left alone.
/// assert_eq!(punctuation::normalize("1,234"), "1,234");
/// assert_eq!(punctuation::normalize("hello, world?"), "hello, world?");
/// ```
#[must_use]
pub fn normalize(text: &str) -> Cow<'_, str> {
    // Find the first mark that has to be replaced. Spaces do not change the
    // "preceded by a letter" state; every other character does.
    let mut after_letter = false;
    let mut first_change = None;
    for (index, c) in text.char_indices() {
        if c == ' ' {
            continue;
        }
        if after_letter && persian_mark(c).is_some() {
            first_change = Some(index);
            break;
        }
        after_letter = chars::is_letter(c);
    }

    let Some(first_change) = first_change else {
        return Cow::Borrowed(text);
    };

    // `first_change` comes from `char_indices`, so it is always a valid boundary.
    let (unchanged, rest) = text.split_at(first_change);

    let mut out = String::with_capacity(text.len());
    out.push_str(unchanged);

    // At `first_change` the previous non-space character is a letter.
    let mut after_letter = true;
    for c in rest.chars() {
        if c == ' ' {
            out.push(c);
            continue;
        }
        match persian_mark(c) {
            Some(mark) if after_letter => {
                out.push(mark);
                // The Persian mark is not a letter.
                after_letter = false;
            }
            _ => {
                out.push(c);
                after_letter = chars::is_letter(c);
            }
        }
    }
    Cow::Owned(out)
}

/// The Persian mark that replaces `c`, if `c` is one of the Latin marks.
const fn persian_mark(c: char) -> Option<char> {
    match c {
        ',' => Some('\u{060C}'),
        ';' => Some('\u{061B}'),
        '?' => Some('\u{061F}'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SI: &str = "\u{0633}\u{06CC}"; // two Persian letters

    #[test]
    fn replaces_each_mark_after_a_letter() {
        assert_eq!(
            normalize(&alloc::format!("{SI},")),
            alloc::format!("{SI}\u{060C}")
        );
        assert_eq!(
            normalize(&alloc::format!("{SI};")),
            alloc::format!("{SI}\u{061B}")
        );
        assert_eq!(
            normalize(&alloc::format!("{SI}?")),
            alloc::format!("{SI}\u{061F}")
        );
    }

    #[test]
    fn spaces_between_the_letter_and_the_mark_are_ignored() {
        assert_eq!(
            normalize(&alloc::format!("{SI} ,")),
            alloc::format!("{SI} \u{060C}")
        );
    }

    #[test]
    fn other_characters_reset_the_context() {
        assert!(matches!(normalize("1,2"), Cow::Borrowed("1,2")));
        assert!(matches!(normalize("a,b"), Cow::Borrowed("a,b")));
        assert!(matches!(normalize(","), Cow::Borrowed(",")));
        assert!(matches!(normalize(""), Cow::Borrowed("")));
    }

    #[test]
    fn a_converted_mark_does_not_enable_the_next_one() {
        let out = normalize(&alloc::format!("{SI},,")).into_owned();
        assert_eq!(out, alloc::format!("{SI}\u{060C},"));
        assert!(matches!(normalize(&out), Cow::Borrowed(_)));
    }

    #[test]
    fn handles_multibyte_text_before_the_first_change() {
        // The input must outlive `out`, which borrows from it when unchanged.
        let input = alloc::format!("\u{1F600}x {SI}?");
        let out = normalize(&input);
        assert_eq!(out, alloc::format!("\u{1F600}x {SI}\u{061F}"));
    }
}
