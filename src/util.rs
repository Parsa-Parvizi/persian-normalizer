//! Crate-private helpers that implement the zero-copy fast-path pattern.

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

/// Maps every character of `text` through `map`.
///
/// Scans for the first character that `map` would change. If there is none,
/// returns `text` borrowed. Otherwise allocates exactly once, copies the
/// unchanged prefix and maps the rest.
///
/// `map` must be a pure function: the same input character always yields the
/// same output character.
pub(crate) fn map_chars<F>(text: &str, map: F) -> Cow<'_, str>
where
    F: Fn(char) -> char,
{
    let Some((first_change, _)) = text.char_indices().find(|&(_, c)| map(c) != c) else {
        return Cow::Borrowed(text);
    };

    // `first_change` comes from `char_indices`, so it is always a valid boundary.
    let (unchanged, rest) = text.split_at(first_change);

    let mut out = String::with_capacity(text.len());
    out.push_str(unchanged);
    out.extend(rest.chars().map(map));
    Cow::Owned(out)
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

    #[test]
    fn map_chars_borrows_when_nothing_changes() {
        assert!(matches!(map_chars("abc", |c| c), Cow::Borrowed("abc")));
        assert!(matches!(map_chars("", |c| c), Cow::Borrowed("")));
    }

    #[test]
    fn map_chars_replaces_every_occurrence() {
        let out = map_chars("a-b-c", |c| if c == '-' { '+' } else { c });
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out, "a+b+c");
    }

    #[test]
    fn map_chars_is_safe_with_multibyte_prefix() {
        let out = map_chars("é😀x😀", |c| if c == 'x' { 'y' } else { c });
        assert_eq!(out, "é😀y😀");
    }

    #[test]
    fn map_chars_handles_change_in_first_position() {
        let out = map_chars("xabc", |c| if c == 'x' { 'y' } else { c });
        assert_eq!(out, "yabc");
    }
}
