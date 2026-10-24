//! The configurable [`Normalizer`] and its [`NormalizerBuilder`].

use alloc::borrow::Cow;

use crate::{diacritics, digits, letters, punctuation, spacing, util, DigitStyle};

/// A reusable, configured text normalizer.
///
/// Build one with [`Normalizer::builder`], or use [`Normalizer::default`] for
/// the default configuration. A normalizer is cheap to create, `Clone`,
/// `Send + Sync`, and meant to be built once and reused.
///
/// Steps always run in a fixed order; see the [crate docs](crate#pipeline).
///
/// # Defaults
///
/// | Step | Default |
/// |---|---|
/// | Letter unification | on |
/// | Diacritics and tatweel removal | on |
/// | Digits | [`DigitStyle::Persian`] |
/// | Space and ZWNJ cleanup | on |
/// | Smart ZWNJ insertion | off |
/// | Persian punctuation | off |
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use persian_normalizer::Normalizer;
///
/// let normalizer = Normalizer::default();
/// assert!(matches!(normalizer.normalize("کتاب"), Cow::Borrowed(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "private on/off switches, one per pipeline step"
)]
pub struct Normalizer {
    unify_letters: bool,
    remove_diacritics: bool,
    digits: DigitStyle,
    fix_spacing: bool,
    smart_zwnj: bool,
    punctuation: bool,
}

impl Default for Normalizer {
    fn default() -> Self {
        Self {
            unify_letters: true,
            remove_diacritics: true,
            digits: DigitStyle::Persian,
            fix_spacing: true,
            smart_zwnj: false,
            punctuation: false,
        }
    }
}

impl Normalizer {
    /// Starts building a normalizer from the default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use persian_normalizer::{DigitStyle, Normalizer};
    ///
    /// let normalizer = Normalizer::builder()
    ///     .digits(DigitStyle::Latin)
    ///     .smart_zwnj(true)
    ///     .build();
    /// # let _ = normalizer;
    /// ```
    #[must_use]
    pub fn builder() -> NormalizerBuilder {
        NormalizerBuilder::default()
    }

    /// Normalizes `text` according to this configuration.
    ///
    /// Returns [`Cow::Borrowed`] when no step changes the input, so already
    /// normalized text costs no allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use persian_normalizer::Normalizer;
    ///
    /// let out = Normalizer::default().normalize("سلام");
    /// assert!(matches!(out, Cow::Borrowed("سلام")));
    /// ```
    #[must_use]
    pub fn normalize<'a>(&self, text: &'a str) -> Cow<'a, str> {
        let mut out = Cow::Borrowed(text);

        if self.unify_letters {
            out = util::apply(out, letters::unify);
        }
        if self.remove_diacritics {
            out = util::apply(out, diacritics::strip);
        }
        if self.digits != DigitStyle::Keep {
            let style = self.digits;
            out = util::apply(out, |s| digits::convert(s, style));
        }
        if self.punctuation {
            out = util::apply(out, punctuation::normalize);
        }
        if self.fix_spacing {
            out = util::apply(out, spacing::clean);
        }
        if self.smart_zwnj {
            out = util::apply(out, spacing::smart_zwnj);
        }

        out
    }
}

/// Builder for [`Normalizer`].
///
/// Obtained from [`Normalizer::builder`]. Every method consumes and returns the
/// builder so calls can be chained; [`build`](Self::build) is infallible.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NormalizerBuilder {
    config: Normalizer,
}

impl NormalizerBuilder {
    /// Enables or disables Arabic to Persian letter unification.
    ///
    /// Default: enabled. See [`letters::unify`].
    #[must_use]
    pub fn unify_letters(mut self, enabled: bool) -> Self {
        self.config.unify_letters = enabled;
        self
    }

    /// Enables or disables removal of diacritics and tatweel.
    ///
    /// Default: enabled. Removal is lossy; disable it for text where
    /// diacritics carry meaning. See [`diacritics::strip`].
    #[must_use]
    pub fn remove_diacritics(mut self, enabled: bool) -> Self {
        self.config.remove_diacritics = enabled;
        self
    }

    /// Sets the target digit style.
    ///
    /// Default: [`DigitStyle::Persian`]. Use [`DigitStyle::Keep`] to leave
    /// digits unchanged. See [`digits::convert`].
    #[must_use]
    pub fn digits(mut self, style: DigitStyle) -> Self {
        self.config.digits = style;
        self
    }

    /// Enables or disables space and ZWNJ cleanup.
    ///
    /// Default: enabled. See [`spacing::clean`].
    #[must_use]
    pub fn fix_spacing(mut self, enabled: bool) -> Self {
        self.config.fix_spacing = enabled;
        self
    }

    /// Enables or disables heuristic ZWNJ insertion.
    ///
    /// Default: disabled. See [`spacing::smart_zwnj`].
    #[must_use]
    pub fn smart_zwnj(mut self, enabled: bool) -> Self {
        self.config.smart_zwnj = enabled;
        self
    }

    /// Enables or disables conversion to Persian punctuation.
    ///
    /// Default: disabled. See [`punctuation::normalize`].
    #[must_use]
    pub fn punctuation(mut self, enabled: bool) -> Self {
        self.config.punctuation = enabled;
        self
    }

    /// Finishes building and returns the [`Normalizer`].
    #[must_use]
    pub fn build(self) -> Normalizer {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn default_equals_builder_default() {
        assert_eq!(Normalizer::default(), Normalizer::builder().build());
    }

    #[test]
    fn default_configuration_matches_documented_defaults() {
        let n = Normalizer::default();
        assert!(n.unify_letters);
        assert!(n.remove_diacritics);
        assert_eq!(n.digits, DigitStyle::Persian);
        assert!(n.fix_spacing);
        assert!(!n.smart_zwnj);
        assert!(!n.punctuation);
    }

    #[test]
    fn builder_overrides_each_option() {
        let n = Normalizer::builder()
            .unify_letters(false)
            .remove_diacritics(false)
            .digits(DigitStyle::Latin)
            .fix_spacing(false)
            .smart_zwnj(true)
            .punctuation(true)
            .build();

        assert!(!n.unify_letters);
        assert!(!n.remove_diacritics);
        assert_eq!(n.digits, DigitStyle::Latin);
        assert!(!n.fix_spacing);
        assert!(n.smart_zwnj);
        assert!(n.punctuation);
    }

    #[test]
    fn all_steps_disabled_is_identity_and_borrowed() {
        let n = Normalizer::builder()
            .unify_letters(false)
            .remove_diacritics(false)
            .digits(DigitStyle::Keep)
            .fix_spacing(false)
            .build();
        let input = "كتاب ١٢٣";
        assert!(matches!(n.normalize(input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn public_types_are_send_and_sync() {
        assert_send_sync::<Normalizer>();
        assert_send_sync::<NormalizerBuilder>();
        assert_send_sync::<DigitStyle>();
    }
}
