<div align="center">

# persian-normalizer

**Small, fast, zero-copy-when-possible Persian (Farsi) text normalization for Rust.**

[![Crates.io](https://img.shields.io/crates/v/persian-normalizer.svg)](https://crates.io/crates/persian-normalizer)
[![Docs.rs](https://docs.rs/persian-normalizer/badge.svg)](https://docs.rs/persian-normalizer)
[![CI](https://github.com/Parsa-Parvizi/persian-normalizer/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Parsa-Parvizi/persian-normalizer/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

[English](#english) · [فارسی](#فارسی)

</div>

---

## English

`persian-normalizer` turns messy Persian text into a single, consistent form so
that search, deduplication, comparison, storage and NLP pipelines behave
predictably.

It is deliberately **small**. It is not an NLP framework: it does one job,
does it with no mandatory dependencies, and is meant to be the foundation that
larger tools build on.

### Why this crate?

| Principle | What it means |
|---|---|
| **Zero-copy when possible** | Every step returns `Cow<str>`. If the input is already normalized, nothing is allocated. |
| **Dependency-free core** | No `regex`, no heavy Unicode tables. Only `serde` is available, and only behind a feature flag. |
| **Idempotent** | `normalize(normalize(x)) == normalize(x)`. Enforced by property-based tests. |
| **Never panics** | Total functions over all valid `&str` input. Covered by property tests, a stress test and a fuzz target. |
| **Composable** | Use the one-line `normalize()` or pick individual steps. |
| **`no_std` ready** | Works with `alloc` only. Disable the default `std` feature. |
| **Conventional API** | Designed against the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/). |

### Installation

```sh
cargo add persian-normalizer
```

or in `Cargo.toml`:

```toml
[dependencies]
persian-normalizer = "0.1"
```

To try the development version from GitHub:

```sh
cargo add persian-normalizer --git https://github.com/Parsa-Parvizi/persian-normalizer
```

### Quick start

```rust
use persian_normalizer::normalize;

// Arabic Kaf/Yeh and Arabic-Indic digits become their Persian forms.
assert_eq!(normalize("كتاب ١٢٣"), "کتاب ۱۲۳");
```

`normalize` returns `Cow<str>`, so you only pay for allocation when something
actually changed:

```rust
use std::borrow::Cow;
use persian_normalizer::normalize;

let already_clean = "کتاب ۱۲۳";
assert!(matches!(normalize(already_clean), Cow::Borrowed(_)));
```

### Configuring the pipeline

```rust
use persian_normalizer::{DigitStyle, Normalizer};

let normalizer = Normalizer::builder()
    .unify_letters(true)           // ي -> ی, ك -> ک
    .remove_diacritics(true)       // strip harakat, superscript alef, tatweel
    .digits(DigitStyle::Latin)     // ۱۲۳ and ١٢٣ -> 123
    .fix_spacing(true)             // collapse spaces, clean misplaced ZWNJ
    .smart_zwnj(true)              // می روم -> می‌روم   (opt-in heuristic)
    .punctuation(true)             // , ; ? -> ، ؛ ؟    (opt-in)
    .build();

assert_eq!(normalizer.normalize("۱۲۳ ٤٥٦"), "123 456");
assert_eq!(
    normalizer.normalize("می روم, کتاب ها?"),
    "می\u{200C}روم، کتاب\u{200C}ها؟"
);
```

A `Normalizer` is cheap to build, `Clone`, `Send + Sync`, and meant to be
created once and reused.

### Using individual steps

Every step is a pure function, available on its own:

```rust
use persian_normalizer::{diacritics, digits, letters, punctuation, spacing, DigitStyle};

assert_eq!(letters::unify("علي كتاب"), "علی کتاب");
assert_eq!(digits::convert("۱۲۳", DigitStyle::Arabic), "١٢٣");
assert_eq!(diacritics::strip("ک\u{0650}تاب"), "کتاب");
assert_eq!(spacing::clean("این   یک  متن"), "این یک متن");
assert_eq!(spacing::smart_zwnj("می روم"), "می\u{200C}روم");
assert_eq!(punctuation::normalize("سلام, چطوری? 1,234"), "سلام، چطوری؟ 1,234");
```

Runnable versions live in [`examples/`](examples):

```sh
cargo run --example basic
cargo run --example builder
cargo run --example individual_steps
cargo run --example search_indexing
```

### What it normalizes

| Step | Module | Default | Examples |
|---|---|:---:|---|
| Arabic → Persian letters | `letters` | on | `ي` → `ی`, `ك` → `ک`, `ى` → `ی`, `ة` → `ه` |
| Digit unification | `digits` | on (`Persian`) | `123`, `١٢٣`, `۱۲۳` ⇄ any one script |
| Diacritics and tatweel | `diacritics` | on | removes U+064B–U+0652, U+0670 and U+0640 (`ـ`) |
| Spacing and ZWNJ cleanup | `spacing` | on | collapses runs of spaces; removes ZWNJ at the edges, next to spaces or newlines, and when repeated |
| Smart ZWNJ insertion | `spacing` | **off** | `می روم` → `می‌روم`, `کتاب ها` → `کتاب‌ها` |
| Persian punctuation | `punctuation` | **off** | `,` → `،`, `;` → `؛`, `?` → `؟` (only after Persian letters, so `1,234` is untouched) |

Things that are deliberately **left alone**: alef and hamza variants (`أ إ ٱ ؤ ئ`),
tabs, newlines and no-break spaces, decimal and thousands separators, and
leading or trailing whitespace (the text is never trimmed).

Notes:

* Removing diacritics is lossy. Disable it with `remove_diacritics(false)` for
  text where they carry meaning, such as Quranic text.
* Digit conversion has no notion of context, so digits inside identifiers,
  URLs and code are converted too.
* Smart ZWNJ insertion is heuristic. It turns a single space into a ZWNJ only
  between two whole Persian words: after the verb prefixes `می` / `نمی` when the
  next word has at least two letters, or before the plurals `ها` / `های` when the
  previous word has at least two letters. It uses no dictionary.

The complete character tables and every deliberate difference from other
tools are listed in [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md).

### Pipeline order

Steps always run in this fixed order (not configurable, to keep the result
idempotent). Disabled steps are skipped.

1. `letters::unify`
2. `diacritics::strip`
3. `digits::convert`
4. `spacing::clean`
5. `punctuation::normalize` (off by default)
6. `spacing::smart_zwnj` (off by default)

The order matters. For example, spacing cleanup runs before punctuation because
cleanup can remove a ZWNJ that sits between a letter and a mark. The reasoning
behind every decision is recorded in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

### Feature flags

| Feature | Default | Description |
|---|:---:|---|
| `std` | yes | Uses the standard library. Disable it for `no_std` + `alloc` targets. |
| `serde` | no | `Serialize` / `Deserialize` for `DigitStyle`. |

```toml
# no_std + alloc
persian-normalizer = { version = "0.1", default-features = false }

# with serde support
persian-normalizer = { version = "0.1", features = ["serde"] }
```

### Guarantees

These properties are part of the public contract and are checked by the test
suite:

1. **Idempotence**: applying the pipeline twice equals applying it once, for
   every configuration.
2. **Totality**: no input causes a panic.
3. **Zero-copy fast path**: already-normalized input is returned borrowed.
4. **Bounded size and linear time**: the output is at most three times the input
   size in bytes, and processing time is linear in the input length.
5. **No `unsafe`**: the crate forbids `unsafe_code`.

They are verified by property tests (`proptest`), a deterministic stress test and
a `cargo-fuzz` target. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), D-20.

### Performance

Benchmarks live in [`benches/`](benches) and use
[Criterion](https://github.com/bheisler/criterion.rs):

```sh
cargo bench
```

Results depend on your hardware and input, so run them on your own data
before drawing conclusions.

### Minimum Supported Rust Version

The MSRV is **1.85**. Raising it is treated as a minor-version change and is
recorded in the [changelog](CHANGELOG.md).

```sh
rustup toolchain install 1.85
cargo +1.85 check --lib --no-default-features
cargo +1.85 check --lib --all-features
```

### Alternatives

Other Persian text crates exist, and they target different needs:

- [`yekdast`](https://crates.io/crates/yekdast): configurable normalizer with user dictionaries and protection for URLs and emails.
- [`parsitext`](https://crates.io/crates/parsitext): broad Persian text-processing engine (entities, validators, transliteration and more).
- [`rustam`](https://crates.io/crates/rustam): a Hazm-style NLP pipeline (tokenization, stemming, lemmatization and more).

Choose `persian-normalizer` when you want a small, dependency-free building
block with strict guarantees. Choose the others when you need their wider
feature sets.

### Roadmap

- [ ] Golden-file comparison with [Hazm](https://github.com/roshan-research/hazm)
- [ ] Opt-in handling of Arabic Presentation Forms
- [ ] Opt-in protection for URLs, emails and code spans
- [ ] Streaming / iterator API for very large inputs
- [ ] Optional Python bindings (PyO3)

### Development

You need a recent stable Rust toolchain with `rustfmt` and `clippy`:

```sh
rustup default stable
rustup component add rustfmt clippy
git clone https://github.com/Parsa-Parvizi/persian-normalizer.git
cd persian-normalizer
```

**Everything CI runs:**

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --lib --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo doc --no-deps --all-features
cargo bench --no-run
```

Treat rustdoc warnings as errors, like CI does:

```sh
# Linux / macOS
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
# Windows (cmd)
set RUSTDOCFLAGS=-D warnings && cargo doc --no-deps --all-features
# Windows (PowerShell)
$env:RUSTDOCFLAGS = "-D warnings"; cargo doc --no-deps --all-features
```

**Property tests with many more cases** (the default is 256):

```sh
# Linux / macOS
PROPTEST_CASES=20000 cargo test --test properties
# Windows (cmd)
set PROPTEST_CASES=20000 && cargo test --test properties
# Windows (PowerShell)
$env:PROPTEST_CASES = 20000; cargo test --test properties
```

**`no_std` on a bare-metal target:**

```sh
rustup target add thumbv7em-none-eabi
cargo build --lib --no-default-features --target thumbv7em-none-eabi
```

**Fuzzing** (needs a nightly toolchain; Linux, macOS or WSL):

```sh
cargo install cargo-fuzz --locked
cargo +nightly fuzz run normalize -- -max_total_time=60
```

**Dependency, license and advisory checks:**

```sh
cargo install cargo-deny --locked
cargo deny check
```

<details>
<summary>Maintainers: release checklist</summary>

Run these on an up-to-date, clean default branch:

```sh
git status
cargo package --list
cargo publish --dry-run
cargo publish
git tag v0.1.0
git push origin v0.1.0
```

Publishing is irreversible: a version can be yanked but never deleted. Update
`CHANGELOG.md` first, and enable the `semver` job in `.github/workflows/ci.yml`
once a first version exists on crates.io.

</details>

### Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](CONTRIBUTING.md)
first, work on a branch, and open a pull request. Run the "Everything CI runs"
block above before submitting.

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md). To report a security
issue, see [`SECURITY.md`](SECURITY.md).

### Project documents

| Document | Purpose |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Design decisions and their reasons |
| [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md) | Exact character tables and differences from other tools |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history |
| [`SECURITY.md`](SECURITY.md) | How to report a vulnerability |

### Acknowledgements

Behavior was cross-checked against [Hazm](https://github.com/roshan-research/hazm)
(MIT) and [persian-tools](https://github.com/persian-tools/persian-tools) (MIT).
API design follows the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

### License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

---

<div dir="rtl">

## فارسی

`persian-normalizer` یک کتابخانه‌ی کوچک و سریع برای **یکدست‌سازی متن فارسی** در Rust است.
متن‌های واقعی پر از نویسه‌های عربی، ارقام ناهمگون، اعراب و نیم‌فاصله‌های نابه‌جا هستند؛
این کتابخانه آن‌ها را به یک شکل استاندارد درمی‌آورد تا جست‌وجو، مقایسه، ذخیره‌سازی و پردازش زبان طبیعی قابل‌اتکا شود.

### ویژگی‌ها

- **بدون تخصیص حافظه‌ی اضافی:** اگر متن از قبل نرمال باشد، همان را بدون کپی برمی‌گرداند (`Cow<str>`).
- **بدون وابستگی اجباری:** هسته‌ی کتابخانه به هیچ کرِیت دیگری وابسته نیست.
- **idempotent:** اجرای دوباره‌ی نرمال‌سازی روی خروجی، چیزی را تغییر نمی‌دهد.
- **بدون panic:** روی هر رشته‌ی معتبر به‌درستی کار می‌کند.
- **ماژولار:** هم یک تابع ساده (`normalize`) دارد و هم مراحل جداگانه.
- **سازگار با `no_std`** (با `alloc`).
- **بدون `unsafe`.**

### نصب

```sh
cargo add persian-normalizer
```

برای امتحان نسخه‌ی در حال توسعه از GitHub:

```sh
cargo add persian-normalizer --git https://github.com/Parsa-Parvizi/persian-normalizer
```

### استفاده‌ی سریع

```rust
use persian_normalizer::normalize;

assert_eq!(normalize("كتاب ١٢٣"), "کتاب ۱۲۳");
```

### تنظیم دقیق‌تر

```rust
use persian_normalizer::{DigitStyle, Normalizer};

let normalizer = Normalizer::builder()
    .digits(DigitStyle::Latin)
    .remove_diacritics(true)
    .fix_spacing(true)
    .smart_zwnj(true)
    .punctuation(true)
    .build();

assert_eq!(normalizer.normalize("۱۲۳ ٤٥٦"), "123 456");
```

### چه کارهایی انجام می‌دهد؟

| مرحله | پیش‌فرض | مثال |
|---|:---:|---|
| یکسان‌سازی حروف عربی و فارسی | روشن | `ي` ← `ی` ، `ك` ← `ک` |
| یکسان‌سازی ارقام | روشن (فارسی) | `123` ، `١٢٣` ، `۱۲۳` |
| حذف اعراب و کشیده | روشن | `کِتاب` ← `کتاب` |
| پاکسازی فاصله و نیم‌فاصله | روشن | حذف فاصله‌های تکراری و نیم‌فاصله‌ی نابه‌جا |
| درج هوشمند نیم‌فاصله | **خاموش** | `می روم` ← `می‌روم` |
| علائم نگارشی فارسی | **خاموش** | `,` ← `،` |

**ترتیب اجرای مراحل ثابت است:** حروف، اعراب، ارقام، پاکسازی فاصله، علائم نگارشی، درج نیم‌فاصله.
این ترتیب عمداً قابل تغییر نیست تا نتیجه همیشه idempotent بماند.

چند نکته:

- حذف اعراب اطلاعات را از بین می‌برد؛ برای متن‌هایی که اعراب معنا دارند (مثل متن قرآنی) با `remove_diacritics(false)` خاموشش کنید.
- تبدیل ارقام زمینه را نمی‌شناسد؛ ارقام داخل شناسه‌ها و URLها هم تبدیل می‌شوند.
- درج نیم‌فاصله یک روش ابتکاری (heuristic) است و بدون واژه‌نامه کار می‌کند.
- همزه‌دارها (`أ إ ؤ ئ`)، tab، خط جدید، فاصله‌ی بدون‌شکست و جداکننده‌های اعشار عمداً دست‌نخورده می‌مانند، و متن هیچ‌وقت trim نمی‌شود.

جدول کامل کاراکترها در [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md) و دلیل هر تصمیم طراحی در [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) است.

### دستورهای اصلی توسعه

```sh
git clone https://github.com/Parsa-Parvizi/persian-normalizer.git
cd persian-normalizer

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo doc --no-deps --all-features

cargo run --example basic
cargo bench
```

فهرست کامل دستورها (تست property با تعداد بالا، `no_std`، fuzz، `cargo deny` و بررسی MSRV) در بخش
[Development](#development) بالاست.

### مشارکت

مشارکت شما با کمال میل پذیرفته می‌شود. لطفاً پیش از ارسال pull request فایل
[`CONTRIBUTING.md`](CONTRIBUTING.md) را بخوانید، روی یک شاخه‌ی جدا کار کنید و بعد pull request باز کنید.
گزارش و پیشنهاد به زبان فارسی یا انگلیسی پذیرفته می‌شود. گزارش مشکلات امنیتی را طبق [`SECURITY.md`](SECURITY.md) و نه در issue عمومی ارسال کنید.

### مجوز

این پروژه تحت مجوز **MIT** یا **Apache-2.0** (به انتخاب شما) منتشر شده است.

</div>
