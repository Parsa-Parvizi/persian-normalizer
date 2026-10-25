## Summary

<!-- What does this change, and why? Link the issue: "Closes #123". -->

## Type of change

- [ ] Bug fix
- [ ] New feature or behavior change
- [ ] Documentation only
- [ ] Refactor, performance or tooling (no behavior change)

## Checklist

- [ ] `cargo fmt --all` has been run
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` and `cargo test --no-default-features` pass
- [ ] New or changed public items have rustdoc with a runnable example
- [ ] Tests cover the change (unit or integration, and a property test if it
  affects idempotence or totality)
- [ ] `CHANGELOG.md` is updated under **Unreleased**
- [ ] The change respects `docs/ARCHITECTURE.md` (no new dependency, no
  `unsafe`, no panics). If it changes a decision, a new decision record is
  included.

## Behavior changes

<!-- If outputs change for any input, list before/after examples.
     Write non-ASCII characters as \u{...} escapes so the exact code points are clear. -->