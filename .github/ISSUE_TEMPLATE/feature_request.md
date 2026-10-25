---
name: Feature request
about: Suggest a new normalization rule or API improvement
title: "feat: "
labels: enhancement
---

## Problem

<!-- What real-world text or use case is not handled today? Examples with
     \u{...} escapes are very helpful. -->

## Proposed solution

<!-- What should the crate do? Sketch the API if relevant. -->

## Scope check

This crate intentionally stays small and dependency-free (see
`docs/ARCHITECTURE.md`, section 1). Please confirm:

- [ ] This is text **normalization**, not tokenization, stemming, spell
  checking, transliteration or validation.
- [ ] It can be implemented without new runtime dependencies.
- [ ] It can be made idempotent and panic-free.

## Alternatives considered

<!-- Other ways to solve it, including doing it in your own code or in a
     different crate. -->