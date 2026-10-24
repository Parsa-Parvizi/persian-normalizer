//! Space and zero-width non-joiner (ZWNJ, U+200C) handling.
//!
//! Two independent steps live here:
//!
//! * [`clean`] removes whitespace and ZWNJ noise (on by default in the pipeline).
//! * [`smart_zwnj`] inserts ZWNJ where Persian orthography expects it
//!   (off by default, heuristic).

use alloc::borrow::Cow;

/// Cleans up spaces and misplaced ZWNJ.
///
/// Rules:
///
/// 1. Runs of the space character (U+0020) collapse into one space. Tabs,
///    newlines and no-break spaces are left unchanged, and the text is not
///    trimmed.
/// 2. Consecutive ZWNJ collapse into one.
/// 3. A ZWNJ next to a space or newline is removed.
/// 4. A ZWNJ at the start or end of the text is removed.
///
/// ZWNJ anywhere else is preserved.
///
/// Returns [`Cow::Borrowed`] when the input needs no changes.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-6): enable this example once `clean` is implemented.
/// use persian_normalizer::spacing;
///
/// assert_eq!(spacing::clean("این   یک  متن"), "این یک متن");
/// assert_eq!(spacing::clean("می\u{200C}\u{200C}روم"), "می\u{200C}روم");
/// ```
#[must_use]
pub fn clean(text: &str) -> Cow<'_, str> {
    // TODO(step-6): implement.
    Cow::Borrowed(text)
}

/// Inserts ZWNJ using a small, dictionary-free set of rules.
///
/// Version 0.1 handles only:
///
/// * the verb prefixes `می` and `نمی` followed by a word
///   (`می روم` becomes `می‌روم`),
/// * the plural suffixes `ها` and `های` preceded by a word
///   (`کتاب ها` becomes `کتاب‌ها`).
///
/// The rules are heuristic and can produce false positives, which is why the
/// step is off by default in the pipeline. Output is guaranteed to be stable:
/// applying the function twice gives the same result as applying it once.
///
/// Returns [`Cow::Borrowed`] when the input needs no changes.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-6): enable this example once `smart_zwnj` is implemented.
/// use persian_normalizer::spacing;
///
/// assert_eq!(spacing::smart_zwnj("می روم"), "می\u{200C}روم");
/// assert_eq!(spacing::smart_zwnj("کتاب ها"), "کتاب\u{200C}ها");
/// ```
#[must_use]
pub fn smart_zwnj(text: &str) -> Cow<'_, str> {
    // TODO(step-6): implement.
    Cow::Borrowed(text)
}
