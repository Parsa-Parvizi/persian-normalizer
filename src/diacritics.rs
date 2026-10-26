//! Removal of diacritics (harakat) and tatweel.
//!
//! Removed code points:
//!
//! | Code points | Meaning |
//! |---|---|
//! | U+064B to U+0652 | tanwin, fatha, damma, kasra, shadda, sukun |
//! | U+0670 | superscript alef |
//! | U+0640 | tatweel (kashida) |
//!
//! All other combining marks are left untouched. Removal is lossy. Do not use
//! it on text where diacritics carry meaning, for example Quranic text.

use alloc::borrow::Cow;

use crate::{chars, util};

/// Removes diacritics and tatweel from `text`.
///
/// Returns [`Cow::Borrowed`] when the input contains none of them.
///
/// The function is idempotent and never lengthens the text.
///
/// # Examples
///
/// ```
/// use persian_normalizer::diacritics;
///
/// // Kasra (U+0650) after the first letter is removed.
/// assert_eq!(diacritics::strip("\u{06A9}\u{0650}\u{062A}\u{0627}\u{0628}"), "\u{06A9}\u{062A}\u{0627}\u{0628}");
///
/// // A tatweel (U+0640) stretching a word is removed too.
/// assert_eq!(diacritics::strip("\u{0633}\u{0640}\u{0644}\u{0627}\u{0645}"), "\u{0633}\u{0644}\u{0627}\u{0645}");
/// ```
#[must_use]
pub fn strip(text: &str) -> Cow<'_, str> {
    util::filter_chars(text, |c| !chars::is_diacritic(c))
}
