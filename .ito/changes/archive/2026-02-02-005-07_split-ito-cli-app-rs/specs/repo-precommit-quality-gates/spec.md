## ADDED Requirements

### Requirement: Repo prevents oversized source files

The repository SHALL prevent source files from growing beyond a maintainability limit.

#### Scenario: ito-cli Rust sources stay under the per-file limit

- **GIVEN** the repository contains `ito-rs/crates/ito-cli/src/**/*.rs`
- **WHEN** quality gates run (tests and/or pre-commit hooks)
- **THEN** they SHALL fail if any file exceeds the configured per-file size limit
- **AND** the default limit is 1000 (SLOC or strict lines, as documented)
