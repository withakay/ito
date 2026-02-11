## ADDED Requirements

### Requirement: Artifact-scoped user prompt files

Ito SHALL support optional artifact-scoped user prompt files under `.ito/user-prompts/` using file names that match instruction artifact IDs.

#### Scenario: Proposal prompt file is recognized

- **WHEN** `.ito/user-prompts/proposal.md` exists
- **AND** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the proposal-scoped prompt file is considered for guidance injection

#### Scenario: Apply prompt file is recognized

- **WHEN** `.ito/user-prompts/apply.md` exists
- **AND** a user runs `ito agent instruction apply --change "<change-id>"`
- **THEN** the apply-scoped prompt file is considered for guidance injection

#### Scenario: Unknown artifact file does not affect other artifacts

- **WHEN** `.ito/user-prompts/<artifact-id>.md` exists for an artifact different from the current command
- **THEN** that file is ignored for the current instruction output
