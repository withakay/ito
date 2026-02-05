## MODIFIED Requirements

### Requirement: CLI argument parsing infrastructure

The CLI SHALL use clap's derive API for argument parsing, replacing the hand-rolled parsing implementation while preserving all existing command names, flags, and behaviors.

#### Scenario: Top-level command parsing

- **WHEN** user executes `ito <command>`
- **THEN** the system SHALL parse the command using clap's derive macros
- **AND** SHALL dispatch to the appropriate command handler
- **AND** SHALL preserve the exact command names from the existing implementation

#### Scenario: Subcommand parsing

- **WHEN** user executes `ito tasks <subcommand>`
- **THEN** the system SHALL parse the subcommand using nested clap subcommands
- **AND** SHALL preserve the exact subcommand names from the existing implementation

#### Scenario: Flag parsing

- **WHEN** user provides flags like `--module` or `--wave`
- **THEN** the system SHALL parse flags using clap's derive macros
- **AND** SHALL support both short (`-m`) and long (`--module`) forms where applicable

#### Scenario: Unknown command error

- **WHEN** user executes `ito <unknown-command>`
- **THEN** the system SHALL display a helpful error message via clap's error formatting
- **AND** SHALL suggest similar valid commands if available

### Requirement: Help text generation

The CLI SHALL generate help text automatically from doc comments and type information, eliminating manual help string constants.

#### Scenario: Display command help

- **WHEN** user executes `ito --help` or `ito -h`
- **THEN** the system SHALL display auto-generated help text
- **AND** the help text SHALL list all available commands with descriptions
- **AND** the help text SHALL be derived from doc comments on the CLI struct

#### Scenario: Display subcommand help

- **WHEN** user executes `ito tasks --help`
- **THEN** the system SHALL display auto-generated help text for the tasks command
- **AND** the help text SHALL list all available subcommands with descriptions

#### Scenario: Display specific command help

- **WHEN** user executes `ito tasks start --help`
- **THEN** the system SHALL display auto-generated help text for the start subcommand
- **AND** the help text SHALL describe all arguments and flags

### Requirement: Version information

The CLI SHALL display version information derived from Cargo.toml.

#### Scenario: Display version

- **WHEN** user executes `ito --version` or `ito -V`
- **THEN** the system SHALL display the version from Cargo.toml
- **AND** the version SHALL be auto-populated by clap's `#[command(version)]` attribute

### Requirement: Type-safe argument handling

The CLI SHALL use type-safe argument handling via clap's value parsing.

#### Scenario: Parse constrained enum values

- **WHEN** a command accepts an enum argument (e.g., output format)
- **THEN** the system SHALL use clap's `ValueEnum` derive for type-safe parsing
- **AND** SHALL automatically validate against allowed values
- **AND** SHALL display allowed values in help and error messages

#### Scenario: Parse custom types

- **WHEN** a command accepts a domain-specific type (e.g., change-id)
- **THEN** the system SHALL use a custom value parser
- **AND** SHALL provide clear error messages for invalid values

### Requirement: Styled help output

The CLI SHALL display styled, colored help output for improved readability.

#### Scenario: Colored help text

- **WHEN** user executes `ito --help` in a terminal supporting colors
- **THEN** the system SHALL display colored help text
- **AND** headers, commands, and flags SHALL be visually distinguished

#### Scenario: Plain text fallback

- **WHEN** user executes `ito --help` with `NO_COLOR` environment variable set
- **OR** stdout is not a terminal
- **THEN** the system SHALL display plain text help without color codes
