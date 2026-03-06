## ADDED Requirements

### Requirement: CLI can import local active and archived changes into backend state

When backend mode is enabled, the CLI SHALL provide a command to seed backend change state from local project artifacts.

The command SHALL be `ito backend import`.

#### Scenario: Import includes both active and archived local changes

- **GIVEN** local active changes exist under `.ito/changes/`
- **AND** local archived changes exist under `.ito/changes/archive/`
- **WHEN** the user runs `ito backend import`
- **THEN** Ito uploads active changes as active backend changes
- **AND** uploads archived changes as archived backend changes

#### Scenario: Dry run shows migration plan without backend writes

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito backend import --dry-run`
- **THEN** Ito prints counts of active, archived, and skipped changes
- **AND** no backend mutations are performed

### Requirement: Import is idempotent and resumable

The import command MUST support safe retries so repeated executions do not duplicate or corrupt backend state.

#### Scenario: Re-running import skips unchanged artifacts

- **GIVEN** a previous import already uploaded a change revision
- **WHEN** the user runs `ito backend import` again
- **THEN** Ito skips unchanged artifacts for that change
- **AND** reports the change as already synchronized

#### Scenario: Import resumes after partial failure

- **GIVEN** a prior import attempt failed after importing only a subset of changes
- **WHEN** the user reruns `ito backend import`
- **THEN** Ito resumes by importing remaining unsynchronized changes
- **AND** leaves already imported changes in a consistent state

### Requirement: Import success requires parity validation before local cleanup

The import workflow MUST verify backend parity for imported changes before treating migration as successful and allowing local cleanup.

#### Scenario: Successful import validates parity

- **GIVEN** local changes were uploaded to backend
- **WHEN** import finalization runs
- **THEN** Ito verifies imported change/artifact counts against local source scope
- **AND** reports success only when parity checks pass

#### Scenario: Parity failure blocks cleanup and reports remediation

- **GIVEN** import uploads completed but parity validation fails
- **WHEN** import finalization runs
- **THEN** Ito reports a deterministic validation failure
- **AND** local change artifacts are not removed
