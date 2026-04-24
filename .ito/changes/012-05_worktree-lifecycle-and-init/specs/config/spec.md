<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: WorktreesConfig includes init sub-section

The `worktrees` configuration block in `config.json` (and its JSON Schema representation)
SHALL include an `init` sub-section of type `WorktreeInitConfig`. This sub-section
SHALL contain an `include` field holding a list of glob pattern strings and an optional
`setup` field accepting either a single command string or an ordered list of command strings.
All fields default to empty/absent without error.

- **Requirement ID**: `config:worktrees-init-config`

#### Scenario: Default — empty include list and no setup

- **WHEN** `worktrees.init` is absent from `config.json`
- **THEN** the resolved config has an empty `include` list and no setup commands; no files are
  copied and no commands are run during worktree initialization

#### Scenario: Explicit include list

- **WHEN** `config.json` contains `"worktrees": { "init": { "include": [".env", ".envrc"] } }`
- **THEN** the resolved config has `include = [".env", ".envrc"]`

#### Scenario: Single setup command string

- **WHEN** `config.json` contains `"worktrees": { "init": { "setup": "make init" } }`
- **THEN** the resolved config has a single setup command `"make init"`

#### Scenario: Ordered setup command list

- **WHEN** `config.json` contains `"worktrees": { "init": { "setup": ["npm ci", "npm run build:types"] } }`
- **THEN** the resolved config has two setup commands in that order

#### Scenario: JSON Schema validates include as array of strings

- **WHEN** a config file sets `worktrees.init.include` to a non-array value
- **THEN** schema validation rejects it with a clear error

#### Scenario: JSON Schema validates setup as string or array of strings

- **WHEN** a config file sets `worktrees.init.setup` to a non-string, non-array value
- **THEN** schema validation rejects it with a clear error

#### Scenario: Existing worktrees config fields unaffected

- **WHEN** `worktrees.init` is added alongside existing fields (`enabled`, `strategy`, `layout`, `apply`, `default_branch`)
- **THEN** all existing fields retain their previous behavior and defaults
<!-- ITO:END -->
