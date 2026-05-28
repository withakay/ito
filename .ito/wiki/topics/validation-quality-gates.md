# Validation And Quality Gates

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-05-27
source_refs:
  - docs/ito/specs/cli-validate/spec.md
  - docs/ito/specs/validate-repo-cli-surface/spec.md
  - docs/ito/specs/repo-precommit-quality-gates/spec.md
  - docs/ito/specs/rust-documentation-standards/spec.md
  - docs/ito/specs/rust-foundations/spec.md
known_gaps: []
```

Validation spans Ito artifact validation, repository guardrails, Rust checks,
markdown checks, docs checks, coverage, and architecture boundaries.

## Practical Gate Sequence

- `ito validate <change-id> --strict` for change package integrity.
- Focused Rust tests for touched behavior before repo-wide checks.
- `make test` for the workspace test suite.
- `make check` for pre-push hooks: markdownlint, fmt, clippy, rustdoc,
  coverage, affected tests, max-lines, architecture guardrails, and cargo-deny.

## Current Expectations

Coverage has an 80 percent hard floor. Source file size guardrails enforce a
1200-line hard limit for Rust source files and warn over the soft limit.
Architecture guardrails protect crate boundaries and prevent accidental
dependency regressions.
