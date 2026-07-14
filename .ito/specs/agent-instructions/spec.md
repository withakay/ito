# Agent Instructions

## Purpose

This spec defines the current behavior and requirements for agent instructions.

## Requirements

### Requirement: Apply instruction generation is offline-friendly by default

`ito agent instruction apply --change <id>` MUST NOT perform network I/O by default.

#### Scenario: Apply instructions do not fetch by default

- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system SHALL NOT run `git fetch` (or other network operations) as part of instruction generation

### Requirement: Coordination sync can be explicitly requested for apply instructions

The system SHALL allow users to opt in to coordination branch synchronization when generating apply instructions.

#### Scenario: Apply instructions sync coordination branch when requested

- **GIVEN** coordination-branch sync is explicitly enabled for apply instructions
- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system SHALL attempt to fetch the configured coordination branch from `origin` before printing instructions
- **AND** instruction generation continues (with warning) if the remote branch is missing

### Requirement: Migrate-to-main instruction is always available

The standard Ito binary SHALL embed and render `ito agent instruction migrate-to-main` even when backend and coordination-branch runtime features are not compiled.

- **Requirement ID**: agent-instructions:migrate-to-main-availability
- **Tags**: behavior

#### Scenario: Standard build renders migration instruction

- **GIVEN** Ito was built without backend and coordination-branch features
- **WHEN** a user runs `ito agent instruction migrate-to-main`
- **THEN** Ito renders the complete migration prompt successfully

### Requirement: Legacy diagnostics name one remediation

Every warning or blocking diagnostic produced by legacy coordination detection SHALL name the exact command `ito agent instruction migrate-to-main` and SHALL explain whether the attempted operation was allowed as a read or rejected as a write.

- **Requirement ID**: agent-instructions:legacy-coordination-remediation
- **Tags**: behavior

#### Scenario: Read warning identifies remediation

- **GIVEN** a read-only command is allowed against legacy state
- **WHEN** Ito prints the legacy-state warning
- **THEN** the warning includes `ito agent instruction migrate-to-main`
- **AND** states that the current operation remained read-only

#### Scenario: Write error identifies remediation

- **GIVEN** a mutating command is rejected against legacy state
- **WHEN** Ito prints the blocking error
- **THEN** the error includes `ito agent instruction migrate-to-main`
- **AND** states that no mutation occurred
