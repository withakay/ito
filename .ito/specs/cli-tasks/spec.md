## ADDED Requirements

### Requirement: ID-ordered task and change lists

The CLI SHALL emit deterministic ascending ID order for ID-bearing task list outputs.

#### Scenario: Status ready and blocked lists are task-ID ordered

- **WHEN** executing `ito tasks status <change-id>`
- **THEN** ready tasks are output in ascending canonical task ID order
- **AND** blocked tasks are output in ascending canonical task ID order

#### Scenario: Ready command across changes is ID ordered

- **WHEN** executing `ito tasks ready` without a specific change ID
- **THEN** changes are output in ascending canonical change ID order
- **AND** each change's `ready_tasks` list is output in ascending canonical task ID order

#### Scenario: Show JSON task list is task-ID ordered

- **WHEN** executing `ito tasks show <change-id> --json`
- **THEN** `tasks` are output in ascending canonical task ID order
- **AND** `waves` are output in ascending wave number order
