# rust-view-and-validate Specification

## Purpose

TBD - created by archiving change 006-05_port-list-show-validate. Update Purpose after archive.

## Requirements

### Requirement: `list` matches output and JSON shapes

The Rust CLI MUST match TypeScript for `ito list` output, exit codes, and `--json` shapes.

#### Scenario: List modules in JSON mode

- GIVEN a repository with modules
- WHEN the user runs `ito list --modules --json`
- THEN Rust prints JSON matching TypeScript (fields, types)
- AND exit code matches TypeScript

### Requirement: `show` matches errors and renderings

The Rust CLI MUST match TypeScript for `ito show` outputs and errors.

#### Scenario: Show a missing change

- GIVEN a repository without the requested change
- WHEN the user runs `ito show <missing-id>`
- THEN Rust prints the same error message as TypeScript
- AND exit code matches TypeScript

### Requirement: `validate` matches strictness and JSON

The Rust CLI MUST match TypeScript for `ito validate` in both default and `--strict` modes.

#### Scenario: Strict validation fails on warnings

- GIVEN a repository that produces validation warnings
- WHEN the user runs `ito validate --strict`
- THEN Rust exits with the same code as TypeScript
- AND Rust prints the same warnings/errors as TypeScript
