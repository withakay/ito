## ADDED Requirements

### Requirement: Instruction generation does not require remote coordination sync

Instruction generation MUST remain usable offline. The system MUST NOT require a remote coordination-branch fetch to print apply instructions.

#### Scenario: Apply instruction generation skips coordination sync by default

- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system does not fetch the coordination branch from `origin` by default

### Requirement: Remote coordination sync is available as an opt-in preflight

The system SHALL allow users to opt in to a coordination-branch fetch preflight before apply instructions are printed.

#### Scenario: Coordination sync preflight is enabled

- **GIVEN** coordination-branch sync preflight is enabled
- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system attempts to fetch the coordination branch from `origin` before printing instructions
