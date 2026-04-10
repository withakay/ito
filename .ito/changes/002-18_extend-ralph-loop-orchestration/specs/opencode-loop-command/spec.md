<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: OpenCode provides a /loop command to run Ito Ralph

The system SHALL install an OpenCode slash command at `.opencode/commands/ito-loop.md` that users invoke as `/ito-loop` to run Ito Ralph for a supported target mode.

- **Requirement ID**: opencode-loop-command:ito-loop-command

#### Scenario: Loop command is installed

- **WHEN** `ito init` installs OpenCode commands
- **THEN** `.opencode/commands/ito-loop.md` SHALL exist

#### Scenario: Loop command runs Ralph for a change id

- **GIVEN** a user runs `/ito-loop 002-17_opencode-loop-command`
- **WHEN** the command is executed
- **THEN** the workflow SHALL run `ito ralph --no-interactive --harness opencode --change 002-17_opencode-loop-command`

#### Scenario: Loop command runs Ralph for a module id

- **GIVEN** a user runs `/ito-loop 002`
- **WHEN** the command is executed
- **THEN** the workflow SHALL run `ito ralph --no-interactive --harness opencode --module 002`

#### Scenario: Loop command defaults to continue-ready

- **GIVEN** a user runs `/ito-loop` with no explicit target
- **WHEN** the command is executed
- **THEN** the workflow SHALL run `ito ralph --no-interactive --harness opencode --continue-ready`

### Requirement: Loop restarts append restart context

When the wrapper supervises a bounded restart of a Ralph run, it SHALL append a restart note into the Ralph context so the next run continues from the last known progress.

- **Requirement ID**: opencode-loop-command:restart-context

The restart note SHOULD follow this structure:

- “You have been restarted …”
- A short bullet list of progress (for example: last iteration, last error/exit, tasks status)
- A single “continue from here” instruction.

#### Scenario: Restart appends context

- **GIVEN** a Ralph run exits early in a way the wrapper treats as restartable
- **WHEN** the wrapper decides to restart
- **THEN** it SHALL run `ito ralph --no-interactive --change <change-id> --add-context <restart-note>` before or as part of the rerun flow

#### Scenario: Successful run is not wrapped in an outer infinite loop

- **GIVEN** the wrapper launches `ito ralph` for a target
- **WHEN** Ralph completes successfully or exhausts its own iterations without a restartable early exit
- **THEN** the wrapper SHALL stop supervising that run instead of wrapping it in another external infinite loop

### Requirement: Optional model override

The wrapper SHALL allow an explicit model id to be passed through to the OpenCode harness.

- **Requirement ID**: opencode-loop-command:model-override

#### Scenario: Model is passed through

- **GIVEN** the user supplies a model id
- **WHEN** the wrapper runs Ralph
- **THEN** it SHALL pass `--model <model-id>` to `ito ralph`
<!-- ITO:END -->
