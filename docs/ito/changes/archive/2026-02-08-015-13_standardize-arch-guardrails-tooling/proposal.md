## Why

The current `arch_guardrails.py` gives strict enforcement, but it relies on bespoke logic that is harder to maintain than standard Rust ecosystem tooling. We want to reduce custom policy code and adopt more idiomatic, reusable tooling while preserving most architectural safety guarantees.

## What Changes

- Replace most custom guardrail checks with standard tools and declarative config (primarily `cargo-deny`, Cargo feature/build checks, and optional `cargo-hack` for feature matrix confidence).
- Keep architecture enforcement in pre-commit and CI, but shift checks to generic commands that are easier to understand and maintain.
- Soften the most brittle guardrail (string-count baseline checks in `ito-domain`) to an 80/20 policy that prefers compiler/tooling-backed rules over ad-hoc text matching.
- Retain a minimal targeted custom check only if required to preserve critical required-edge guarantees not expressible cleanly in off-the-shelf tools.

## Capabilities

### Modified Capabilities

- `repo-precommit-quality-gates`: Update gate definitions to run ecosystem-native architecture checks instead of relying primarily on a bespoke Python guardrail script.
- `rust-clippy-policy`: Clarify or extend lint-oriented policy for domain-layer restrictions where practical, replacing brittle baseline counting with idiomatic lint/test enforcement.

## Impact

- Affected code: `ito-rs/tools/arch_guardrails.py`, Makefile targets, CI workflows, and pre-commit (`prek`) hook configuration.
- Affected dependencies/tooling: addition or formal adoption of Rust policy tools (for example `cargo-deny`, `cargo-hack`) and corresponding config files.
- Expected outcome: lower maintenance overhead and more idiomatic Rust workflow with acceptable strictness trade-off (targeting roughly 80% coverage of current bespoke checks).
