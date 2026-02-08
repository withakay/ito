## MODIFIED Requirements

### Requirement: Worktree workspace defaults

The system SHALL support user-level global configuration for worktree workspace behavior through a nested `worktrees` object.

The `worktrees` object SHALL support:

- `enabled` (boolean): Enables worktree policy features.
- `strategy` (string enum): `bare_control_siblings`, `checkout_subdir`, or `checkout_siblings`.
- `layout.base_dir` (string): Base path used to resolve `main` and change worktree directories for the selected strategy.
- `layout.dir_name` (string): Name of the directory that holds change worktrees. Defaults to `ito-worktrees`. Used by `checkout_subdir` (as `.<dir_name>/` inside the checkout), `checkout_siblings` (as `<project>-<dir_name>/` next to the checkout), and `bare_control_siblings` (as `<dir_name>/` inside the bare repo directory).
- `apply.enabled` (boolean): Enables worktree-specific setup in apply instructions.
- `apply.integration_mode` (string enum): `commit_pr` or `merge_parent`.
- `apply.copy_from_main` (array of glob patterns): Files to copy from `./main` into the change worktree without staging by default.
- `apply.setup_commands` (array of strings): Ordered shell commands to run in the change worktree before implementation starts.
- `default_branch` (string): Branch used when creating/reusing the base worktree.

#### Scenario: Default branch selection

- **WHEN** worktree workspace mode requires a default branch
- **THEN** the system uses `worktrees.default_branch` if present
- **AND** otherwise defaults to `main`
- **AND** falls back to `master` if `main` does not exist

#### Scenario: Default local file copy patterns

- **WHEN** creating a new change worktree
- **THEN** the system uses `worktrees.apply.copy_from_main` patterns to select files copied from `./main`
- **AND** the default list includes `.env`, `.envrc`, and `.mise.local.toml`

#### Scenario: Default layout strategy

- **WHEN** worktree mode is enabled and `worktrees.strategy` is not configured
- **THEN** the system defaults to `checkout_subdir`

#### Scenario: Unsupported strategy is rejected

- **WHEN** `worktrees.strategy` is set to a value outside the supported enum
- **THEN** configuration validation fails with a clear error
- **AND** Ito does not attempt to infer a custom topology

#### Scenario: Layout base directory resolution

- **WHEN** `worktrees.layout.base_dir` is configured
- **THEN** the system resolves worktree paths from that base directory
- **AND** generated instructions show resolved `main` and change worktree paths

#### Scenario: checkout_subdir strategy path resolution

- **WHEN** `worktrees.strategy` is `checkout_subdir`
- **THEN** the main worktree is the checkout directory itself
- **AND** change worktrees are placed under a gitignored `.<dir_name>/` subdirectory inside the checkout, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)

#### Scenario: checkout_siblings strategy path resolution

- **WHEN** `worktrees.strategy` is `checkout_siblings`
- **THEN** the main worktree is the original checkout directory
- **AND** change worktrees are placed under a dedicated `<project>-<dir_name>/` sibling directory next to the checkout, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)

#### Scenario: bare_control_siblings strategy path resolution

- **WHEN** `worktrees.strategy` is `bare_control_siblings`
- **THEN** the main worktree is at `<base>/main`
- **AND** change worktrees are placed under a `<dir_name>/` subfolder inside the bare repo directory, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)

#### Scenario: Default worktree directory name

- **WHEN** `worktrees.layout.dir_name` is not configured
- **THEN** the system defaults to `ito-worktrees`

#### Scenario: Custom worktree directory name

- **WHEN** `worktrees.layout.dir_name` is set to a custom value (e.g., `worktrees`)
- **THEN** the system uses that value in place of `ito-worktrees` when resolving worktree directory paths for all strategies

#### Scenario: Default integration mode

- **WHEN** `worktrees.apply.integration_mode` is not configured
- **THEN** the system uses `commit_pr` as the default integration preference

#### Scenario: Setup commands are optional

- **WHEN** `worktrees.apply.setup_commands` is omitted or empty
- **THEN** no setup commands are emitted or executed

#### Scenario: Legacy camelCase keys are accepted with deprecation warning

- **WHEN** a config file contains the legacy key `worktrees.defaultBranch`
- **THEN** the system reads the value as `worktrees.default_branch`
- **AND** emits a deprecation warning recommending the new key name

#### Scenario: Legacy localFiles key is accepted with deprecation warning

- **WHEN** a config file contains the legacy key `worktrees.localFiles`
- **THEN** the system reads the value as `worktrees.apply.copy_from_main`
- **AND** emits a deprecation warning recommending the new key name

#### Scenario: New keys take precedence over legacy keys

- **WHEN** a config file contains both a legacy key and its new equivalent
- **THEN** the new key value takes precedence
- **AND** the legacy key value is ignored
