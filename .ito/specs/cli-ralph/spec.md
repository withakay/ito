## ADDED Requirements

### Requirement: Interactive change selection

When `ito ralph` is executed without `--change`, the system SHALL prompt the user to select one or more changes to run Ralph against, unless `--no-interactive` is set.

#### Scenario: Select one change when no target is provided

- **GIVEN** `ito ralph` is executed with no `--change` and no `--module`
- **AND** interactive mode is enabled (default)
- **WHEN** the user selects exactly one change
- **THEN** the system SHALL run the Ralph loop for the selected change

#### Scenario: Select multiple changes and run sequentially

- **GIVEN** `ito ralph` is executed with no `--change`
- **AND** interactive mode is enabled (default)
- **WHEN** the user selects multiple changes
- **THEN** the system SHALL run the Ralph loop for each selected change, sequentially
- **AND** the system SHALL run changes in a stable order (the order presented in the selection list)

#### Scenario: Select changes within a module

- **GIVEN** `ito ralph --module <module-id>` is executed
- **AND** the module contains more than one change
- **WHEN** the user selects one or more changes
- **THEN** the system SHALL run the Ralph loop for each selected change, sequentially

#### Scenario: Cancellation exits cleanly

- **GIVEN** an interactive selection prompt is displayed
- **WHEN** the user cancels the prompt
- **THEN** the command SHALL exit with a non-zero exit code
- **AND** the command SHALL print a cancellation message

#### Scenario: No-interactive requires an explicit target

- **GIVEN** `--no-interactive` is set
- **WHEN** `ito ralph` is executed without `--change` and without `--module`
- **THEN** the command SHALL fail with an error explaining that `--change` is required

#### Scenario: Single-target actions prompt for exactly one change

- **GIVEN** `ito ralph` is executed without `--change`
- **AND** interactive mode is enabled (default)
- **AND** the command includes a single-target action flag (`--status`, `--add-context`, or `--clear-context`)
- **WHEN** the user selects a change
- **THEN** the system SHALL apply the action to the selected change
- **AND** the system SHALL NOT allow selecting more than one change for that prompt

#### Scenario: Archived changes are excluded

- **GIVEN** the repository contains archived changes under `.ito/changes/archive/`
- **WHEN** the interactive selection list is presented
- **THEN** the selection list SHALL NOT include archived changes

### Requirement: Interactive Ralph option selection

When `ito ralph` enters interactive change selection (no `--change`, no `--file`, and `--no-interactive` is not set), the system SHALL prompt the user for missing Ralph options with prefilled defaults.

- Options explicitly provided on the CLI SHALL NOT be re-prompted.
- The resolved options SHALL apply to all selected changes.

#### Scenario: Prompt for unset options with defaults

- **GIVEN** `ito ralph` is executed with no `--change` and no `--file`
- **AND** interactive mode is enabled (default)
- **AND** the user is prompted to select one or more changes
- **WHEN** the command is missing any of: `--harness`, `--model`, `--min-iterations`, `--max-iterations`, `--no-commit`, `--allow-all`, `--exit-on-error`
- **THEN** the system SHALL prompt for the missing values
- **AND** the prompts SHALL be prefilled with the current defaults

#### Scenario: Do not prompt when option is provided

- **GIVEN** `ito ralph` is executed with an explicit option value (for example `--max-iterations 3`)
- **WHEN** interactive change selection occurs
- **THEN** the system SHALL NOT prompt for that option
