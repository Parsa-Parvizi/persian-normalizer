//! Integration tests for `spacing::clean` and `spacing::smart_zwnj`.

use std::borrow::Cow;

use persian_normalizer::spacing;

const Z: char = '\u{200C}'; // ZWNJ

// Persian words, written with escapes.
const MI: &str = "\u{0645}\u{06CC}"; // می
const NAMI: &str = "\u{0646}\u{0645}\u{06CC}"; // نمی
const ROVAM: &str = "\u{0631}\u{0648}\u{0645}"; // روم
const KETAB: &str = "\u{06A9}\u{062A}\u{0627}\u{0628}"; // کتاب
const HA: &str = "\u{0647}\u{0627}"; // ها
const HAYE: &str = "\u{0647}\u{0627}\u{06CC}"; // های
const AND: &str = "\u{0648}"; // و

mod clean {
    use super::*;

    #[test]
    fn collapses_runs_of_spaces() {
        assert_eq!(spacing::clean("a  b   c"), "a b c");
        assert_eq!(spacing::clean("   "), " ");
    }

    #[test]
    fn leaves_tabs_newlines_and_no_break_spaces_alone() {
        for input in ["a\tb", "a\n\nb", "a\r\nb", "a\u{00A0}\u{00A0}b"] {
            assert!(
                matches!(spacing::clean(input), Cow::Borrowed(s) if s == input),
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn does_not_trim_the_text() {
        let input = " a ";
        assert!(matches!(spacing::clean(input), Cow::Borrowed(" a ")));
    }

    #[test]
    fn keeps_a_single_zwnj_between_letters() {
        let input = format!("{MI}{Z}{ROVAM}");
        assert!(matches!(spacing::clean(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn collapses_repeated_zwnj() {
        let input = format!("{MI}{Z}{Z}{Z}{ROVAM}");
        assert_eq!(spacing::clean(&input), format!("{MI}{Z}{ROVAM}"));
    }

    #[test]
    fn removes_zwnj_next_to_spaces_and_newlines() {
        assert_eq!(
            spacing::clean(&format!("{MI}{Z} {ROVAM}")),
            format!("{MI} {ROVAM}")
        );
        assert_eq!(
            spacing::clean(&format!("{MI} {Z}{ROVAM}")),
            format!("{MI} {ROVAM}")
        );
        assert_eq!(
            spacing::clean(&format!("{MI}{Z}\n{ROVAM}")),
            format!("{MI}\n{ROVAM}")
        );
        assert_eq!(
            spacing::clean(&format!("{MI}\r{Z}{ROVAM}")),
            format!("{MI}\r{ROVAM}")
        );
    }

    #[test]
    fn removes_zwnj_at_the_start_and_end() {
        assert_eq!(spacing::clean(&format!("{Z}{MI}")), MI);
        assert_eq!(spacing::clean(&format!("{MI}{Z}")), MI);
        assert_eq!(spacing::clean(&format!("{Z}{Z}")), "");
    }

    #[test]
    fn handles_zwnj_surrounded_by_spaces() {
        assert_eq!(
            spacing::clean(&format!("{MI} {Z} {ROVAM}")),
            format!("{MI} {ROVAM}")
        );
    }

    #[test]
    fn clean_input_is_borrowed_and_changed_input_is_owned() {
        assert!(matches!(spacing::clean(""), Cow::Borrowed("")));
        assert!(matches!(spacing::clean("a b"), Cow::Borrowed("a b")));
        assert!(matches!(spacing::clean("a  b"), Cow::Owned(_)));
    }

    #[test]
    fn keeps_other_characters_untouched() {
        let input = "\u{1F600}  x\u{0651}  1";
        assert_eq!(spacing::clean(input), "\u{1F600} x\u{0651} 1");
    }
}

mod smart_zwnj {
    use super::*;

    #[test]
    fn joins_the_verb_prefix_mi() {
        assert_eq!(
            spacing::smart_zwnj(&format!("{MI} {ROVAM}")),
            format!("{MI}{Z}{ROVAM}")
        );
    }

    #[test]
    fn joins_the_negative_verb_prefix_nami() {
        assert_eq!(
            spacing::smart_zwnj(&format!("{NAMI} {ROVAM}")),
            format!("{NAMI}{Z}{ROVAM}")
        );
    }

    #[test]
    fn joins_plural_suffixes() {
        assert_eq!(
            spacing::smart_zwnj(&format!("{KETAB} {HA}")),
            format!("{KETAB}{Z}{HA}")
        );
        assert_eq!(
            spacing::smart_zwnj(&format!("{KETAB} {HAYE} {ROVAM}")),
            format!("{KETAB}{Z}{HAYE} {ROVAM}")
        );
    }

    #[test]
    fn accepts_the_arabic_yeh_variants() {
        let mi_arabic = "\u{0645}\u{064A}";
        assert_eq!(
            spacing::smart_zwnj(&format!("{mi_arabic} {ROVAM}")),
            format!("{mi_arabic}{Z}{ROVAM}")
        );
    }

    #[test]
    fn does_not_join_when_the_following_word_is_one_letter() {
        // "می و" means "wine and"; it must not become "می‌و".
        let input = format!("{MI} {AND}");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn does_not_join_when_the_word_before_a_suffix_is_one_letter() {
        let input = format!("{AND} {HA}");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn requires_whole_words() {
        // A longer word that merely starts with "ها" is not the plural suffix.
        let input = format!("{KETAB} {HA}\u{0634}\u{0645}");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));

        // A longer word that merely ends with "می" is not the verb prefix.
        let input = format!("\u{0627}{MI} {ROVAM}");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn ignores_digits_latin_text_and_punctuation() {
        for input in ["1 2", "a b", "a , b"] {
            assert!(
                matches!(spacing::smart_zwnj(input), Cow::Borrowed(s) if s == input),
                "input: {input:?}"
            );
        }
        let input = format!("{MI} 123");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn only_a_single_space_is_replaced() {
        // Two spaces: neither space has a letter run on both sides.
        let input = format!("{MI}  {ROVAM}");
        assert!(matches!(spacing::smart_zwnj(&input), Cow::Borrowed(s) if s == input));
    }

    #[test]
    fn handles_several_joins_in_one_text() {
        let input = format!("{MI} {ROVAM} {KETAB} {HA} {NAMI} {ROVAM}");
        let expected = format!("{MI}{Z}{ROVAM} {KETAB}{Z}{HA} {NAMI}{Z}{ROVAM}");
        assert_eq!(spacing::smart_zwnj(&input), expected);
    }

    #[test]
    fn output_is_stable_and_borrowed_on_the_second_pass() {
        let input = format!("{MI} {MI} {ROVAM} {KETAB} {HA} {HAYE}");
        let once = spacing::smart_zwnj(&input).into_owned();
        assert!(matches!(spacing::smart_zwnj(&once), Cow::Borrowed(s) if s == once));
    }

    #[test]
    fn output_survives_cleaning_unchanged() {
        // The inserted ZWNJ sit between letters, so `clean` keeps them.
        let input = format!("{MI} {ROVAM} {KETAB} {HA}");
        let joined = spacing::smart_zwnj(&input).into_owned();
        assert!(matches!(spacing::clean(&joined), Cow::Borrowed(s) if s == joined));
    }
}
