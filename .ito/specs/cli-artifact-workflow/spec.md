## MODIFIED Requirements

### Requirement: Worktree-aware apply instructions

When worktree apply mode is enabled by configuration, `ito instructions apply` SHALL include deterministic instructions that resolve the configured layout strategy, create (or reuse) a worktree for the change branch, prepare local files, and tell the agent which directory to work in.

#### Scenario: Apply instructions include worktree script
- **GIVEN** `worktrees.enabled=true` and `worktrees.apply.enabled=true`
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include a copy/pasteable shell snippet that:
  - Ensures the configured `main` worktree path exists and is on the default branch
  - Creates or reuses a change worktree directory at a stable path derived from `worktrees.strategy` and `worktrees.layout.base_dir`
  - Prints the expected working directory for subsequent commands

#### Scenario: Strategy-specific path conventions are deterministic
- **GIVEN** worktree apply mode is enabled
- **WHEN** `worktrees.strategy` is `bare_control_siblings`
- **THEN** instructions resolve `main` as a worktree at `<base>/main`
- **AND** instructions resolve change worktrees under `<base>/<dir_name>/<change-id>`, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)
- **AND** the change worktree path is deterministic for the change ID

#### Scenario: Checkout-subdir strategy path conventions
- **GIVEN** worktree apply mode is enabled
- **WHEN** `worktrees.strategy` is `checkout_subdir`
- **THEN** instructions resolve change worktrees under a gitignored `.<dir_name>/` subdirectory in the checkout, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)

#### Scenario: Checkout-siblings strategy path conventions
- **GIVEN** worktree apply mode is enabled
- **WHEN** `worktrees.strategy` is `checkout_siblings`
- **THEN** instructions resolve change worktrees under a dedicated `<project>-<dir_name>/` sibling directory next to the checkout, where `<dir_name>` is `worktrees.layout.dir_name` (default `ito-worktrees`)

#### Scenario: Apply instructions ask for layout preference when missing
- **GIVEN** worktree apply mode is enabled
- **AND** `worktrees.strategy` is not configured
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include an explicit ask-user step to confirm preferred strategy before creating new worktrees
- **AND** the instructions include only supported strategy options with a recommended default

#### Scenario: Apply instructions do not offer custom strategy
- **GIVEN** worktree apply mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** instructions present only codified strategy options
- **AND** instructions do not suggest unsupported custom topology modes

#### Scenario: Apply instructions include local file copy step
- **GIVEN** worktree apply mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include a step to copy files matched by `worktrees.apply.copy_from_main` from `./main` into the change worktree
- **AND** missing files are treated as non-fatal (copy what exists)
- **AND** copied files are identified as local/uncommitted setup files

#### Scenario: Apply instructions include setup commands
- **GIVEN** worktree apply mode is enabled
- **AND** `worktrees.apply.setup_commands` is non-empty
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include the configured commands in order, scoped to the change worktree

#### Scenario: Apply instructions include integration guidance
- **GIVEN** worktree apply mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** instructions include integration guidance based on `worktrees.apply.integration_mode`
- **AND** `commit_pr` guidance includes commit and PR creation steps
- **AND** `merge_parent` guidance includes merge-into-parent steps

#### Scenario: Apply instructions include cleanup guidance
- **GIVEN** worktree apply mode is enabled
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** the instructions include post-merge cleanup steps for the change worktree and associated local branch

#### Scenario: Worktree instructions are skipped when disabled
- **GIVEN** `worktrees.enabled=false` or `worktrees.apply.enabled=false`
- **WHEN** the user runs `ito instructions apply --change <id>`
- **THEN** worktree-specific setup and cleanup instructions are not injected
