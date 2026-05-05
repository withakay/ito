## ADDED Requirements

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
