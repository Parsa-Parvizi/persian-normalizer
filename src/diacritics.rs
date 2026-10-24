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
//! Removal is lossy. Do not use it on text where diacritics carry meaning,
//! for example Quranic text.

use alloc::borrow::Cow;

/// Removes diacritics and tatweel from `text`.
///
/// Returns [`Cow::Borrowed`] when the input contains none of them.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-5): enable this example once `strip` is implemented.
/// use persian_normalizer::diacritics;
///
/// assert_eq!(diacritics::strip("ک\u{0650}تاب"), "کتاب");
/// ```
#[must_use]
pub fn strip(text: &str) -> Cow<'_, str> {
    // TODO(step-5): implement.
    Cow::Borrowed(text)
}
