## ADDED Requirements

### Requirement: Project Validation Contract

Ito SHALL expose a deterministic project validation command.

#### Scenario: Run configured checks

- **WHEN** a user runs `ito check --json`
- **THEN** Ito SHALL run the configured validation plan
- **AND** the JSON response SHALL include command source, commands, result, duration, and concise failure excerpts.

#### Scenario: No explicit validation config

- **WHEN** no Ito validation config exists
- **THEN** Ito SHALL infer a safe default from repository files such as `Makefile`, package manifests, or project guidance
- **AND** the response SHALL include that the command was inferred.

### Requirement: Affected Test Plan

Ito SHALL expose a command for determining and running affected tests when possible.

#### Scenario: Affected plan available

- **WHEN** Ito can map changed files to targeted tests
- **THEN** `ito test affected --json` SHALL run or report the targeted plan.

#### Scenario: Affected plan unavailable

- **WHEN** Ito cannot safely determine affected tests
- **THEN** the command SHALL fall back to the configured full validation plan or report the fallback command.

### Requirement: CI Doctor

Ito SHALL summarize CI failures for agent consumption.

#### Scenario: Diagnose failing GitHub Actions run

- **WHEN** a user runs `ito doctor ci --json`
- **THEN** Ito SHALL inspect the current branch or PR CI state when available
- **AND** Ito SHALL return failed jobs, failed steps, links, and concise actionable excerpts.

### Requirement: Shared Validation For Agent Workflows

Agent workflows SHALL use the validation contract instead of inferring commands independently.

#### Scenario: Ralph validates completion

- **WHEN** Ralph detects a completion promise
- **THEN** Ralph SHALL run or consume `ito check --json` unless validation is explicitly disabled by configuration.
