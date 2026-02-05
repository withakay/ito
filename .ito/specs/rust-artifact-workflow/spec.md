# rust-artifact-workflow Specification

## Purpose

TBD - created by archiving change 006-07_port-artifact-workflow-commands. Update Purpose after archive.

## Requirements

### Requirement: `create module` matches TS

Rust MUST write the same module structure and emit matching output.

#### Scenario: Create a module

- GIVEN a repository with existing modules
- WHEN the user runs `ito create module "my-module"`
- THEN Rust creates the same directory structure as TypeScript
- AND stdout/stderr/exit code match TypeScript

### Requirement: `create change` matches TS

Rust MUST scaffold changes with the same naming and numbering rules.

#### Scenario: Create a change under a module

- GIVEN a module ID
- WHEN the user runs `ito create change "my-change" --module <id>`
- THEN Rust creates the same change directory and `.ito.yaml` as TypeScript

### Requirement: `status` and `instructions` match TS output

Rust MUST render the same status and instruction text as TypeScript.

#### Scenario: Show instructions for a proposal

- GIVEN a change directory
- WHEN the user runs `ito agent instruction proposal --change <change-id>`
- THEN Rust prints the same instructions as TypeScript
