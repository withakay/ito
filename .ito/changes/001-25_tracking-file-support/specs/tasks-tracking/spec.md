## MODIFIED Requirements

### Requirement: Tasks tracking supports checkbox encoding

The tasks tracking format MUST support a checkbox-list encoding.

In checkbox encoding, a task SHALL be represented by a markdown list item beginning with one of:

- `- [ ]` (pending)
- `- [x]` (complete)
- `- [~]` (in-progress)
- `- [>]` (in-progress alias)

#### Scenario: Checkbox tasks are recognized

- **WHEN** a tracking file contains checkbox-list items using the supported markers
- **THEN** the system recognizes those items as tasks
- **AND** it assigns each one a status consistent with the marker

### Requirement: Declared tracking files contain at least one task

If a file is validated as a tasks tracking file, it MUST contain at least one recognizable task.

#### Scenario: Empty tracking file is a warning in non-strict mode

- **GIVEN** a tracking file is validated as `ito.tasks-tracking.v1`
- **AND** it contains no checkbox tasks and no enhanced task blocks
- **WHEN** executing validation in non-strict mode
- **THEN** validation emits a warning with an actionable message

#### Scenario: Empty tracking file is an error in strict mode

- **GIVEN** a tracking file is validated as `ito.tasks-tracking.v1`
- **AND** it contains no checkbox tasks and no enhanced task blocks
- **WHEN** executing validation in strict mode
- **THEN** validation emits an error with an actionable message
