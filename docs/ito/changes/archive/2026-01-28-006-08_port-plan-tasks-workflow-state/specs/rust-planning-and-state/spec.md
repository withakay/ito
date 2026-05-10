# Spec Delta: rust-planning-and-state

## Purpose

Port `plan`, `tasks`, `workflow`, and `state` commands to Rust with on-disk schema compatibility.

## ADDED Requirements

### Requirement: Workflow/state schemas are compatible

Rust MUST read and write workflow/state files compatible with the TS implementation.

#### Scenario: Roundtrip workflow YAML

- GIVEN a workflow YAML file produced by TypeScript
- WHEN Rust parses and re-serializes it
- THEN the semantic content is preserved

### Requirement: Command outputs match TypeScript

Rust MUST match TypeScript stdout/stderr/exit codes for planning and state commands.

#### Scenario: `tasks` output parity

- GIVEN a change with tasks
- WHEN the user runs `ito tasks --change <id>`
- THEN Rust output matches TypeScript
