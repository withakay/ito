## Why

The repo currently relies on ad-hoc local tooling for formatting and linting, which makes it easy for inconsistencies to slip into commits and for contributors to have different local outcomes.
Adding a standard pre-commit workflow (via prek) creates fast, repeatable quality gates for Rust and common text formats before changes land.

## What Changes

- Adopt `prek` as the supported pre-commit runner (drop-in compatible with `pre-commit`).
- Add a repo-level `.pre-commit-config.yaml` to run common checks for Rust, Markdown, JSON, YAML, and line endings/whitespace.
- Wire Rust checks through the same workflow (format + clippy), and define a consistent clippy policy aligned with this repo's Rust style guidance.
- Add/adjust developer documentation and make targets so running the same checks locally and in CI is straightforward.

## Capabilities

### New Capabilities

- `repo-precommit-quality-gates`: Standardized pre-commit style checks and formatting using `prek` + `.pre-commit-config.yaml`.
- `rust-clippy-policy`: A curated, documented clippy lint policy (including configuration) that can be run consistently in hooks and CI.

### Modified Capabilities

- (none)

## Impact

- Developer workflow: contributors install/run `prek` (and optionally install git hooks) to get consistent checks locally.
- Repo config: new `.pre-commit-config.yaml` and related tooling/config files.
- CI: may run the same `prek`/lint steps to ensure parity with local checks.
