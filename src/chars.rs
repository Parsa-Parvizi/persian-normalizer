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
}
