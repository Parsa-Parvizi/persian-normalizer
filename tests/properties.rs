//! Property-based tests for the guarantees in `docs/ARCHITECTURE.md` (D-20):
//! idempotence, totality and the zero-copy fast path.

use std::borrow::Cow;

use persian_normalizer::{digits, letters, normalize, DigitStyle, Normalizer};
use proptest::prelude::*;

mod pipeline;

/// Strings dominated by Arabic-script characters, ZWNJ, ASCII and punctuation.
fn persian_like() -> impl Strategy<Value = String> {
    "[\u{0600}-\u{06FF}\u{200C} a-z0-9,;?]{0,64}"
}

fn all_digit_styles() -> [DigitStyle; 4] {
    [
        DigitStyle::Persian,
        DigitStyle::Arabic,
        DigitStyle::Latin,
        DigitStyle::Keep,
    ]
}

proptest! {
    #[test]
    fn letters_unify_is_idempotent(s in persian_like()) {
        let once = letters::unify(&s).into_owned();
        let twice = letters::unify(&once).into_owned();
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn digits_convert_is_idempotent_for_every_style(s in persian_like()) {
        for style in all_digit_styles() {
            let once = digits::convert(&s, style).into_owned();
            let twice = digits::convert(&once, style).into_owned();
            prop_assert_eq!(once, twice);
        }
    }

    #[test]
    fn default_pipeline_is_idempotent_on_persian_like_text(s in persian_like()) {
        let once = normalize(&s).into_owned();
        let twice = normalize(&once).into_owned();
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn default_pipeline_is_total_and_idempotent_on_arbitrary_text(s in any::<String>()) {
        let once = normalize(&s).into_owned();
        let twice = normalize(&once).into_owned();
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn normalized_text_takes_the_zero_copy_path(s in persian_like()) {
        let once = normalize(&s).into_owned();
        prop_assert!(matches!(normalize(&once), Cow::Borrowed(_)));
    }

    #[test]
    fn fully_enabled_pipeline_is_idempotent(s in persian_like()) {
        let normalizer = Normalizer::builder()
            .smart_zwnj(true)
            .punctuation(true)
            .build();
        let once = normalizer.normalize(&s).into_owned();
        let twice = normalizer.normalize(&once).into_owned();
        prop_assert_eq!(once, twice);
    }
}