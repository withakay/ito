## ADDED Requirements

### Requirement: Backend coordination commands live under `ito tasks`

When backend mode is enabled, Ito SHALL expose backend coordination commands as `tasks` subcommands instead of new top-level commands.

#### Scenario: Claim and release commands are available under tasks

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks claim <change-id>` or `ito tasks release <change-id>`
- **THEN** Ito executes backend lease claim or release behavior for that change

#### Scenario: Allocation command is available under tasks

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks allocate`
- **THEN** Ito executes backend allocation behavior for next available change

#### Scenario: Sync commands are available under tasks sync

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks sync pull <change-id>` or `ito tasks sync push <change-id>`
- **THEN** Ito executes backend artifact synchronization behavior for that change

### Requirement: Task mutations sync through backend in backend mode

When backend mode is enabled, task mutation operations SHALL synchronize task artifact updates through backend APIs before reporting success.

#### Scenario: Complete task updates backend artifact

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks complete <change-id> <task-id>`
- **THEN** Ito applies the mutation to task content
- **AND** pushes the updated tasks artifact through backend synchronization

#### Scenario: Backend revision conflict prevents silent overwrite

- **GIVEN** backend mode is enabled and local tasks content is stale
- **WHEN** the user runs a task mutation command
- **THEN** Ito reports a synchronization conflict
- **AND** Ito does not silently overwrite newer backend task content
