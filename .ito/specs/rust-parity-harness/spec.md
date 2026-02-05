# Rust Parity Harness Specification

## Purpose

Define the `rust-parity-harness` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Parity harness covers init behavior

The Rust parity harness SHALL include parity tests for `init` that compare Rust behavior against the TypeScript CLI for both:

- Non-interactive runs using `--tools`.
- Interactive runs using a PTY-driven harness.

#### Scenario: Parity test for non-interactive init

- **WHEN** the parity harness runs `ito init --tools all` and `itors init --tools all` against the same fixture repo
- **THEN** the harness reports success only if both produce equivalent installed artifacts (modulo known/declared normalizations)

#### Scenario: Parity test for interactive init

- **WHEN** the parity harness drives an interactive `init` session in both CLIs via PTY
- **THEN** the harness reports success only if the resulting configured artifacts are equivalent
