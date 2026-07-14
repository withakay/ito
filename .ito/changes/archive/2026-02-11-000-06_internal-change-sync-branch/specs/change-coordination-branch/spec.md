## ADDED Requirements

### Requirement: Coordination branch configuration

The system SHALL support an internal coordination branch mode for change metadata synchronization.

The configuration SHALL include:

- `changes.coordination_branch.enabled` (boolean, default `true`)
- `changes.coordination_branch.name` (string, default `ito/internal/changes`)

#### Scenario: Default coordination branch settings apply

- **WHEN** the user has not configured coordination branch settings
- **THEN** the system uses `changes.coordination_branch.enabled=true`
- **AND** the system uses `changes.coordination_branch.name="ito/internal/changes"`

#### Scenario: Coordination branch can be disabled

- **WHEN** `changes.coordination_branch.enabled=false`
- **THEN** change operations skip coordination-branch sync behavior
- **AND** existing local workflow behavior remains unchanged

### Requirement: Proposal creation reserves change metadata on the coordination branch

When coordination branch mode is enabled, the system SHALL synchronize and update the coordination branch during change proposal creation.

#### Scenario: Create change performs pre-sync and reservation push

- **GIVEN** `changes.coordination_branch.enabled=true`
- **WHEN** the user runs `ito create change <name>`
- **THEN** the system fetches and updates local state for `changes.coordination_branch.name`
- **AND** creates proposal metadata after sync
- **AND** commits and pushes the new metadata to `changes.coordination_branch.name` immediately

#### Scenario: Non-fast-forward push reports deterministic recovery guidance

- **GIVEN** `changes.coordination_branch.enabled=true`
- **AND** a remote update causes the push to be non-fast-forward
- **WHEN** the system attempts reservation push
- **THEN** the operation fails with a clear conflict message
- **AND** the message instructs the user to sync and retry

### Requirement: Apply and task entry points synchronize from coordination branch

When coordination branch mode is enabled, the system SHALL sync coordination metadata before workflow entry points that consume change state.

#### Scenario: Apply instructions synchronize coordination branch

- **GIVEN** `changes.coordination_branch.enabled=true`
- **WHEN** the user runs `ito agent instruction apply --change <id>`
- **THEN** the system fetches and updates local state from `changes.coordination_branch.name` before generating output

#### Scenario: Task start synchronizes coordination branch

- **GIVEN** `changes.coordination_branch.enabled=true`
- **WHEN** the user runs `ito tasks start <change-id> <task-id>`
- **THEN** the system fetches and updates local state from `changes.coordination_branch.name` before mutating task status

### Requirement: Coordination sync is non-intrusive to user workspace

Coordination branch synchronization SHALL not require switching the user's active branch or mutating unrelated working tree files.

#### Scenario: User branch remains unchanged during synchronization

- **WHEN** coordination synchronization executes during create/apply/tasks workflows
- **THEN** the user's active branch remains unchanged
- **AND** no unrelated tracked files in the current working tree are modified by synchronization
