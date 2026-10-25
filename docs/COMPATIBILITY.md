# Compatibility and Character Tables

This document is the authoritative description of **exactly which code points
`persian-normalizer` changes**, and of how its behavior relates to other Persian
normalization tools.

It complements `docs/ARCHITECTURE.md`: decisions live there, tables and
comparisons live here. Any change to a table in this file is a behavior change
and must be listed in `CHANGELOG.md`.

**Implementation status (version 0.1, in development):**

| Section                   | Status                         |
|---------------------------|--------------------------------|
| 1. Letters                | Implemented and tested         |
| 2. Digits                 | Implemented and tested         |
| 3. Diacritics and tatweel | Specified, not yet implemented |
| 4. Spacing and ZWNJ       | Specified, not yet implemented |
| 5. Punctuation            | Specified, not yet implemented |

Sections marked "not yet implemented" describe the intended behavior from the
decision records. They will be updated, and the status above changed, as each
step lands.

Notation: `U+XXXX` is a Unicode code point. In tests and bug reports, always
write non-ASCII characters as escapes such as `\u{0643}`, because Arabic and
Persian letters look identical on screen.

---

## 1. Letters (`letters::unify`)

Decision: D-09. A character is replaced only if it appears in this table.
Every other code point is left untouched.

| From   | Name                       | To     | Name                    |
|--------|----------------------------|--------|-------------------------|
| U+064A | ARABIC LETTER YEH          | U+06CC | ARABIC LETTER FARSI YEH |
| U+0649 | ARABIC LETTER ALEF MAKSURA | U+06CC | ARABIC LETTER FARSI YEH |
| U+0643 | ARABIC LETTER KAF          | U+06A9 | ARABIC LETTER KEHEH     |
| U+0629 | ARABIC LETTER TEH MARBUTA  | U+0647 | ARABIC LETTER HEH       |

**Deliberately not changed**, because the distinctions carry meaning in
Persian:

| Code point | Name                                |
|------------|-------------------------------------|
| U+0623     | ARABIC LETTER ALEF WITH HAMZA ABOVE |
| U+0625     | ARABIC LETTER ALEF WITH HAMZA BELOW |
| U+0671     | ARABIC LETTER ALEF WASLA            |
| U+0624     | ARABIC LETTER WAW WITH HAMZA ABOVE  |
| U+0626     | ARABIC LETTER YEH WITH HAMZA ABOVE  |

Properties: idempotent (no target is also a source), length-preserving in
UTF-8 (every source and target is a two-byte sequence).

---

## 2. Digits (`digits::convert`)

Decision: D-11. Exactly three sets of ten digits are recognized:

| Set                             | Range            | Zero   |
|---------------------------------|------------------|--------|
| Latin                           | U+0030 to U+0039 | U+0030 |
| Arabic-Indic                    | U+0660 to U+0669 | U+0660 |
| Persian (Extended Arabic-Indic) | U+06F0 to U+06F9 | U+06F0 |

`DigitStyle::Persian`, `Arabic` and `Latin` map every recognized digit to the
same value in the target set, so mixed input ends up in one numeral system.
`DigitStyle::Keep` returns the input unchanged. The default is `Persian`.

**Deliberately not changed:**

| Code point        | Name                       | Why                           |
|-------------------|----------------------------|-------------------------------|
| U+066A            | ARABIC PERCENT SIGN        | Not a digit                   |
| U+066B            | ARABIC DECIMAL SEPARATOR   | Context-dependent, deferred   |
| U+066C            | ARABIC THOUSANDS SEPARATOR | Context-dependent, deferred   |
| U+FF10 to U+FF19  | FULLWIDTH DIGITS           | Different script and use      |
| ASCII `,` and `.` | comma, full stop           | Not part of the digit mapping |

Known limitation: the function has no notion of context, so digits inside
identifiers, URLs and code are converted too. Opt-in protection for such spans
is on the roadmap.

---

## 3. Diacritics and tatweel (`diacritics::strip`) — not yet implemented

Decision: D-10. These code points are removed:

| Code point | Name                           |
|------------|--------------------------------|
| U+064B     | ARABIC FATHATAN                |
| U+064C     | ARABIC DAMMATAN                |
| U+064D     | ARABIC KASRATAN                |
| U+064E     | ARABIC FATHA                   |
| U+064F     | ARABIC DAMMA                   |
| U+0650     | ARABIC KASRA                   |
| U+0651     | ARABIC SHADDA                  |
| U+0652     | ARABIC SUKUN                   |
| U+0670     | ARABIC LETTER SUPERSCRIPT ALEF |
| U+0640     | ARABIC TATWEEL                 |

All other combining marks, including U+0653 to U+065F and the Quranic
annotation signs, are left untouched. Removal is lossy and enabled by default;
see D-08 for the rationale and the builder option to disable it.

---

## 4. Spacing and ZWNJ — not yet implemented

### 4.1 `spacing::clean` (decisions D-12 and D-13)

1. Runs of the space character U+0020 collapse to a single space.
2. Tabs, newlines and no-break spaces (U+00A0) are left unchanged.
3. The text is not trimmed.
4. Consecutive ZWNJ (U+200C) collapse into one.
5. A ZWNJ adjacent to a space or newline is removed.
6. A ZWNJ at the start or end of the text is removed.

ZWNJ in every other position is preserved, including after letters that do not
join to the next letter.

### 4.2 `spacing::smart_zwnj` (decision D-14, opt-in)

Version 0.1 handles only:

- the verb prefixes `می` and `نمی` followed by a word,
- the plural suffixes `ها` and `های` preceded by a word.

There is no dictionary. The rules are heuristic; a rule that cannot be made
idempotent and free of obvious false positives is removed rather than shipped.

---

## 5. Punctuation (`punctuation::normalize`) — not yet implemented

Decision: D-15, opt-in.

| From       | To         |
|------------|------------|
| `,` U+002C | `،` U+060C |
| `;` U+003B | `؛` U+061B |
| `?` U+003F | `؟` U+061F |

A mark is replaced only when the preceding non-space character is a
Persian-script letter. Numbers such as `1,234`, code and URLs are unaffected.

---

## 6. Relationship to Hazm-style normalizers

[Hazm](https://github.com/roshan-research/hazm) is the most widely used Persian
NLP toolkit and the reference for many users' expectations. Compatibility with
it is a **goal, not a promise** (D-22): correctness and the guarantees in D-20
take priority.

### 6.1 What this crate intentionally does not do

Hazm-style pipelines bundle normalization with broader text repair. For
example, the Rust crate `rustam`, which follows that design, exposes separate
switches for lexicon-based spacing correction, removal of special characters,
reduction of repeated characters and separation of the verb prefix `می`.
`persian-normalizer` deliberately has none of these:

| Capability                                  | Status here  | Reason                                      |
|---------------------------------------------|--------------|---------------------------------------------|
| Spacing correction driven by a word lexicon | Not provided | Needs word lists; out of scope (D-04, D-14) |
| Removal of "special" characters             | Not provided | Destructive and application-specific        |
| Reduction of repeated characters            | Not provided | Needs a lexicon to be safe                  |
| Token-based or POS-aware processing         | Not provided | This crate works on plain `&str`            |

### 6.2 Verification plan

Behavioral comparison is automated, not asserted by hand:

1. A small script, kept in the repository, runs a pinned Hazm version over a
   corpus of test strings and writes `tests/fixtures/hazm_cases.tsv`. The file
   records the Hazm version, the date and the exact options used.
2. `tests/hazm_compat.rs` reads that file. Python is **not** required to run the
   Rust tests.
3. Every row is either `same` or marked as a known difference with a reason.
   A difference without a reason fails the test.

### 6.3 Known differences

The "Status" column is deliberately explicit about what has been verified.

| Topic                                                          | This crate                                 | Status                                         |
|----------------------------------------------------------------|--------------------------------------------|------------------------------------------------|
| Lexicon-based spacing, repeated characters, special characters | Not provided                               | By design (see 6.1)                            |
| Whitespace                                                     | Only U+0020 runs collapse; no trimming     | By design (D-12)                               |
| ZWNJ cleanup                                                   | Removes only provably meaningless ZWNJ     | By design (D-13)                               |
| Alef and hamza variants                                        | Not unified                                | By design (D-09); Hazm behavior to be compared |
| Digits                                                         | Three sets converted; separators untouched | By design (D-11); Hazm behavior to be compared |
| Punctuation                                                    | Opt-in, context-aware                      | By design (D-15); Hazm behavior to be compared |
| Defaults                                                       | See D-08                                   | Hazm defaults to be compared                   |

Rows marked "to be compared" will be filled in from the golden fixture. Until
then, **this document makes no claim about what Hazm does** in those cases.

---

## 7. Reporting a difference

If this crate and another tool disagree on an input you care about, please open
an issue with:

- the input and both outputs, written as `\u{...}` escapes,
- the tool and exact version you compared with,
- which result you believe is correct, and why.

If the difference reveals a bug, it is fixed. If it is a deliberate choice, it
is added to the table in section 6.3.