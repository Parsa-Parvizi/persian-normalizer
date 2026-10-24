//! Conversion of Latin punctuation to Persian punctuation.
//!
//! | From | To |
//! |---|---|
//! | `,` | `،` U+060C |
//! | `;` | `؛` U+061B |
//! | `?` | `؟` U+061F |
//!
//! The conversion is context-aware: a mark is replaced only when the preceding
//! non-space character is a Persian-script letter. Numbers such as `1,234`,
//! code and URLs are therefore left alone.

use alloc::borrow::Cow;

/// Converts `,`, `;` and `?` to their Persian forms after Persian-script letters.
///
/// Returns [`Cow::Borrowed`] when nothing needs converting.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-7): enable this example once `normalize` is implemented.
/// use persian_normalizer::punctuation;
///
/// assert_eq!(punctuation::normalize("سلام, چطوری?"), "سلام، چطوری؟");
/// assert_eq!(punctuation::normalize("1,234"), "1,234");
/// ```
#[must_use]
pub fn normalize(text: &str) -> Cow<'_, str> {
    // TODO(step-7): implement.
    Cow::Borrowed(text)
}
