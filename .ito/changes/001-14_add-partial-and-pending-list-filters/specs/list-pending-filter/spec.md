## ADDED Requirements

### Requirement: Pending progress filter flag

The `ito list` command SHALL support a `--pending` flag that filters output to show only changes where no tasks have been completed yet.

#### Scenario: Filter to pending changes

- **WHEN** user runs `ito list --pending`
- **THEN** only changes with 0 tasks complete (out of N total, where N > 0) are displayed

#### Scenario: Exclude changes with any progress

- **WHEN** user runs `ito list --pending`
- **THEN** changes with 1 or more tasks complete are NOT displayed

#### Scenario: Handle changes with no tasks

- **WHEN** user runs `ito list --pending`
- **THEN** changes with no tasks defined are NOT displayed (they are not actionable pending work)

### Requirement: Mutual exclusivity with other progress filters

The `--pending` flag SHALL be mutually exclusive with `--completed` and `--partial` flags.

#### Scenario: Error on conflicting flags

- **WHEN** user runs `ito list --pending --partial`
- **THEN** the CLI SHALL display an error indicating the flags are mutually exclusive
