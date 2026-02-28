## REMOVED Requirements

### Requirement: Project state management

**Reason**: `ito state` is underused and increases maintenance burden due to command/snapshot/spec drift.

**Migration**: Use `ito plan init` to create `.ito/planning/STATE.md` when needed, then edit `STATE.md` directly in the repository. Any automation using `ito state ...` must be removed or replaced with direct file edits.

#### Scenario: Existing `ito state` automation migrates to direct file edits

- **GIVEN** a workflow previously invoked `ito state note <text>`
- **WHEN** the command is removed
- **THEN** the workflow updates `STATE.md` directly instead of invoking `ito state`

## ADDED Requirements

### Requirement: State command is not exposed

The CLI SHALL NOT expose a top-level `state` command.

#### Scenario: Help output excludes state command

- **WHEN** executing `ito --help`
- **THEN** the command list SHALL NOT include `state`

#### Scenario: State command invocation is rejected

- **WHEN** executing `ito state`
- **THEN** the command SHALL fail with a non-zero exit code
