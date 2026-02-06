## MODIFIED Requirements

### Requirement: Command Execution

The `ito list` command SHALL scan and analyze either active changes or specs based on the selected mode.

#### Scenario: Scanning for changes (default)

- **WHEN** `ito list` is executed without flags
- **THEN** scan the `.ito/changes/` directory for change directories
- **AND** exclude the `archive/` subdirectory from results
- **AND** parse each change's `tasks.md` file to count task completion

#### Scenario: Scanning for specs

- **WHEN** `ito list --specs` is executed
- **THEN** scan the `.ito/specs/` directory for capabilities
- **AND** read each capability's `spec.md`
- **AND** parse requirements to compute requirement counts

### Requirement: Error Handling

The command SHALL gracefully handle missing files and directories with appropriate messages.

#### Scenario: Missing tasks.md file

- **WHEN** a change directory has no `tasks.md` file
- **THEN** display the change with "No tasks" status

#### Scenario: Missing changes directory

- **WHEN** `.ito/changes/` directory doesn't exist
- **THEN** display error: "No Ito changes directory found. Run 'ito init' first."
- **AND** exit with code 1

### Requirement: Sorting

The command SHALL support a `--sort` flag to control ordering of list results.

#### Scenario: Default sort is recent

- **WHEN** `ito list` is executed without `--sort`
- **THEN** it sorts changes by `recent`

#### Scenario: Sorting changes by name

- **GIVEN** multiple changes exist
- **WHEN** `ito list --sort name` is executed
- **THEN** sort them in alphabetical order by change name

#### Scenario: Sorting changes by recent

- **GIVEN** multiple changes exist
- **WHEN** `ito list --sort recent` is executed
- **THEN** order changes from most recent to least recent
