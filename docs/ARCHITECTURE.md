# Architecture and Decision Records

This document records the design of `persian-normalizer` and the reasons
behind each major decision. Decisions are numbered (`D-01`, `D-02`, ...) so they
can be referenced from issues and pull requests. A decision is only changed by
adding a new record that supersedes it, never by silently editing history.

**Status of this document:** accepted for version `0.1.0`.

---

## 1. Purpose and scope

### Goal

Convert messy Persian text into one consistent form, so search, comparison,
deduplication, storage and NLP pipelines behave predictably.

### In scope for 0.1

- Arabic to Persian letter unification
- Digit unification (Persian, Arabic-Indic, Latin)
- Removal of diacritics and tatweel
- Space and ZWNJ cleanup
- Opt-in: heuristic ZWNJ insertion, Persian punctuation

### Non-goals

These are intentionally excluded. Other crates already cover them, and adding
them would break the "small, dependency-free" promise.

- Tokenization and sentence splitting
- Stemming and lemmatization
- Spell correction
- Transliteration and Finglish conversion
- Entity recognition and validators (national ID, IBAN, ...)
- Calendar and date handling
- Number-to-words conversion

---

## 2. Module map

```text
src/
├── lib.rs          Crate docs, re-exports, the `normalize` function
├── normalizer.rs   `Normalizer` and `NormalizerBuilder`
├── options.rs      `DigitStyle` and shared option types
├── chars.rs        Code point constants and character classification
├── util.rs         Helpers for the `Cow` fast-path pattern
├── letters.rs      Arabic -> Persian letters
├── digits.rs       Digit conversion
├── diacritics.rs   Diacritics and tatweel removal
├── spacing.rs      Space cleanup, ZWNJ cleanup, smart ZWNJ insertion
└── punctuation.rs  Persian punctuation (opt-in)
```

Dependency direction is one-way: `lib.rs` and `normalizer.rs` depend on the step
modules; step modules depend only on `chars.rs`, `options.rs` and `util.rs`.
Step modules never depend on each other.

---

## 3. Pipeline

The pipeline order is fixed:

1. `letters::unify`
2. `diacritics::strip`
3. `digits::convert`
4. `punctuation::normalize` (opt-in)
5. `spacing::clean`
6. `spacing::smart_zwnj` (opt-in)

Rationale: letters are canonicalized first so later steps only reason about
canonical forms. Removing diacritics and tatweel happens before spacing because
their removal can bring characters together. Spacing runs last so it sees the
final text. Smart ZWNJ insertion runs after cleanup so it never fights the
cleanup rules.

Each step uses the same **fast-path pattern**: scan for the first character that
needs changing; if none exists, return `Cow::Borrowed(input)`; otherwise
allocate once, copy the unchanged prefix, then process the rest.

---

## 4. Decision records

### D-01: Crate name and repository

**Decision:** `persian-normalizer` (module name `persian_normalizer`).
**Rationale:** Descriptive and searchable. Availability on crates.io must be
verified before the first publish; if taken, this record is superseded.

### D-02: License

**Decision:** `MIT OR Apache-2.0`, shipped as `LICENSE-MIT` and `LICENSE-APACHE`.
**Rationale:** The standard dual license of the Rust ecosystem
(API Guidelines C-PERMISSIVE). It also keeps the crate compatible with the MIT
and Apache-2.0 projects it was cross-checked against.

### D-03: Edition and MSRV

**Decision:** Edition 2024, MSRV 1.85.
**Rationale:** 1.85 is the first release supporting edition 2024, and modern
Cargo writes lockfile version 4, which older toolchains cannot read, so a lower
MSRV cannot be tested reliably. Raising the MSRV is a minor-version change and
is announced in the changelog. CI tests on the MSRV and on stable.

### D-04: Scope

**Decision:** Normalization only (see section 1).
**Rationale:** A focused crate is easier to keep correct, fast and stable.
Other crates cover the broader NLP space.

### D-05: Public API shape

**Decision:**

- A free function `normalize(&str) -> Cow<'_, str>` using the default configuration.
- A `Normalizer` configured through `Normalizer::builder()`.
- One module per step, each exposing pure functions.

**Rationale:** Beginners get a one-liner; advanced users get composition. The
builder follows API Guidelines C-BUILDER.

### D-06: Input and output types

**Decision:** Functions take `&str` and return `Cow<'_, str>`. They do not take
`impl AsRef<str>`.
**Rationale:** `&String` and `&Cow<str>` already coerce to `&str`, so the generic
form adds compile time and code size without helping callers. `Cow` makes the
no-change case allocation-free.

### D-07: Error handling

**Decision:** Normalization is infallible. No public function returns `Result`.
No `unwrap`, `expect`, `panic!` or byte-offset slicing that could split a UTF-8
sequence in library code.
**Rationale:** Every valid `&str` has a defined normalized form. A fallible API
would add noise with nothing to report.

### D-08: Defaults

| Step | Default |
|---|---|
| Letters (Arabic to Persian) | on |
| Diacritics and tatweel removal | on |
| Digits | `DigitStyle::Persian` |
| Space and ZWNJ cleanup | on |
| Smart ZWNJ insertion | off |
| Persian punctuation | off |

**Rationale:** Defaults match what most Persian NLP tooling (notably Hazm)
does, so results are familiar. Heuristic or context-dependent steps are opt-in
because they can produce false positives.
**Known trade-off:** Removing diacritics is lossy. It is unsuitable for texts
where diacritics carry meaning (for example Quranic text), so the builder makes
it easy to disable.
**Note:** Changing a default after release is a breaking change, so these
values are final for the 0.x series unless a new record supersedes this one.

### D-09: Letter unification table (0.1)

**Decision:**

| From | To |
|---|---|
| `ي` U+064A | `ی` U+06CC |
| `ى` U+0649 | `ی` U+06CC |
| `ك` U+0643 | `ک` U+06A9 |
| `ة` U+0629 | `ه` U+0647 |

Alef and hamza variants (`أ`, `إ`, `ٱ`, `ؤ`, `ئ`) are **not** unified.
**Rationale:** Those characters carry meaning in Persian words and an
automatic merge would damage valid text. They can become opt-in later.

### D-10: Diacritics and tatweel

**Decision:** Remove U+064B to U+0652, U+0670 and tatweel U+0640.
**Rationale:** Covers tanwin, fatha, damma, kasra, shadda, sukun, superscript
alef and kashida, which are the marks that fragment search and comparison.

### D-11: Digits

**Decision:** Convert only the three sets of ten digits: Persian (U+06F0 to
U+06F9), Arabic-Indic (U+0660 to U+0669) and Latin (`0` to `9`). Decimal and
thousands separators are not touched in 0.1. `DigitStyle` has `Persian`,
`Arabic`, `Latin` and `Keep`.
**Rationale:** Digit mapping is exact and unambiguous; separators are
context-dependent and deferred.

### D-12: Whitespace

**Decision:** Collapse runs of the space character (U+0020) into one space. Tabs,
newlines and no-break spaces are left unchanged. The text is not trimmed.
**Rationale:** Conservative behavior avoids surprising callers who rely on line
structure. Callers can call `.trim()` themselves.

### D-13: ZWNJ cleanup rules

**Decision:** Cleanup (on by default) applies only these rules:

1. Collapse consecutive ZWNJ (U+200C) into one.
2. Remove a ZWNJ adjacent to a space or newline.
3. Remove a ZWNJ at the start or end of the text.

ZWNJ elsewhere is preserved, including after letters that do not join.
**Rationale:** These rules only remove ZWNJ that is certainly meaningless.
Everything else could be a deliberate, correct use.

### D-14: Smart ZWNJ insertion

**Decision:** Opt-in. Version 0.1 handles only the verb prefixes `می` and `نمی`
followed by a word, and the plural suffixes `ها` and `های` preceded by a word.
No dictionaries.
**Rationale:** A rule set small enough to explain and test. Anything beyond
needs word lists, which are out of scope.
**Constraint:** If a rule cannot be made idempotent and free of obvious false
positives, it is removed rather than shipped.

### D-15: Punctuation

**Decision:** Opt-in. Convert `,`, `;` and `?` to `،`, `؛` and `؟` only when the
preceding non-space character is a Persian-script letter.
**Rationale:** Blind replacement would corrupt numbers (`1,234`), code and URLs.

### D-16: Unicode normalization forms

**Decision:** The core does not apply NFC or NFKC and has no Unicode-table
dependency. Handling of Arabic Presentation Forms is a roadmap item, to be
added as an opt-in feature.
**Rationale:** Keeps the core dependency-free and predictable.

### D-17: Features and dependencies

**Decision:**

- `std` (default): uses the standard library. Without it the crate is
  `no_std` + `alloc`.
- `serde` (optional): `Serialize` and `Deserialize` for option types.
- No runtime dependencies in the default build.
- No `regex`.

**Rationale:** A tiny dependency tree is a core selling point and avoids
regex-related performance and denial-of-service concerns. Dev-dependencies
(`proptest`, `criterion`, `serde_json`) are allowed.

### D-18: Traits and future-proofing

**Decision:**

- Public enums are `#[non_exhaustive]` and derive `Debug`, `Clone`, `Copy`,
  `PartialEq`, `Eq` and `Hash`.
- `Normalizer` derives `Debug`, `Clone`, `Default`, `PartialEq`, `Eq` and
  `Hash`, has private fields, and is `Send + Sync`. It does not derive `Copy`.
- Builder methods consume and return `self`.

**Rationale:** Adding derives later is compatible; removing them is not.
Private fields and `non_exhaustive` let options grow without breaking users.

### D-19: Safety

**Decision:** `unsafe_code = "forbid"` at crate level.
**Rationale:** There is no performance need for `unsafe` in a text mapper, and
forbidding it is a strong, cheap trust signal.

### D-20: Guarantees

The following are part of the public contract and must be enforced by tests:

1. **Idempotence:** `normalize(normalize(x)) == normalize(x)` for every
   configuration.
2. **Totality:** no input causes a panic.
3. **Zero-copy fast path:** already-normalized input is returned as `Borrowed`.
4. **Linear time and memory:** processing time is O(n); output is at most three
   times the input size in bytes.

If a guarantee cannot be verified by the test suite, it is removed from the
README until it can be.

### D-21: Performance policy

**Decision:** No performance claim is published without a reproducible Criterion
benchmark in `benches/`. Version 0.1 chains independent steps. Fusing them into
a single pass is only considered if benchmarks justify the added complexity.

### D-22: Compatibility with other tools

**Decision:** Compatibility with Hazm is a goal, not a promise. Every
deliberate difference is documented in `docs/COMPATIBILITY.md`.
**Rationale:** Familiar behavior helps adoption, but correctness and the
guarantees above take priority.

### D-23: Testing strategy

**Decision:**

- Unit tests next to the code; one integration test file per step in `tests/`.
- Doctests for every public item.
- Property tests (`proptest`) for idempotence and totality, per configuration.
- Golden tests from a fixture file (`tests/fixtures/hazm_cases.tsv`), generated
  once offline and checked in with a note about its origin. Running the tests
  never requires Python.
- A `cargo-fuzz` target for the full pipeline.
- Coverage is measured with `cargo-llvm-cov`; the goal is high coverage but it
  is not a hard gate until the code stabilizes.

### D-24: Documentation standard

**Decision:** Every public item has rustdoc with at least one runnable example.
Fallible or panicking behavior is documented (there should be none). The crate
docs mirror the README quick start. No unstable rustdoc attributes are used.

### D-25: Versioning and releases

**Decision:** SemVer. Stay on `0.x` until the API is proven. In `0.x`, breaking
changes bump the minor version. `CHANGELOG.md` follows *Keep a Changelog*.
Releases are tagged `vX.Y.Z` after `cargo publish --dry-run` succeeds and
`cargo semver-checks` passes.

### D-26: Source control and contribution workflow

**Decision:** `main` is always releasable. Work happens on branches and lands
through pull requests, even for a solo maintainer. Commits follow Conventional
Commits (`feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`, `perf:`).
`Cargo.lock` is committed.

### D-27: Continuous integration

**Decision:** CI runs on every push and pull request:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features` and `cargo test --no-default-features`
- MSRV check on 1.85
- A `no_std` build check against a bare-metal target (for example
  `thumbv7em-none-eabi`)
- `cargo doc --no-deps --all-features` with warnings denied
- `cargo deny check` in a separate scheduled workflow

### D-28: Security

**Decision:** No `unsafe`, no regex, linear-time algorithms, and a fuzz target
to guard totality. Vulnerabilities are reported privately as described in
`SECURITY.md`.

---

## 5. Open questions (post 0.1)

- Opt-in handling of Arabic Presentation Forms (feature flag and dependency).
- Opt-in protection of URLs, emails and code spans.
- Iterator or streaming API for very large inputs.
- Fused single-pass pipeline, if benchmarks justify it.
- Python bindings (PyO3), only after the Rust API stabilizes.