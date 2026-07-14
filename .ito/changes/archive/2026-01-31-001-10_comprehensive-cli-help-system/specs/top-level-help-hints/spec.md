## MODIFIED Requirements

### Requirement: Top-level help provides option hints

The top-level help output SHALL provide better visibility into available options for each command.

#### Scenario: Commands with options show key options inline

- **WHEN** user runs `ito -h`
- **AND** a command has commonly-used options
- **THEN** the help output MAY show abbreviated option hints (e.g., `list [--json|--specs|--modules]`)
- **OR** the help output SHALL note that options are available

#### Scenario: Help suggests drilling down

- **WHEN** user runs `ito -h`
- **THEN** the output SHALL include a hint like "Run 'ito <command> -h' for command-specific options"

### Requirement: Consistent help footer

Each command's help output SHALL include a consistent footer with navigation hints.

#### Scenario: Help footer for commands with subcommands

- **WHEN** user runs `ito agent -h`
- **THEN** the help output SHALL include "Run 'ito agent <command> -h' for subcommand help"

#### Scenario: Help footer for leaf commands

- **WHEN** user runs `ito list -h`
- **AND** `list` has no subcommands
- **THEN** the help output SHALL NOT include subcommand navigation hint
