## ADDED Requirements

### Requirement: Review instructions include traceability context

When a change provides requirement traceability metadata, the review instruction SHALL include a computed traceability summary covering covered requirements, uncovered requirements, and unresolved task references.

#### Scenario: Review instruction shows uncovered requirement gap

- **GIVEN** a change declares requirement ids and enhanced task references
- **AND** at least one declared requirement is uncovered
- **WHEN** an agent runs `ito agent instruction review --change <id>`
- **THEN** the instruction output includes the uncovered requirement in its review context
- **AND** prompts the reviewer to verify whether the task plan should be revised before implementation

#### Scenario: Review instruction reports unavailable traceability

- **GIVEN** a change declares requirement ids
- **AND** its active tracking file does not use enhanced task encoding
- **WHEN** an agent runs `ito agent instruction review --change <id>`
- **THEN** the instruction output explains that computed requirement coverage is unavailable
- **AND** prompts the reviewer to confirm whether the change should migrate to enhanced task tracking before implementation
