<!-- ITO:START -->
# Ito Init

## Purpose

This spec defines the current behavior and requirements for ito init.

## Requirements

### Requirement: ito init emits a repo-validation advisory when at least one rule activates
After `ito init` and `ito init --upgrade` complete their primary work, the system SHALL emit a post-install advisory only when the resolved configuration activates at least one `ito validate repo` rule. The advisory SHALL name direct validation or instruction commands and MUST NOT delegate remediation to a retired helper skill.

#### Scenario: Active rule produces direct remediation
- **WHEN** initialization completes with at least one active repository-validation rule
- **THEN** the advisory names `ito validate repo`
- **AND** it identifies the direct CLI or emitted instruction that owns remediation
- **AND** it does not recommend `ito-update-repo`

#### Scenario: No active rule remains quiet
- **WHEN** initialization completes with no active repository-validation rule
- **THEN** no validation advisory is printed

### Requirement: Advisory names the detected pre-commit system

The advisory SHALL invoke `detect_pre_commit_system(project_root)` and include the detected system in its message so the user knows what the agent will configure on their behalf.

- **Requirement ID**: ito-init:advisory-detected-system

#### Scenario: Advisory states the detected prek system

- **GIVEN** the repo root contains `.pre-commit-config.yaml` and a prek toolchain marker
- **WHEN** `ito init` emits the advisory
- **THEN** the message SHALL state that the detected pre-commit system is `prek`

#### Scenario: Advisory states None when no system is detected

- **GIVEN** the repo has no supported pre-commit framework markers
- **WHEN** `ito init` emits the advisory
- **THEN** the message SHALL state that no pre-commit system was detected
- **AND** the message SHALL list the supported systems so the user can choose one
<!-- ITO:END -->
