## ADDED Requirements

### Requirement: Partial progress filter flag

The `ito list` command SHALL support a `--partial` flag that filters output to show only changes where task completion is partial (at least one task complete, but not all tasks complete).

#### Scenario: Filter to partially complete changes

- **WHEN** user runs `ito list --partial`
- **THEN** only changes with 1 to N-1 tasks complete (out of N total) are displayed

#### Scenario: Exclude changes with no progress

- **WHEN** user runs `ito list --partial`
- **THEN** changes with 0 tasks complete are NOT displayed

#### Scenario: Exclude fully complete changes

- **WHEN** user runs `ito list --partial`
- **THEN** changes with all tasks complete are NOT displayed

#### Scenario: Handle changes with no tasks

- **WHEN** user runs `ito list --partial`
- **THEN** changes with no tasks defined are NOT displayed (they have no partial progress)

### Requirement: Mutual exclusivity with other progress filters

The `--partial` flag SHALL be mutually exclusive with `--completed` and `--pending` flags.

#### Scenario: Error on conflicting flags

- **WHEN** user runs `ito list --partial --completed`
- **THEN** the CLI SHALL display an error indicating the flags are mutually exclusive
