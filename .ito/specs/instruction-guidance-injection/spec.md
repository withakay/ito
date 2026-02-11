## ADDED Requirements

### Requirement: Artifact-scoped guidance injection

Instruction generation SHALL inject artifact-scoped guidance from `.ito/user-prompts/<artifact-id>.md` when present.

#### Scenario: Proposal includes proposal-scoped guidance

- **GIVEN** `.ito/user-prompts/proposal.md` contains guidance text
- **WHEN** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the output includes the proposal-scoped guidance text

#### Scenario: Apply includes apply-scoped guidance

- **GIVEN** `.ito/user-prompts/apply.md` contains guidance text
- **WHEN** a user runs `ito agent instruction apply --change "<change-id>"`
- **THEN** the output includes the apply-scoped guidance text

#### Scenario: No artifact-scoped file falls back cleanly

- **GIVEN** `.ito/user-prompts/<artifact-id>.md` does not exist
- **WHEN** a user runs `ito agent instruction <artifact-id> --change "<change-id>"`
- **THEN** instruction generation proceeds without artifact-scoped guidance errors

### Requirement: Shared and scoped guidance composition

When both shared and scoped guidance are available, instruction output SHALL include both as additive guidance.

#### Scenario: Output includes both shared and scoped guidance

- **GIVEN** `.ito/user-prompts/guidance.md` and `.ito/user-prompts/proposal.md` both exist
- **WHEN** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the output includes guidance from both files
- **AND** schema-defined requirements remain unchanged and authoritative

#### Scenario: Legacy shared guidance is used when new shared file is absent

- **GIVEN** `.ito/user-prompts/guidance.md` does not exist
- **AND** `.ito/user-guidance.md` exists
- **WHEN** a user runs `ito agent instruction apply --change "<change-id>"`
- **THEN** the output includes shared guidance from `.ito/user-guidance.md`
