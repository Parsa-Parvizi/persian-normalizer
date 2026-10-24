//! Option types shared by the normalizer and the individual steps.

/// The numeral system that digits are converted to.
///
/// Used by [`digits::convert`](crate::digits::convert) and
/// [`NormalizerBuilder::digits`](crate::NormalizerBuilder::digits).
///
/// This enum is `#[non_exhaustive]`: new styles may be added in a minor
/// release, so `match` expressions on it need a wildcard arm.
///
/// # Examples
///
/// ```
/// use persian_normalizer::DigitStyle;
///
/// assert_eq!(DigitStyle::default(), DigitStyle::Persian);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DigitStyle {
    /// Persian digits `۰۱۲۳۴۵۶۷۸۹` (U+06F0 to U+06F9). This is the default.
    #[default]
    Persian,
    /// Arabic-Indic digits `٠١٢٣٤٥٦٧٨٩` (U+0660 to U+0669).
    Arabic,
    /// ASCII digits `0123456789`.
    Latin,
    /// Leave digits unchanged.
    Keep,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_persian() {
        assert_eq!(DigitStyle::default(), DigitStyle::Persian);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let json = serde_json::to_string(&DigitStyle::Latin).unwrap();
        assert_eq!(json, "\"Latin\"");
        let back: DigitStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(back, DigitStyle::Latin);
    }
}
