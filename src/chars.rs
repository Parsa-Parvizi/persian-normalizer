//! Code point constants and character classification shared by the steps.

// Arabic letter variants that are unified into Persian letters.
pub(crate) const ARABIC_YEH: char = '\u{064A}';
pub(crate) const ALEF_MAKSURA: char = '\u{0649}';
pub(crate) const ARABIC_KAF: char = '\u{0643}';
pub(crate) const TEH_MARBUTA: char = '\u{0629}';

// Persian target letters.
pub(crate) const PERSIAN_YEH: char = '\u{06CC}';
pub(crate) const PERSIAN_KAF: char = '\u{06A9}';
pub(crate) const HEH: char = '\u{0647}';

// Other special characters.
pub(crate) const TATWEEL: char = '\u{0640}';
pub(crate) const ZWNJ: char = '\u{200C}';

// Code point of the digit zero in each supported numeral system.
pub(crate) const LATIN_ZERO: u32 = 0x0030;
pub(crate) const ARABIC_INDIC_ZERO: u32 = 0x0660;
pub(crate) const PERSIAN_ZERO: u32 = 0x06F0;

/// Returns the numeric value (0 to 9) of a Latin, Arabic-Indic or Persian digit.
///
/// Returns `None` for every other character.
pub(crate) fn digit_value(c: char) -> Option<u32> {
    let zero = match c {
        '0'..='9' => LATIN_ZERO,
        '\u{0660}'..='\u{0669}' => ARABIC_INDIC_ZERO,
        '\u{06F0}'..='\u{06F9}' => PERSIAN_ZERO,
        _ => return None,
    };
    Some(u32::from(c) - zero)
}

/// Returns `true` for the diacritics and the tatweel that `diacritics::strip` removes.
pub(crate) const fn is_diacritic(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{0652}' | '\u{0670}' | TATWEEL)
}

/// Returns `true` for letters of the Arabic script as used in Persian.
///
/// Digits, marks, tatweel, punctuation and ZWNJ are not letters.
pub(crate) const fn is_letter(c: char) -> bool {
    matches!(
        c,
        '\u{0621}'..='\u{063A}' | '\u{0641}'..='\u{064A}' | '\u{0671}'..='\u{06D3}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_value_covers_all_three_sets() {
        for value in 0..10u32 {
            for zero in [LATIN_ZERO, ARABIC_INDIC_ZERO, PERSIAN_ZERO] {
                let c = char::from_u32(zero + value).unwrap();
                assert_eq!(digit_value(c), Some(value));
            }
        }
    }

    #[test]
    fn digit_value_rejects_neighbours_and_other_characters() {
        // Just outside each range, plus unrelated characters.
        for c in [
            '/', ':', '\u{065F}', '\u{066A}', '\u{06EF}', '\u{06FA}', 'a', ' ',
        ] {
            assert_eq!(digit_value(c), None, "{c:?}");
        }
    }

    #[test]
    fn is_diacritic_matches_the_documented_set() {
        for code in 0x064B..=0x0652u32 {
            assert!(is_diacritic(char::from_u32(code).unwrap()), "{code:#06X}");
        }
        assert!(is_diacritic('\u{0670}'));
        assert!(is_diacritic(TATWEEL));
    }

    #[test]
    fn is_diacritic_rejects_neighbours_and_letters() {
        for c in [
            '\u{064A}', '\u{0653}', '\u{065F}', '\u{0671}', '\u{06A9}', 'a', ' ',
        ] {
            assert!(!is_diacritic(c), "{c:?}");
        }
    }

    #[test]
    fn is_letter_accepts_persian_letters() {
        for c in [
            '\u{0627}', '\u{0628}', '\u{067E}', '\u{0686}', '\u{0698}', '\u{06A9}', '\u{06AF}',
            '\u{06CC}',
        ] {
            assert!(is_letter(c), "{c:?}");
        }
    }

    #[test]
    fn is_letter_rejects_digits_marks_and_separators() {
        for c in [
            '\u{0640}', // tatweel
            '\u{064B}', // fathatan
            '\u{0660}', // Arabic-Indic zero
            '\u{06F0}', // Persian zero
            '\u{060C}', // Arabic comma
            ZWNJ, ' ', 'a', '1',
        ] {
            assert!(!is_letter(c), "{c:?}");
        }
    }
}
