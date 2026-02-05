# Cli Artifact Workflow Specification

## Purpose

Define the `cli-artifact-workflow` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Worktree-aware apply instructions

When a change is applied in worktree workspace mode, `ito instructions apply` SHALL include deterministic instructions that create (or reuse) a worktree for the change branch and tell the agent which directory to work in.

#### Scenario: Apply instructions include worktree script
- **GIVEN** worktree workspace mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include a copy/pasteable shell snippet that:
  - Ensures `./main` exists and is on the default branch
  - Creates or reuses a worktree directory for the change at a stable path (e.g., `./changes/<id>`)
  - Prints the expected working directory for subsequent commands

#### Scenario: Apply instructions include local file copy step
- **GIVEN** worktree workspace mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include a step to copy configured local environment files from `./main` into the change worktree
- **AND** missing files are treated as non-fatal (copy what exists)
