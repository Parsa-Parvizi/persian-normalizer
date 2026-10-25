---
name: Bug report
about: Normalization produces a wrong result, panics, or behaves unexpectedly
title: "bug: "
labels: bug
---

> **Security issues:** please do not report them here. See `SECURITY.md`.

## What happened

<!-- A clear description of the problem. -->

## Reproduction

Please write non-ASCII characters as escapes, so the exact code points are
unambiguous (copy-pasted Persian and Arabic text often changes in transit).

```rust
use persian_normalizer::normalize;

let input = "\u{0643}\u{064A}"; // replace with your input
let output = normalize(input);
println!("{output:?}");
```

## Expected output

<!-- Use escapes here too, for example "\u{06A9}\u{06CC}". -->

## Actual output

## Environment

- `persian-normalizer` version:
- Enabled features (`std`, `serde`):
- Rust version (`rustc --version`):
- Operating system: