## ADDED Requirements

### Requirement: Automatic installation during ito init

The ito.md slash command MUST be automatically installed in the agent harness when ito init is run. The installation SHALL place the command file in the correct location for the harness to recognize it.

#### Scenario: Slash command installed during init

- **WHEN** user runs 'ito init'
- **THEN** ito installs ito.md slash command to `.opencode/command/ito.md`
- **AND** command file is created with proper format
- **AND** agent harness recognizes the command
- **AND** user can invoke '/ito <command>' syntax

#### Scenario: Command file creation

- **WHEN** ito init creates the slash command
- **THEN** file path is `.opencode/command/ito.md`
- **AND** file contains slash command metadata and invocation logic
- **AND** file has correct permissions for agent harness to read

### Requirement: Slash command syntax and parsing

The ito.md slash command SHALL parse commands in the format '/ito <command> \[args...\]' and invoke the ito skill with the extracted command and arguments.

#### Scenario: Simple command parsing

- **WHEN** user types '/ito dashboard'
- **THEN** slash command extracts command as 'view'
- **AND** invokes ito skill with arguments \['view'\]
- **AND** ito skill handles routing

#### Scenario: Command with arguments parsing

- **WHEN** user types '/ito archive 123-45 --json'
- **THEN** slash command extracts command as 'archive'
- **AND** extracts arguments as \['123-45', '--json'\]
- **AND** invokes ito skill with arguments \['archive', '123-45', '--json'\]

#### Scenario: No arguments provided

- **WHEN** user types '/ito'
- **THEN** slash command detects missing command
- **AND** outputs usage information
- **AND** does not invoke ito skill

### Requirement: Output formatting

The ito.md slash command SHALL display output from the ito skill in a formatted manner suitable for the agent harness interface. The output MUST preserve markdown formatting and code blocks.

#### Scenario: Successful command output

- **WHEN** ito skill returns successful output
- **THEN** slash command displays output in harness
- **AND** markdown formatting is preserved
- **AND** code blocks are properly rendered
- **AND** response is clearly identified as ito output

#### Scenario: Error output formatting

- **WHEN** ito skill returns error output
- **THEN** slash command displays error in harness
- **AND** error is clearly distinguished from success output
- **AND** error details are preserved for debugging

### Requirement: Integration with agent harness

The ito.md slash command SHALL integrate seamlessly with agent harnesses (e.g., opencode) by following the harness's slash command format and conventions.

#### Scenario: Harness discovers slash command

- **WHEN** agent harness loads available commands
- **THEN** harness discovers ito.md slash command
- **AND** command is available via '/ito' syntax
- **AND** command appears in command list or help

#### Scenario: Harness invokes slash command

- **WHEN** user types '/ito dashboard change-123'
- **THEN** harness routes to ito.md slash command
- **AND** slash command invokes ito skill
- **AND** output is returned to harness for display

### Requirement: Manual installation support

The ito.md slash command MUST support manual installation via 'ito install ito' command for cases where automatic installation failed or needs to be reinstalled.

#### Scenario: Manual install command

- **WHEN** user runs 'ito install ito'
- **THEN** command installs ito.md to `.opencode/command/ito.md`
- **AND** reports successful installation
- **AND** slash command is immediately available

#### Scenario: Reinstall command

- **WHEN** ito.md slash command already exists
- **AND** user runs 'ito install ito'
- **THEN** command overwrites existing ito.md
- **AND** reports successful reinstallation
- **AND** latest version is installed

### Requirement: Command help and usage

The ito.md slash command SHALL provide help information when invoked with '--help' or '-help' flag. The help SHALL display available commands and usage examples.

#### Scenario: Help command

- **WHEN** user types '/ito --help'
- **THEN** slash command displays usage information
- **AND** shows command syntax
- **AND** lists common ito commands with brief descriptions
- **AND** provides examples of usage

#### Scenario: Unknown command help

- **WHEN** user types '/ito unknown-command'
- **AND** ito skill reports invalid command
- **THEN** output includes suggestion to use '--help'
- **AND** user is guided to available commands
