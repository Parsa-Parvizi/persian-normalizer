# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While the version is `0.x`, a breaking change bumps the minor version, and
raising the minimum supported Rust version is also treated as a minor change.

## [Unreleased]

## [0.1.0] - 2026-11-08

First release.

### Added

- Public API: `normalize`, `Normalizer`, `NormalizerBuilder` and `DigitStyle`.
- Six normalization steps, each also available as a standalone pure function:
    - `letters::unify`: Arabic yeh, alef maksura, kaf and teh marbuta become their
      Persian forms.
    - `diacritics::strip`: removes harakat, superscript alef and tatweel.
    - `digits::convert`: converts between Persian, Arabic-Indic and Latin digits.
    - `spacing::clean`: collapses spaces and removes repeated or misplaced ZWNJ.
    - `punctuation::normalize` (opt-in): converts `,` `;` `?` after Persian letters.
    - `spacing::smart_zwnj` (opt-in): inserts ZWNJ after the verb prefixes `می` and
      `نمی` and before the plural suffixes `ها` and `های`.
- A fixed pipeline order (letters, diacritics, digits, spacing cleanup,
  punctuation, smart ZWNJ) that keeps the result idempotent. See
  `docs/ARCHITECTURE.md`, D-29.
- `serde` feature: `Serialize` and `Deserialize` for `DigitStyle`.
- `no_std` support (with `alloc`) by disabling the default `std` feature.
- Zero-copy fast path: every step returns `Cow::Borrowed` when nothing changes.
- Property tests for idempotence, totality, bounded output size and the
  zero-copy path; a `cargo-fuzz` target; Criterion benchmarks.
- Runnable examples: `basic`, `builder`, `individual_steps` and `search_indexing`.
- Documentation: README, architecture decision records, exact character tables
  and compatibility notes, contributing guide, code of conduct and security
  policy.
- CI for formatting, linting, tests on Linux, Windows and macOS (and Linux beta),
  MSRV 1.85, a bare-metal `no_std` build and documentation; scheduled
  dependency audit (`cargo-deny`) and fuzz runs; Dependabot updates.

[Unreleased]: https://github.com/Parsa-Parvizi/persian-normalizer/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Parsa-Parvizi/persian-normalizer/releases/tag/v0.1.0
