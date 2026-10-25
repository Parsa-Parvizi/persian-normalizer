# Contributing to persian-normalizer

Thank you for your interest in improving `persian-normalizer`. Contributions of
all kinds are welcome: bug reports, test cases, documentation, and code.
Issues and pull requests may be written in **English or Persian**.

Please read the [Code of Conduct](CODE_OF_CONDUCT.md) before participating. To
report a security problem, follow [SECURITY.md](SECURITY.md) instead of opening
a public issue.

## Before you start

This crate is deliberately small. Read `docs/ARCHITECTURE.md` first. In
particular:

- **Scope (section 1):** normalization only. Tokenization, stemming, spell
  checking, transliteration and validators are out of scope.
- **No new runtime dependencies** in the default build, and no `regex`.
- **No `unsafe` code.** The crate forbids it.
- **No panics.** Do not use `unwrap`, `expect`, `panic!` or indexing that can
  fail in library code. Tests may use them.
- **Guarantees (D-20):** every step must be idempotent, total, zero-copy when
  nothing changes, and linear in time and memory.

For anything beyond a small fix, please open an issue first so we can agree on
the approach before you spend time on code.

## Development setup

You need a recent stable Rust toolchain plus `rustfmt` and `clippy`:

```sh
rustup default stable
rustup component add rustfmt clippy
```

Clone the repository and run the full check:

```sh
git clone https://github.com/Parsa-Parvizi/persian-normalizer.git
cd persian-normalizer
cargo test --all-features
```

Optional tools used by maintainers: `cargo-deny`, `cargo-semver-checks`,
`cargo-llvm-cov`, `cargo-nextest` (note that nextest does not run doctests, so
also run `cargo test --doc`).

## Before opening a pull request

Run all of these. CI runs the same checks:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --lib --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo doc --no-deps --all-features
```

The minimum supported Rust version is **1.85**. Do not use language or library
features newer than that.

## Writing a normalization step

Every step follows the same pattern:

1. A pure function `fn(&str) -> Cow<'_, str>` in its own module.
2. A **fast path**: scan for the first character that needs changing and return
   `Cow::Borrowed(text)` if there is none. Allocate at most once otherwise.
   `util::map_chars` implements this for one-to-one character mappings.
3. Documented rules in the module docs, including what is deliberately **not**
   changed.
4. Wiring into `Normalizer` at the position defined in the pipeline order
   (`docs/ARCHITECTURE.md`, section 3). The order is not user-configurable.

A step is not finished until it has:

- unit tests and an integration test file in `tests/`,
- property tests for idempotence and the zero-copy path in
  `tests/properties.rs`,
- rustdoc with a runnable example,
- an entry in `CHANGELOG.md`,
- an entry in `docs/COMPATIBILITY.md` for any difference from Hazm.

## Tests: use escapes for non-ASCII text

Persian and Arabic characters look alike and are easily changed by editors and
copy-paste (for example Arabic kaf `U+0643` versus Persian kaf `U+06A9`). In
tests, write the characters as escapes and add a comment:

```rust
let input = "\u{0643}"; // Arabic kaf
assert_eq!(letters::unify(input), "\u{06A9}"); // Persian kaf
```

Examples meant for readers (README, crate docs, `examples/`) may use the
characters directly.

## Commit messages

We use [Conventional Commits](https://www.conventionalcommits.org/):
`feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `perf:`, `chore:`. For example:

```text
feat(digits): convert Arabic-Indic digits to Persian
fix(spacing): keep ZWNJ between a prefix and a verb
```

## Pull requests

- Work on a branch; do not push to `main`.
- Keep pull requests focused. Unrelated cleanups belong in their own PR.
- Fill in the pull request template.
- Behavior changes need before/after examples in the PR description.
- Changing a default value is a **breaking change**. It needs a new decision
  record that supersedes the old one in `docs/ARCHITECTURE.md`.

## License of contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project, as defined in the Apache-2.0 license, is dual
licensed under **MIT OR Apache-2.0**, without any additional terms or
conditions.