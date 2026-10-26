//! # persian-normalizer
//!
//! Small, fast, zero-copy-when-possible Persian (Farsi) text normalization.
//!
//! Every step returns a [`Cow<str>`](alloc::borrow::Cow): if the input is
//! already normalized, nothing is allocated and the input is returned borrowed.
//!
//! ```
//! use std::borrow::Cow;
//! use persian_normalizer::normalize;
//!
//! assert!(matches!(normalize("کتاب"), Cow::Borrowed(_)));
//! ```
//!
//! # Quick start
//!
//! ```
//! use persian_normalizer::normalize;
//!
//! assert_eq!(normalize("كتاب ١٢٣"), "کتاب ۱۲۳");
//! ```
//!
//! # Configuration
//!
//! Use [`Normalizer::builder`] to choose which steps run:
//!
//! ```
//! use std::borrow::Cow;
//! use persian_normalizer::{DigitStyle, Normalizer};
//!
//! let normalizer = Normalizer::builder()
//!     .digits(DigitStyle::Latin)
//!     .remove_diacritics(false)
//!     .build();
//!
//! assert!(matches!(normalizer.normalize("کتاب"), Cow::Borrowed(_)));
//! ```
//!
//! # Pipeline
//!
//! Steps always run in this fixed order:
//!
//! 1. [`letters::unify`]
//! 2. [`diacritics::strip`]
//! 3. [`digits::convert`]
//! 4. [`spacing::clean`]
//! 5. [`punctuation::normalize`] (off by default)
//! 6. [`spacing::smart_zwnj`] (off by default)
//!
//! Each step is also available on its own as a pure function.
//!
//! # Feature flags
//!
//! | Feature | Default | Description |
//! |---|:---:|---|
//! | `std` | yes | Uses the standard library. Disable for `no_std` + `alloc`. |
//! | `serde` | no | `Serialize` and `Deserialize` for [`DigitStyle`]. |

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::borrow::Cow;

pub mod diacritics;
pub mod digits;
pub mod letters;
pub mod punctuation;
pub mod spacing;

mod chars;
mod normalizer;
mod options;
mod util;

pub use normalizer::{Normalizer, NormalizerBuilder};
pub use options::DigitStyle;

/// Normalizes Persian text with the default configuration.
///
/// This is equivalent to `Normalizer::default().normalize(text)`. Returns
/// [`Cow::Borrowed`] when the input needs no changes.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use persian_normalizer::normalize;
///
/// let clean = normalize("کتاب");
/// assert!(matches!(clean, Cow::Borrowed(_)));
/// ```
#[must_use]
pub fn normalize(text: &str) -> Cow<'_, str> {
    Normalizer::default().normalize(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_clean_input_is_borrowed() {
        assert!(matches!(normalize("کتاب"), Cow::Borrowed(_)));
    }

    #[test]
    fn normalize_matches_default_normalizer() {
        let text = "متن آزمایشی";
        assert_eq!(normalize(text), Normalizer::default().normalize(text));
    }
}
