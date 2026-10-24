//! Crate-private helpers.

use alloc::borrow::Cow;
use alloc::string::String;

/// Applies one normalization `step` to `input`, keeping the zero-copy fast path.
///
/// If the step reports no change (returns `Cow::Borrowed`), the original `input`
/// is returned untouched, preserving its lifetime. Otherwise the step's owned
/// output replaces it.
pub(crate) fn apply<F>(input: Cow<'_, str>, step: F) -> Cow<'_, str>
where
    F: FnOnce(&str) -> Cow<'_, str>,
{
    // The temporary returned by `step` borrows `input`. Reduce it to an
    // `Option<String>` inside this statement so that borrow ends before
    // `input` is moved below.
    let changed: Option<String> = match step(&input) {
        Cow::Borrowed(_) => None,
        Cow::Owned(owned) => Some(owned),
    };

    match changed {
        Some(owned) => Cow::Owned(owned),
        None => input,
    }
}

#[cfg(test)]
#[allow(
    clippy::redundant_closure,
    reason = "the closures pin the higher-ranked signature"
)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_step_keeps_borrowed_input() {
        let out = apply(Cow::Borrowed("abc"), |s| Cow::Borrowed(s));
        assert!(matches!(out, Cow::Borrowed("abc")));
    }

    #[test]
    fn changing_step_returns_owned_output() {
        let out = apply(Cow::Borrowed("abc"), |s| Cow::Owned(s.to_uppercase()));
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out, "ABC");
    }

    #[test]
    fn steps_can_be_chained_over_owned_input() {
        let first = apply(Cow::Borrowed("abc"), |s| Cow::Owned(s.to_uppercase()));
        let second = apply(first, |s| Cow::Borrowed(s));
        assert!(matches!(second, Cow::Owned(_)));
        assert_eq!(second, "ABC");
    }
}
