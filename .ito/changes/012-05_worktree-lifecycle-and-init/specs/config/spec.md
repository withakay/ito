<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: WorktreesConfig includes init sub-section

The `worktrees` configuration block in `config.json` (and its JSON Schema representation)
SHALL include an `init` sub-section of type `WorktreeInitConfig`. This sub-section
SHALL contain an `include` field holding a list of glob pattern strings. Both the sub-section
and the `include` field SHALL default to empty/absent without error.

- **Requirement ID**: `config:worktrees-init-config`

#### Scenario: Default — empty include list

- **WHEN** `worktrees.init` is absent from `config.json`
- **THEN** the resolved config has an empty `include` list and no files are copied during worktree initialization

#### Scenario: Explicit include list

- **WHEN** `config.json` contains `"worktrees": { "init": { "include": [".env", ".envrc"] } }`
- **THEN** the resolved config has `include = [".env", ".envrc"]`

#### Scenario: JSON Schema validates include as array of strings

- **WHEN** a config file sets `worktrees.init.include` to a non-array value
- **THEN** schema validation rejects it with a clear error

#### Scenario: Existing worktrees config fields unaffected

- **WHEN** `worktrees.init` is added alongside existing fields (`enabled`, `strategy`, `layout`, `apply`, `default_branch`)
- **THEN** all existing fields retain their previous behavior and defaults
<!-- ITO:END -->
