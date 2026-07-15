# Validation And Quality Gates

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-14
source_refs:
- .ito/specs/cli-validate/spec.md
- .ito/specs/validate-repo-cli-surface/spec.md
- .ito/specs/repo-precommit-quality-gates/spec.md
- .ito/specs/rust-documentation-standards/spec.md
- .ito/specs/rust-foundations/spec.md
- .ito/changes/archive/2026-07-14-031-06_migrate-ito-authority-and-release/evidence/release-verification.md
known_gaps: []
```

Validation spans Ito artifact validation, repository guardrails, Rust checks,
markdown checks, docs checks, coverage, and architecture boundaries.

## Practical Gate Sequence

- `ito validate <change-id> --strict` for change package integrity.
- Focused Rust tests for touched behavior before repo-wide checks.
- `make test` for the shipping default package graph.
- `make check` for the shipping pre-push lane: markdownlint, fmt, clippy,
  rustdoc, coverage, affected tests, max-lines, architecture guardrails, and
  cargo-deny.
- `make check-experimental` for the independent all-features lane.

## Current Expectations

Coverage has an 80 percent hard floor. Source file size guardrails enforce a
1200-line hard limit for Rust source files and warn over the soft limit.
Architecture guardrails protect crate boundaries and prevent accidental
dependency regressions.
