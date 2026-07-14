## Purpose

Provide reusable, consistent CLI plumbing for the Rust CLI so command handlers share the same patterns for:

- reporting errors
- printing diagnostics (including locations)
- exit codes

This reduces duplication and makes command behavior consistent.

## ADDED Requirements

### Requirement: Command handlers use a shared Result-based flow

Command handlers SHALL return a `Result`-style value and SHALL NOT perform ad-hoc `exit(1)` in multiple places.

#### Scenario: Single failure path

- **GIVEN** a command handler encounters a validation error
- **WHEN** the handler returns an error
- **THEN** a single shared layer prints the error and exits non-zero

### Requirement: Diagnostics printing is consistent

The CLI SHALL provide a shared function for printing a diagnostic that includes file path and optional line number.

#### Scenario: Diagnostic includes line location

- **GIVEN** a diagnostic includes a line number
- **WHEN** it is printed
- **THEN** it includes `path:line` in the message

#### Scenario: Diagnostic without line location

- **GIVEN** a diagnostic does not include a line number
- **WHEN** it is printed
- **THEN** it includes the path without a line suffix

### Requirement: Blocking validation errors are handled uniformly

When a command operates on a file that has validation errors, the CLI SHALL fail without modifying the file.

#### Scenario: Tasks command blocks on invalid tasks.md

- **GIVEN** a tasks file has validation errors
- **WHEN** executing a tasks subcommand that would modify tasks.md
- **THEN** the command exits non-zero
- **AND** the command prints all validation errors
- **AND** tasks.md is not modified
