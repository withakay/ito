## ADDED Requirements

### Requirement: CoordinationStorage enum

The `ito-config` crate SHALL provide a `CoordinationStorage` enum with variants `Worktree` and `Embedded`, serialized as `"worktree"` and `"embedded"` respectively.

- **Requirement ID**: ito-config-crate:coordination-storage-enum

#### Scenario: Worktree variant serializes correctly

- **WHEN** `CoordinationStorage::Worktree` is serialized to JSON
- **THEN** the output is `"worktree"`

#### Scenario: Embedded variant serializes correctly

- **WHEN** `CoordinationStorage::Embedded` is serialized to JSON
- **THEN** the output is `"embedded"`

#### Scenario: Default is Worktree

- **WHEN** no storage field is present in config
- **THEN** the deserialized value is `CoordinationStorage::Worktree`

### Requirement: Coordination branch config gains storage fields

`CoordinationBranchConfig` SHALL include a `storage` field of type `CoordinationStorage` and an optional `worktree_path` field for explicit path override.

- **Requirement ID**: ito-config-crate:coordination-branch-storage-fields

#### Scenario: Storage field defaults to Worktree

- **WHEN** config JSON has `changes.coordination_branch` without a `storage` key
- **THEN** `storage` resolves to `CoordinationStorage::Worktree`

#### Scenario: Explicit worktree_path overrides XDG resolution

- **WHEN** config JSON has `changes.coordination_branch.worktree_path` set to `/custom/path`
- **THEN** `worktree_path` resolves to `Some("/custom/path")`

#### Scenario: Missing worktree_path defaults to None

- **WHEN** config JSON has no `worktree_path` key
- **THEN** `worktree_path` resolves to `None`
