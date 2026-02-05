## MODIFIED Requirements

### Requirement: Automatic installation during ito init

The ito.md slash command MUST be automatically installed in the agent harness when ito init is run. The installation SHALL place the command file in the correct location for the harness to recognize it.

#### Scenario: Slash command installed during init

- **WHEN** user runs 'ito init'
- **THEN** ito installs ito.md slash command to `.opencode/commands/ito.md`
- **AND** command file is created with proper format
- **AND** agent harness recognizes the command
- **AND** user can invoke '/ito <command>' syntax

#### Scenario: Command file creation

- **WHEN** ito init creates the slash command
- **THEN** file path is `.opencode/commands/ito.md`
- **AND** file contains slash command metadata and invocation logic
- **AND** file has correct permissions for agent harness to read
