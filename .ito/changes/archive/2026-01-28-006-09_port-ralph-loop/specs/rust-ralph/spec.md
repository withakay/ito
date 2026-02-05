# Spec Delta: rust-ralph

## Purpose

Port `ito ralph` / `ito loop` to Rust with identical loop semantics, state layout, and completion detection.

## ADDED Requirements

### Requirement: Completion promise detection matches TypeScript

Rust MUST detect completion promises using the same rules as TypeScript.

#### Scenario: Detect `<promise>COMPLETE</promise>`

- GIVEN harness output containing `<promise>COMPLETE</promise>`
- WHEN the loop processes the output
- THEN Rust stops after meeting `--min-iterations` semantics

### Requirement: State is written under `.ito/.state/ralph/<change>`

Rust MUST write loop state and history in the same location and structure as TypeScript.

#### Scenario: State files exist

- GIVEN a completed loop run
- WHEN the user inspects `.ito/.state/ralph/<change-id>/`
- THEN the expected state and history files exist

### Requirement: Tests do not require network

Rust tests MUST run with stub harnesses.

#### Scenario: Parity tests run offline

- GIVEN no network access
- WHEN `cargo test --workspace` runs
- THEN ralph tests pass using stub harnesses
