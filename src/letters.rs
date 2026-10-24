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

/// Replaces Arabic letter variants with their Persian equivalents.
///
/// Returns [`Cow::Borrowed`] when the input contains nothing to replace.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-3): enable this example once `unify` is implemented.
/// use persian_normalizer::letters;
///
/// assert_eq!(letters::unify("كي"), "کی");
/// ```
#[must_use]
pub fn unify(text: &str) -> Cow<'_, str> {
    // TODO(step-3): implement.
    Cow::Borrowed(text)
}
