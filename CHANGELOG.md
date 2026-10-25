# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While the version is `0.x`, a breaking change bumps the minor version, and
raising the minimum supported Rust version is also treated as a minor change.

## [Unreleased]

### Added

- Public API: `normalize`, `Normalizer`, `NormalizerBuilder` and `DigitStyle`.
- `letters::unify`: converts Arabic yeh, alef maksura, kaf and teh marbuta to
  their Persian forms.
- `digits::convert`: converts between Persian, Arabic-Indic and Latin digits.
- `serde` feature: `Serialize` and `Deserialize` for `DigitStyle`.
- `no_std` support (with `alloc`) by disabling the default `std` feature.
- Property tests for idempotence, totality and the zero-copy fast path.
- Runnable examples: `basic`, `builder`, `individual_steps` and `search_indexing`.
- Project documentation: README, architecture decision records, contributing
  guide, code of conduct and security policy.
- CI for formatting, linting, tests on Linux, Windows and macOS, MSRV, `no_std`
  and documentation builds.

### Not yet implemented

- `diacritics::strip`, `spacing::clean`, `spacing::smart_zwnj` and
  `punctuation::normalize` have their final signatures but currently return the
  input unchanged.

[Unreleased]: https://github.com/Parsa-Parvizi/persian-normalizer/commits/main