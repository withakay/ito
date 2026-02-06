# Refactor: Architecture guardrails (make + prek + CI)

## Why

The refactor work in this module needs safety rails. Without guardrails, architecture drift will continue (or regress during migration), and reviewers will have to manually re-audit layering over and over.

## What

- Add a single canonical entrypoint: `make arch-guardrails`.
- Implement checks that enforce (at minimum):
  - crate-edge rules (e.g., `ito-domain` must not depend on adapters)
  - domain API bans (e.g., no new direct `std::fs` / `std::process::Command` usage in `ito-domain`)
- Enforce guardrails in both:
  - `prek` (pre-commit)
  - CI

## Notes

- The initial implementation may use baseline/allowlist-style enforcement to prevent new violations while existing violations are migrated away in later changes.
- This change does not move any production code between crates; it only adds protections and a repeatable workflow.

## Verification

- `make arch-guardrails`
- `prek run --all-files`
