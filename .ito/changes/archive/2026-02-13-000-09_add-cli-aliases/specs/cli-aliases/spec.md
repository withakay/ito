# CLI Aliases Specification

<!-- ITO:START -->

## ADDED Requirements

### Requirement: Main commands have 2-letter aliases
All main CLI commands SHALL have 2-letter visible aliases for faster typing.

#### Scenario: Using ls alias for list command
- **WHEN** user runs `ito ls`
- **THEN** the command executes as `ito list`

#### Scenario: Using ts alias for tasks command
- **WHEN** user runs `ito ts`
- **THEN** the command executes as `ito tasks`

#### Scenario: Using ag alias for agent command
- **WHEN** user runs `ito ag`
- **THEN** the command executes as `ito agent`

#### Scenario: Using cr alias for create command
- **WHEN** user runs `ito cr`
- **THEN** the command executes as `ito create`

### Requirement: Subcommands have 2-letter aliases
All subcommands SHALL have 2-letter visible aliases consistent with their parent command.

#### Scenario: Using ch alias for create change subcommand
- **WHEN** user runs `ito cr ch`
- **THEN** the command executes as `ito create change`

#### Scenario: Using go alias for tasks start subcommand
- **WHEN** user runs `ito ts go <change> <task>`
- **THEN** the command executes as `ito tasks start <change> <task>`

#### Scenario: Using co alias for tasks complete subcommand
- **WHEN** user runs `ito ts co <change> <task>`
- **THEN** the command executes as `ito tasks complete <change> <task>`

#### Scenario: Using in alias for agent instruction subcommand
- **WHEN** user runs `ito ag in <artifact>`
- **THEN** the command executes as `ito agent instruction <artifact>`

#### Scenario: Using de alias for state decision subcommand
- **WHEN** user runs `ito sa de <text>`
- **THEN** the command executes as `ito state decision <text>`

### Requirement: Short flag -c maps to --change
The `-c` short flag SHALL be an alias for `--change` on all commands that accept a change parameter.

#### Scenario: Using -c with tasks start
- **WHEN** user runs `ito ts go -c 005-01 1.1`
- **THEN** the command executes as `ito tasks start --change 005-01 1.1`

#### Scenario: Using -c with agent instruction
- **WHEN** user runs `ito ag in pr -c 005-01`
- **THEN** the command executes as `ito agent instruction proposal --change 005-01`

#### Scenario: Using -c with show command
- **WHEN** user runs `ito sh -c 005-01`
- **THEN** the command executes as `ito show --change 005-01`

### Requirement: Short flag -m maps to --module
The `-m` short flag SHALL be an alias for `--module` on all commands that accept a module parameter.

#### Scenario: Using -m with create change
- **WHEN** user runs `ito cr ch -m 005 my-feature`
- **THEN** the command executes as `ito create change --module 005 my-feature`

#### Scenario: Using -m with list command
- **WHEN** user runs `ito ls -m 005`
- **THEN** the command executes as `ito list --module 005`

#### Scenario: Using -m with ralph command
- **WHEN** user runs `ito ra -m 005`
- **THEN** the command executes as `ito ralph --module 005`

### Requirement: Aliases appear in help output
All visible aliases SHALL appear in the --help output so users can discover them.

#### Scenario: List command shows ls alias
- **WHEN** user runs `ito --help`
- **THEN** the output shows `ls` as an alias for the `list` command

#### Scenario: Tasks subcommand aliases shown
- **WHEN** user runs `ito tasks --help`
- **THEN** the output shows `go` as an alias for the `start` subcommand

<!-- ITO:END -->
