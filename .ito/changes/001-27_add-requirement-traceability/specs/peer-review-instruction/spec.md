## ADDED Requirements

### Requirement: Review instructions include traceability context

When a change provides requirement traceability metadata, the review instruction SHALL include a computed traceability summary covering covered requirements, uncovered requirements, and unresolved task references.

#### Scenario: Review instruction shows uncovered requirement gap

- **GIVEN** a change declares requirement ids and enhanced task references
- **AND** at least one declared requirement is uncovered
- **WHEN** an agent runs `ito agent instruction review --change <id>`
- **THEN** the instruction output includes the uncovered requirement in its review context
- **AND** prompts the reviewer to verify whether the task plan should be revised before implementation
