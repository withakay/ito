<!-- ITO:START -->
## ADDED Requirements

### Requirement: Coordination worktree sync command

The CLI SHALL provide `ito sync` to validate worktree-backed coordination state, fetch the coordination branch, auto-commit pending coordination-worktree artifact changes, and push the coordination branch when the local setup is valid. The command SHALL support `--force` to bypass redundant-push suppression without bypassing local validation.

- **Requirement ID**: `cli-sync:coordination-worktree-sync`

#### Scenario: Successful sync from a valid coordination worktree

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/{changes,specs,modules,workflows,audit}` resolve to the expected coordination worktree targets
- **AND** the coordination worktree contains pending tracked Ito artifact changes
- **WHEN** the user runs `ito sync`
- **THEN** the system validates the wiring before any push happens
- **AND** fetches the coordination branch from the configured remote
- **AND** creates a coordination-worktree commit for the pending artifact changes
- **AND** pushes the coordination branch to the remote

#### Scenario: Invalid local wiring blocks remote sync

- **GIVEN** coordination storage mode is `worktree`
- **AND** at least one `.ito/` coordination entry is missing, duplicated as a real directory, or points to the wrong target
- **WHEN** the user runs `ito sync`
- **THEN** the system aborts the remote sync
- **AND** reports which paths are invalid and what the expected coordination worktree targets are

#### Scenario: Remote divergence requires user intervention

- **GIVEN** coordination storage mode is `worktree`
- **AND** the coordination branch on the remote cannot be synchronized with the local coordination worktree as a non-interactive fast-forward push
- **WHEN** the user runs `ito sync`
- **THEN** the system does not push conflicting history
- **AND** reports that the coordination branch diverged
- **AND** provides an actionable next step instead of attempting an interactive merge

#### Scenario: Force bypasses redundant-push suppression

- **GIVEN** coordination storage mode is `worktree`
- **AND** the local coordination setup is valid
- **AND** a prior `ito sync` completed recently enough that a normal invocation would skip the push
- **WHEN** the user runs `ito sync --force`
- **THEN** the system still performs local coordination validation
- **AND** does not suppress the remote push only because of the quiet window

### Requirement: Quiet rate-limited sync

The CLI SHALL record the last successful coordination sync timestamp and synchronized worktree state in repo-local metadata, use a configurable sync interval that defaults to 120 seconds, and suppress redundant remote sync attempts while still running lightweight local coordination validation on every invocation.

- **Requirement ID**: `cli-sync:quiet-rate-limited-sync`

#### Scenario: Repeated sync within the quiet window skips redundant push

- **GIVEN** no explicit coordination sync interval is configured
- **AND** a prior `ito sync` completed successfully less than 120 seconds ago
- **AND** the local coordination worktree state has not changed since that sync
- **WHEN** the user or a skill runs `ito sync` again
- **THEN** the system still validates local coordination wiring
- **AND** skips the redundant remote push
- **AND** emits no extra success chatter beyond what is needed to explain an actual problem

#### Scenario: New local coordination change bypasses redundant-push suppression

- **GIVEN** a prior `ito sync` completed successfully less than the configured sync interval ago
- **AND** the coordination worktree state has changed since that sync
- **WHEN** the user or a skill runs `ito sync`
- **THEN** the system does not treat the invocation as redundant
- **AND** performs the required push after validation succeeds

#### Scenario: Configured sync interval overrides the default quiet window

- **GIVEN** `changes.coordination_branch.sync_interval_seconds` is set to `300`
- **AND** a prior `ito sync` completed successfully 180 seconds ago
- **AND** the local coordination worktree state has not changed since that sync
- **WHEN** the user or a skill runs `ito sync`
- **THEN** the system still validates local coordination wiring
- **AND** skips the redundant remote push because the configured interval has not elapsed

#### Scenario: Embedded coordination storage is a short no-op

- **GIVEN** coordination storage mode is `embedded`
- **WHEN** the user runs `ito sync`
- **THEN** the command succeeds without attempting coordination-worktree validation or push
- **AND** reports briefly that worktree sync is not active for the current project mode
<!-- ITO:END -->
