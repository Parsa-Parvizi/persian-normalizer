//! Digit conversion between Persian, Arabic-Indic and Latin numerals.
//!
//! Only the three sets of ten digits are converted:
//!
//! | Set | Range |
//! |---|---|
//! | Persian `۰۱۲۳۴۵۶۷۸۹` | U+06F0 to U+06F9 |
//! | Arabic-Indic `٠١٢٣٤٥٦٧٨٩` | U+0660 to U+0669 |
//! | Latin `0123456789` | U+0030 to U+0039 |
//!
//! Decimal and thousands separators are not touched.

use alloc::borrow::Cow;

use crate::{chars, util, DigitStyle};

/// Converts every digit in `text` to the given [`DigitStyle`].
///
/// Digits from any of the three supported sets are converted, so mixed input
/// ends up in a single numeral system. [`DigitStyle::Keep`] returns the input
/// unchanged. Returns [`Cow::Borrowed`] when no digit needs converting.
///
/// Note that this also converts digits inside identifiers, URLs and code,
/// because the function has no notion of context.
///
/// # Examples
///
/// ```
/// use persian_normalizer::{digits, DigitStyle};
///
/// // Persian digits 1 2 3 (U+06F1..U+06F3) become Latin "123".
/// assert_eq!(digits::convert("\u{06F1}\u{06F2}\u{06F3}", DigitStyle::Latin), "123");
///
/// // Latin "123" becomes Persian digits.
/// assert_eq!(
///     digits::convert("123", DigitStyle::Persian),
///     "\u{06F1}\u{06F2}\u{06F3}",
/// );
/// ```
#[must_use]
pub fn convert(text: &str, style: DigitStyle) -> Cow<'_, str> {
    let zero = match style {
        DigitStyle::Persian => chars::PERSIAN_ZERO,
        DigitStyle::Arabic => chars::ARABIC_INDIC_ZERO,
        DigitStyle::Latin => chars::LATIN_ZERO,
        DigitStyle::Keep => return Cow::Borrowed(text),
    };

    util::map_chars(text, |c| {
        chars::digit_value(c)
            .and_then(|value| char::from_u32(zero + value))
            .unwrap_or(c)
    })
}
