<!-- ITO:START -->
## ADDED Requirements

### Requirement: OpenCode provides a /loop command to run Ito Ralph

The system SHALL install an OpenCode slash command named `loop` that runs an Ito Ralph loop for a specified target (change id initially).

#### Scenario: Loop command is installed

- **WHEN** `ito init` installs OpenCode commands
- **THEN** `.opencode/commands/loop.md` SHALL exist

#### Scenario: Loop command runs Ralph for a change id

- **GIVEN** a user runs `/loop 002-17_opencode-loop-command`
- **WHEN** the command is executed
- **THEN** the workflow SHALL run `ito ralph --no-interactive --harness opencode --change 002-17_opencode-loop-command`

### Requirement: Loop restarts append restart context

When the wrapper restarts a Ralph run, it SHALL append a restart note into the Ralph context so the next run continues from the last known progress.

The restart note SHOULD follow this structure:

- “You have been restarted …”
- A short bullet list of progress (for example: last iteration, last error/exit, tasks status)
- A single “continue from here” instruction.

#### Scenario: Restart appends context

- **GIVEN** a Ralph run exits non-zero
- **WHEN** the wrapper decides to restart
- **THEN** it SHALL run `ito ralph --no-interactive --change <change-id> --add-context <restart-note>`

### Requirement: Optional model override

The wrapper SHALL allow an explicit model id to be passed through to the OpenCode harness.

#### Scenario: Model is passed through

- **GIVEN** the user supplies a model id
- **WHEN** the wrapper runs Ralph
- **THEN** it SHALL pass `--model <model-id>` to `ito ralph`
<!-- ITO:END -->
