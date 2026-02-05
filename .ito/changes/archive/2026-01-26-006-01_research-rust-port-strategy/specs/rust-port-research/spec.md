# Spec Delta: rust-port-research

## Purpose

Define the required research outputs and parity strategy that guide the Rust port.

## ADDED Requirements

### Requirement: Research artifacts exist and are maintained

The repository MUST include the required research outputs for the Rust port and keep them consistent with the current TypeScript CLI behavior.

#### Scenario: Required research files are present

- **WHEN** a developer inspects `.ito/research/`
- **THEN** the following files exist:
  - `.ito/research/SUMMARY.md`
  - `.ito/research/investigations/rust-cli-ux.md`
  - `.ito/research/investigations/parity-testing.md`
  - `.ito/research/investigations/rust-crate-architecture.md`
  - `.ito/research/investigations/packaging-distribution.md`

### Requirement: Parity strategy treats TypeScript as the oracle

The parity strategy MUST treat the existing TypeScript `ito` CLI as the behavior oracle and define test mechanisms for stdout, stderr, exit code, JSON shapes, and filesystem side effects.

#### Scenario: Parity testing plan covers non-mutating and mutating commands

- **WHEN** reading `.ito/research/investigations/parity-testing.md`
- **THEN** it specifies how to compare:
  - non-mutating commands (help/version/list/show/validate)
  - mutating commands (init/update/installers)
  - interactive flows via PTY where required
