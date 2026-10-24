<div align="center">

# persian-normalizer

**Small, fast, zero-copy-when-possible Persian (Farsi) text normalization for Rust.**

[![Crates.io](https://img.shields.io/crates/v/persian-normalizer.svg)](https://crates.io/crates/persian-normalizer)
[![Docs.rs](https://docs.rs/persian-normalizer/badge.svg)](https://docs.rs/persian-normalizer)
[![CI](https://github.com/<your-username>/persian-normalizer/actions/workflows/ci.yml/badge.svg)](https://github.com/<your-username>/persian-normalizer/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.75-blue.svg)](#minimum-supported-rust-version)
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
| **Never panics** | Total functions over all valid `&str` input. Covered by fuzzing. |
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
    .digits(DigitStyle::Latin)     // ۱۲۳ and ١٢٣ -> 123
    .remove_diacritics(true)       // strip harakat, tanwin, shadda, tatweel
    .fix_spacing(true)             // collapse spaces, clean misplaced ZWNJ
    .smart_zwnj(true)              // می روم -> می‌روم   (opt-in heuristic)
    .build();

assert_eq!(normalizer.normalize("۱۲۳ ٤٥٦"), "123 456");
assert_eq!(
    normalizer.normalize("می روم"),
    "می\u{200C}روم"
);
```

A `Normalizer` is cheap to build, `Clone`, `Send + Sync`, and meant to be
created once and reused.

### Using individual steps

Every step is a pure function, available on its own:

```rust
use persian_normalizer::{diacritics, digits, letters, DigitStyle};

assert_eq!(letters::unify("كي"), "کی");
assert_eq!(digits::convert("۱۲۳", DigitStyle::Arabic), "١٢٣");
assert_eq!(diacritics::strip("ک\u{0650}تاب"), "کتاب");
```

### What it normalizes

| Step | Module | Default | Examples |
|---|---|:---:|---|
| Arabic → Persian letters | `letters` | on | `ي` → `ی`, `ك` → `ک`, `ى` → `ی`, `ة` → `ه` |
| Digit unification | `digits` | on (`Persian`) | `123`, `١٢٣`, `۱۲۳` ⇄ any one script |
| Diacritics and tatweel | `diacritics` | on | removes U+064B–U+0652, U+0670 and U+0640 (`ـ`) |
| Spacing and ZWNJ cleanup | `spacing` | on | collapses repeated spaces; removes ZWNJ at word edges, next to spaces and when repeated |
| Smart ZWNJ insertion | `spacing` | **off** | `می روم` → `می‌روم`, `کتاب ها` → `کتاب‌ها` |
| Punctuation | `punctuation` | **off** | `,` → `،`, `;` → `؛`, `?` → `؟` (only next to Persian-script letters, so `1,234` is untouched) |

The complete character tables and every deliberate difference from other
tools are listed in [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md).

### Feature flags

| Feature | Default | Description |
|---|:---:|---|
| `std` | yes | Uses the standard library. Disable it for `no_std` + `alloc` targets. |
| `serde` | no | `Serialize` / `Deserialize` for `DigitStyle` and the normalizer options. |

```toml
# no_std + alloc
persian-normalizer = { version = "0.1", default-features = false }
```

### Guarantees

These properties are part of the public contract and are checked in CI:

1. **Idempotence**: applying the pipeline twice equals applying it once.
2. **Totality**: no input causes a panic.
3. **Zero-copy fast path**: already-normalized input is returned borrowed.
4. **No `unsafe`**: the crate uses `#![forbid(unsafe_code)]`.

### Performance

Benchmarks live in [`benches/`](benches) and use
[Criterion](https://github.com/bheisler/criterion.rs):

```sh
cargo bench
```

Results depend on your hardware and input, so run them on your own data
before drawing conclusions.

### Minimum Supported Rust Version

The MSRV is **1.75**. Raising it is treated as a minor-version change and is
recorded in the [changelog](CHANGELOG.md).

### Alternatives

Other Persian text crates exist, and they target different needs:

- [`yekdast`](https://crates.io/crates/yekdast): configurable normalizer with user dictionaries and protection for URLs and emails.
- [`parsitext`](https://crates.io/crates/parsitext): broad Persian text-processing engine (entities, validators, transliteration and more).
- [`rustam`](https://crates.io/crates/rustam): a Hazm-style NLP pipeline (tokenization, stemming, lemmatization and more).

Choose `persian-normalizer` when you want a small, dependency-free building
block with strict guarantees. Choose the others when you need their wider
feature sets.

### Roadmap

- [ ] Opt-in handling of Arabic Presentation Forms
- [ ] Opt-in protection for URLs, emails and code spans
- [ ] Streaming / iterator API for very large inputs
- [ ] Optional Python bindings (PyO3)

### Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](CONTRIBUTING.md)
first, and run these before opening a pull request:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
```

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md). To report a security
issue, see [`SECURITY.md`](SECURITY.md).

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

### نصب

```sh
cargo add persian-normalizer
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

### مشارکت

مشارکت شما با کمال میل پذیرفته می‌شود. لطفاً پیش از ارسال pull request فایل
[`CONTRIBUTING.md`](CONTRIBUTING.md) را بخوانید. گزارش و پیشنهاد به زبان فارسی یا انگلیسی پذیرفته می‌شود.

### مجوز

این پروژه تحت مجوز **MIT** یا **Apache-2.0** (به انتخاب شما) منتشر شده است.

</div>