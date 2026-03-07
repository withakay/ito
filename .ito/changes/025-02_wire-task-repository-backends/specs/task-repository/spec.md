## ADDED Requirements

### Requirement: Task persistence is runtime-selected for reads and mutations

Task state SHALL be resolved through the selected task persistence implementation for the current runtime mode, and task mutations SHALL persist through that same selected implementation.

#### Scenario: Remote mode reads task state without local tasks markdown

- **GIVEN** remote persistence mode is active
- **AND** task state exists in the selected remote-backed implementation
- **WHEN** a caller loads task state for a change
- **THEN** the task persistence layer returns that task state without requiring local `tasks.md`

#### Scenario: Remote mode mutations do not edit tasks markdown directly

- **GIVEN** remote persistence mode is active
- **WHEN** a task mutation is performed
- **THEN** Ito persists the mutation through the selected remote-backed task persistence path
- **AND** it does not require direct local markdown editing as the primary write path
