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

use crate::DigitStyle;

/// Converts every digit in `text` to the given [`DigitStyle`].
///
/// [`DigitStyle::Keep`] returns the input unchanged. Returns
/// [`Cow::Borrowed`] when no digit needs converting.
///
/// # Examples
///
/// ```ignore
/// // TODO(step-4): enable this example once `convert` is implemented.
/// use persian_normalizer::{digits, DigitStyle};
///
/// assert_eq!(digits::convert("123 ٤٥٦", DigitStyle::Persian), "۱۲۳ ۴۵۶");
/// assert_eq!(digits::convert("۱۲۳", DigitStyle::Latin), "123");
/// ```
#[must_use]
pub fn convert(text: &str, style: DigitStyle) -> Cow<'_, str> {
    // TODO(step-4): implement.
    let _ = style;
    Cow::Borrowed(text)
}
