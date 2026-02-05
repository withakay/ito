## ADDED Requirements

### Requirement: Guidance is included in instruction artifacts

When `.ito/user-guidance.md` exists, `ito agent instruction <artifact>` SHALL include the guidance content in its output.

#### Scenario: Proposal instructions include guidance

- **GIVEN** `.ito/user-guidance.md` contains guidance text
- **WHEN** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the output includes a user guidance block
- **AND** the block includes the guidance text

#### Scenario: No guidance file means no injected section

- **GIVEN** `.ito/user-guidance.md` does not exist
- **WHEN** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the output does not include a user guidance block

### Requirement: Schema instructions remain authoritative

User guidance MUST be treated as additive and MUST NOT weaken or contradict schema-defined requirements.

#### Scenario: Conflicting guidance does not change requirements

- **GIVEN** the schema requires a specific section or format
- **AND** the user guidance requests a different format
- **WHEN** an instruction artifact is generated
- **THEN** schema-required content remains present and unchanged
