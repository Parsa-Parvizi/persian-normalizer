//! Arabic to Persian letter unification.
//!
//! | From | To |
//! |---|---|
//! | `ي` U+064A | `ی` U+06CC |
//! | `ى` U+0649 | `ی` U+06CC |
//! | `ك` U+0643 | `ک` U+06A9 |
//! | `ة` U+0629 | `ه` U+0647 |
//!
//! Alef and hamza variants (`أ`, `إ`, `ٱ`, `ؤ`, `ئ`) are intentionally left
//! unchanged because they carry meaning in Persian words.

use alloc::borrow::Cow;

use crate::{chars, util};

/// Replaces Arabic letter variants with their Persian equivalents.
///
/// Returns [`Cow::Borrowed`] when the input contains nothing to replace.
///
/// The function is idempotent: none of the replacement letters is itself
/// replaced.
///
/// # Examples
///
/// ```
/// use persian_normalizer::letters;
///
/// // Arabic yeh (U+064A) and kaf (U+0643) become Persian yeh and kaf.
/// let input = "\u{064A}\u{0643}";
/// assert_eq!(letters::unify(input), "\u{06CC}\u{06A9}");
/// ```
#[must_use]
pub fn unify(text: &str) -> Cow<'_, str> {
    util::map_chars(text, unify_char)
}

const fn unify_char(c: char) -> char {
    match c {
        chars::ARABIC_YEH | chars::ALEF_MAKSURA => chars::PERSIAN_YEH,
        chars::ARABIC_KAF => chars::PERSIAN_KAF,
        chars::TEH_MARBUTA => chars::HEH,
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unify_char_is_idempotent_for_every_target() {
        for c in [chars::PERSIAN_YEH, chars::PERSIAN_KAF, chars::HEH] {
            assert_eq!(unify_char(c), c);
        }
    }

    #[test]
    fn unify_char_maps_all_sources() {
        assert_eq!(unify_char(chars::ARABIC_YEH), chars::PERSIAN_YEH);
        assert_eq!(unify_char(chars::ALEF_MAKSURA), chars::PERSIAN_YEH);
        assert_eq!(unify_char(chars::ARABIC_KAF), chars::PERSIAN_KAF);
        assert_eq!(unify_char(chars::TEH_MARBUTA), chars::HEH);
    }
}
