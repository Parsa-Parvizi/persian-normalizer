# Security Policy

## Supported versions

`persian-normalizer` is in the `0.x` series. Security fixes are released for the
**latest published minor version** only.

| Version              | Supported |
|----------------------|:---------:|
| Latest `0.x` release |    yes    |
| Older releases       |    no     |

## Reporting a vulnerability

**Please do not open a public issue for security problems.**

Report privately using one of these channels:

1. **GitHub private vulnerability reporting (preferred):** open the repository's
   **Security** tab and choose **Report a vulnerability**.
2. **Email:** `parsa.dev.python@gmail.com` with the subject line
   `[persian-normalizer] security report`.

Please include:

- the affected version (`cargo tree -i persian-normalizer` or your `Cargo.lock`),
- a minimal input or code sample that reproduces the problem,
- what you expected to happen and what happened instead,
- the impact as you understand it.

You will get an acknowledgement within **7 days**. This is a volunteer-maintained
project, so the following are goals, not guarantees: an initial assessment within
**14 days**, and a fix or mitigation for confirmed issues within **90 days**,
usually much sooner.

## What counts as a vulnerability

The crate promises (see `docs/ARCHITECTURE.md`, D-20) that normalization never
panics, runs in linear time and memory, is idempotent, and contains no `unsafe`
code. These are in scope:

- an input that makes any public function **panic** or abort,
- an input that causes **super-linear time or memory use**,
- any use of `unsafe`, or memory-safety problems in the crate,
- a **normalization bypass**, where text that this crate claims to normalize
  stays distinguishable in a way that defeats a security check built on it
  (for example, two spellings that should compare equal but do not),
- a vulnerable or malicious **dependency** introduced through this crate.

## What is not a vulnerability

- Normalization results that are linguistically debatable but documented
  (see `docs/COMPATIBILITY.md`). Please open a normal issue instead.
- Behavior that is intentionally lossy and documented, such as diacritic removal.
- Using normalization as the *only* defense against spoofing or injection.
  Normalization reduces variation; it is not a substitute for input
  validation, escaping or access control.
- Issues in your own application code, or in tools used only for development.

## Disclosure process

1. You report the issue privately.
2. We confirm it and work on a fix, keeping you informed.
3. We release a patched version and publish a GitHub security advisory. If
   appropriate, we also file an advisory with the [RustSec advisory
   database](https://github.com/rustsec/advisory-db).
4. We credit you in the advisory and the changelog, unless you prefer to stay
   anonymous.

We ask that you give us a reasonable time to ship a fix before disclosing the
issue publicly.

## Defensive measures already in place

- `#![forbid(unsafe_code)]` through the crate's lint configuration.
- No runtime dependencies in the default build.
- No regular expressions, so no regex-based denial-of-service risk.
- Property tests for idempotence and totality.

<!-- Add "fuzz target for the full pipeline" and "cargo deny checks in CI" to
     the list above once they exist. -->