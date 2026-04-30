<!-- ITO:START -->
## ADDED Requirements

### Requirement: Missing coordination wiring recovery

When `ito create change` runs in coordination-worktree mode and the current worktree is missing required `.ito/*` coordination links, the system SHALL either repair the wiring before continuing or fail with a targeted recovery error that names the missing or invalid path and explains the next step.

- **Requirement ID**: `change-creation:missing-coordination-wiring-recovery`

#### Scenario: Missing links are detected before module lookup

- **GIVEN** coordination storage mode is `worktree`
- **AND** the current worktree is missing `.ito/modules` or another required coordination link
- **WHEN** `ito create change <name> --module <id>` runs
- **THEN** the command does not proceed into generic module-allocation or filesystem errors first
- **AND** it detects the wiring problem as the primary failure mode

#### Scenario: Actionable recovery error replaces generic IO failure

- **GIVEN** coordination storage mode is `worktree`
- **AND** `ito create change` cannot repair the missing or invalid coordination wiring automatically
- **WHEN** the command exits
- **THEN** the error message names the affected path or paths
- **AND** it explains how to repair the worktree before retrying
- **AND** it does not surface only a generic `No such file or directory (os error 2)` message
<!-- ITO:END -->
