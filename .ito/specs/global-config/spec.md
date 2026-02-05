# Global Config Specification

## Purpose

Define the `global-config` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Worktree workspace defaults

The system SHALL support user-level global configuration for worktree workspace behavior.

#### Scenario: Default branch selection
- **WHEN** worktree workspace mode requires a default branch
- **THEN** the system uses a configured default branch if present
- **AND** otherwise defaults to `main`
- **AND** falls back to `master` if `main` does not exist

#### Scenario: Default local file copy patterns
- **WHEN** creating a new change worktree
- **THEN** the system uses a configured list of local file patterns to copy
- **AND** the default list includes `.env`, `.envrc`, and a Mise local config file name
