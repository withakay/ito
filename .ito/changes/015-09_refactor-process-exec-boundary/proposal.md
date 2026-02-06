# Refactor: Centralize process execution behind a core boundary

## Why

- Process execution is a high-risk side effect (environment, cwd, stdout/stderr, exit codes).
- Scattered `std::process::Command` usage makes it hard to test and hard to enforce layering.
- A single boundary makes it easier to capture structured output and to present consistent errors.

## What

- Introduce a process execution boundary in `ito-core` (for example: a `ProcessRunner` trait + default implementation).
- Migrate existing process execution call sites to use the boundary.
- Enforce guardrails that prevent process execution from leaking into `ito-domain`.

## Scope

- Core-side process execution for workflow/runner style features (e.g., ralph/harness runners).
- Does not change the CLI UX; adapters remain responsible for formatting output.

## Depends on

- 015-01_refactor-arch-guardrails
- 015-08_refactor-error-boundaries (recommended, so process failures map cleanly to use-case errors)

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
