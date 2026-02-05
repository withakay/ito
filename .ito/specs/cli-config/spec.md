# Cli Config Specification

## Purpose

Define the `cli-config` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Configure worktree workspace defaults

The config command SHALL allow setting and retrieving configuration keys related to worktree workspace behavior.

#### Scenario: Set default branch for worktrees
- **WHEN** the user executes `ito config set worktrees.defaultBranch <value>`
- **THEN** Ito stores the value in global configuration

#### Scenario: Set local file copy patterns
- **WHEN** the user executes `ito config set worktrees.localFiles <json-array>`
- **THEN** Ito stores the list in global configuration
- **AND** the list is used when generating worktree-aware apply instructions
