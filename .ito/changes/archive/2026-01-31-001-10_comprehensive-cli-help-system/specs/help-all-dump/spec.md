## ADDED Requirements

### Requirement: CLI supports complete help dump

The system SHALL support outputting complete help documentation for all commands and subcommands in a single operation.

#### Scenario: Dump all help via help command

- **WHEN** user runs `ito help --all`
- **THEN** the system SHALL output help text for every command and subcommand
- **AND** the output SHALL be formatted with clear section headers
- **AND** the output SHALL be suitable for terminal display or piping to a file

#### Scenario: Dump all help via global flag

- **WHEN** user runs `ito --help-all`
- **THEN** the system SHALL output the same complete help as `ito help --all`

#### Scenario: Help dump includes nested subcommands

- **WHEN** the complete help is dumped
- **THEN** commands with subcommands (e.g., `agent instruction`, `tasks status`) SHALL have their subcommand help included
- **AND** the hierarchy SHALL be visually indicated (e.g., indentation or section nesting)

### Requirement: Help dump supports machine-readable format

The system SHALL support JSON output for programmatic consumption of the complete CLI reference.

#### Scenario: JSON help dump

- **WHEN** user runs `ito help --all --json`
- **THEN** the system SHALL output a JSON structure containing all commands, their options, and subcommands
- **AND** each command entry SHALL include: name, description, options array, subcommands array

#### Scenario: JSON schema structure

- **WHEN** JSON help is requested
- **THEN** each option SHALL include: name, short flag (if any), description, required boolean, default value (if any)
