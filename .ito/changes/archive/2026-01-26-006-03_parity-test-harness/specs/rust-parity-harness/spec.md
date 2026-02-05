# Spec Delta: rust-parity-harness

## Purpose

Provide a deterministic parity testing harness that compares TypeScript `ito` behavior to Rust `ito` behavior.

## ADDED Requirements

### Requirement: Harness compares outputs and exit codes

The harness MUST be able to execute both CLIs and compare stdout, stderr, and exit code.

#### Scenario: Compare help output

- **WHEN** the harness runs `ito --help` via TypeScript and Rust
- **THEN** it records stdout/stderr and exit codes
- **AND** the parity test fails if any differ

### Requirement: Harness supports fixture repositories

The harness MUST run commands inside isolated fixture repos and support deterministic snapshots.

#### Scenario: Run list in a fixture repo

- **WHEN** the harness runs `ito list --json` in a fixture directory
- **THEN** it captures stable JSON output for comparison

### Requirement: Harness can test interactive flows via PTY

The harness MUST support PTY-driven tests for commands that require TTY interaction.

#### Scenario: Interactive command is executed under PTY

- **WHEN** a parity test marks a command as interactive
- **THEN** it runs it under a PTY and can feed deterministic input
