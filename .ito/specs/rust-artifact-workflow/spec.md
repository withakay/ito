# Spec: rust-artifact-workflow

## Purpose

Define the `rust-artifact-workflow` capability and its current-truth behavior. This spec captures requirements and scenarios (for example: `create module` matches TS).

## Requirements

### Requirement: `create module` matches TS

Rust MUST write the same module structure and emit matching output.

#### Scenario: Create a module

- GIVEN a repository with existing modules
- WHEN the user runs `ito create module "my-module"`
- THEN Rust creates the same directory structure as TypeScript
- AND stdout/stderr/exit code match TypeScript

#### Scenario: Create a module with description argument

- GIVEN a repository with existing modules
- WHEN the user runs `ito create module "my-module" --description "My module description"`
- THEN Rust writes module metadata with the provided description text
- AND Rust output and exit behavior match TypeScript for the same command
