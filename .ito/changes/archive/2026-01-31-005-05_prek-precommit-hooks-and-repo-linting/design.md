## Context

- The repo already exposes Rust linting via `make lint` (currently `cargo fmt --check` and `cargo clippy ... -D warnings`).
- There is no repo-wide pre-commit configuration today.
- prek is a drop-in replacement for pre-commit and can run an existing `.pre-commit-config.yaml` unchanged.

## Goals / Non-Goals

**Goals:**

- Provide a single, documented local entrypoint for pre-commit checks via `prek`.
- Add common quality gates for Rust + Markdown/JSON/YAML + whitespace/line endings.
- Define a clippy policy that is consistent, intentional, and aligned with this repo's Rust style expectations.
- Keep the workflow reproducible across contributors and CI.

**Non-Goals:**

- Redesign the Rust codebase to satisfy every possible lint (the policy should be curated; local `allow` is acceptable when justified).
- Require editors/IDEs to be configured a certain way (we rely on repo hooks + CLI entrypoints).

## Decisions

- **Use `.pre-commit-config.yaml` as the source config**: prek can run pre-commit configs unchanged, which keeps the ecosystem of existing hooks available and lowers migration cost.
- **Prefer repo-local hook entrypoints for Rust**: run `cargo fmt` / `cargo clippy` directly so the hook behavior matches `make lint` and workspace structure.
- **Prefer Python-distributed hooks for text formats**: use pre-commit hook repos that install their own Python tooling, avoiding Node/Go dependencies for the initial rollout.
  - Hygiene + validation: `pre-commit/pre-commit-hooks` (whitespace, EOF newline, mixed line endings, JSON/YAML syntax checks).
  - Markdown: `markdownlint-cli2` linting only (no auto-formatting) to avoid destructive rewrites of Ito task/spec artifacts.
  - JSON formatting: `pretty-format-json`.
  - YAML: validate + lint (`check-yaml` + `yamllint`); no automatic YAML formatting in the initial rollout.
- **Curated clippy policy**: enable additional clippy lint groups selectively (rather than blanket-enabling `clippy::restriction`) and document the rationale + escape hatches (`#[allow(...)]`).
- **Clippy policy rollout strategy**: start with the existing `-D warnings` baseline plus a small set of high-signal clippy lints (deny `dbg_macro`, `todo`, and `unimplemented`), then iterate if more style enforcement is warranted.
- **Keep "fast" checks in pre-commit**: avoid overly slow checks on every commit; consider heavier checks in CI or pre-push if needed.

## Risks / Trade-offs

- \[More tooling for contributors\] → Provide clear install/run docs (`prek run`, `prek install`) and keep configs minimal.
- \[Hook runtime becomes slow\] → Keep hook set curated; consider running clippy on staged files only where feasible, or moving expensive checks to CI/pre-push.
- \[Lint policy churn/noise\] → Start with a small, high-signal clippy set; document how to add/remove lints.

## Migration Plan

1. Add `.pre-commit-config.yaml` and document `prek` usage.
1. Add/adjust make targets (optional) to run the same checks.
1. (Optional) Update CI to run `prek run --all-files` (or equivalent) so CI matches local hooks.

## Open Questions

- None.
