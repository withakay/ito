## Context

Ito's Rust workspace is split into multiple crates under `ito-rs/crates/`. Documentation quality is uneven across crates and public APIs, and doc build warnings can hide real issues.

This change is intentionally documentation-only: it improves generated Rust docs without changing runtime behavior.

## Goals / Non-Goals

Goals:
- Every core crate has clear crate-level documentation (`//!` in `lib.rs`).
- Public APIs have useful docs that explain purpose, when to use, and any gotchas.
- `cargo doc` (and `make docs` where available) runs without documentation warnings.

Non-goals:
- No behavioral changes.
- No API redesigns or refactors except what is required to attach documentation.
- No new dependencies.

## Decisions

- Validation is anchored on warning-free docs: `cargo doc --no-deps` (or `make docs`) must complete without warnings.
- `#![warn(missing_docs)]` is enabled only where it helps prevent regressions without forcing noisy, low-value documentation; otherwise doc coverage is enforced through warning-free docs and review.
- Documentation follows `.ito/user-rust-style.md`: focus on why/when, avoid perfunctory restatements.

## Risks / Trade-offs

- Adding `missing_docs` warnings everywhere can create noisy churn. This change prefers targeted lint enabling plus documentation build hygiene.
- Improving docs may surface existing doc-test or markup issues; fixing them is part of the change.

## Verification

- `cargo test --workspace` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `make docs` (or `cargo doc --no-deps`) completes without warnings.
