## ADDED Requirements

### Requirement: Lifecycle-stage workflow guidance

Agent instruction artifacts SHALL present proposal, execution, and review as a coherent staged workflow for change delivery.

#### Scenario: Proposal stage includes research framing

- **WHEN** a user runs `ito agent instruction proposal --change <change-id>`
- **THEN** the output SHALL include guidance for structured research inputs that improve proposal quality
- **AND** it SHALL explain how research outcomes feed specs and design artifacts

#### Scenario: Apply stage includes structured execution guidance

- **WHEN** a user runs `ito agent instruction apply --change <change-id>`
- **THEN** the output SHALL provide structured execution guidance equivalent in intent to legacy execute workflows
- **AND** it SHALL direct progress tracking through `ito tasks` commands
- **AND** it SHALL include checkpoints or pause guidance when human review is required

#### Scenario: Review is represented as a first-class stage

- **WHEN** a user runs `ito agent instruction review --change <change-id>`
- **THEN** the output SHALL position review as a stage in the proposal-to-archive lifecycle
- **AND** it SHALL describe expected review inputs and outputs relative to proposal/specs/tasks artifacts
