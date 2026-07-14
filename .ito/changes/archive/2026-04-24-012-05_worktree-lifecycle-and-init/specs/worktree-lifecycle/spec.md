<!-- ITO:START -->
## ADDED Requirements

### Requirement: Worktree existence check before apply

Before an agent begins applying a change, when `worktrees.enabled` is `true` in the Ito
config, the system SHALL verify that a worktree for the change exists at the expected path
derived from the configured strategy and layout.

- **Requirement ID**: `worktree-lifecycle:existence-check`

#### Scenario: Worktree already exists

- **WHEN** `ito worktree ensure --change <id>` is run and the worktree path already exists and is a valid git worktree
- **THEN** the command exits 0 and prints the resolved absolute worktree path to stdout

#### Scenario: Worktree does not exist — created automatically

- **WHEN** `ito worktree ensure --change <id>` is run and no worktree exists for the change
- **THEN** the system creates the worktree from the configured default branch, runs worktree initialization, and prints the resolved absolute worktree path to stdout

#### Scenario: Worktrees disabled

- **WHEN** `ito worktree ensure --change <id>` is run and `worktrees.enabled` is `false`
- **THEN** the command exits 0 and prints the current working directory as the resolved path

### Requirement: Worktree path reporting

The `ito worktree ensure` command SHALL emit the resolved worktree path as the only line on
stdout (no decorative output), so that scripts and agents can capture it with command
substitution.

- **Requirement ID**: `worktree-lifecycle:path-reporting`

#### Scenario: Path printed to stdout

- **WHEN** `ito worktree ensure --change <id>` completes successfully
- **THEN** a single absolute path is written to stdout with a trailing newline and nothing else

#### Scenario: Informational output goes to stderr

- **WHEN** the worktree is being created and progress messages are emitted
- **THEN** those messages go to stderr and do not appear on stdout

### Requirement: Worktree creation uses configured strategy

When creating a worktree, the system SHALL derive the target path from `worktrees.strategy`
and `worktrees.layout` (for strategies that use a layout), branch the worktree from
`worktrees.default_branch`, and name the branch after the change id.

- **Requirement ID**: `worktree-lifecycle:strategy-aware-creation`

#### Scenario: BareControlSiblings strategy

- **WHEN** `worktrees.strategy` is `bare_control_siblings` and the worktree does not exist
- **THEN** the worktree is created as a sibling of the main worktree directory with a branch named after the change id

#### Scenario: CheckoutSiblings strategy

- **WHEN** `worktrees.strategy` is `checkout_siblings` and the worktree does not exist
- **THEN** the worktree is created as a sibling of the current checkout with a branch named after the change id

#### Scenario: CheckoutSubdir strategy

- **WHEN** `worktrees.strategy` is `checkout_subdir` and the worktree does not exist
- **THEN** the worktree is created inside the configured subdirectory under the current checkout

### Requirement: Apply instruction guidance includes worktree ensure step

When `worktrees.enabled` is `true`, the agent instruction artifact for `apply` SHALL include
an explicit step instructing the agent to run `ito worktree ensure --change <id>`, capture
the output path, and perform all subsequent file operations under that path.

- **Requirement ID**: `worktree-lifecycle:apply-instruction-guidance`

#### Scenario: Worktrees enabled — guidance present

- **WHEN** `ito agent instruction apply --change <id>` is generated and `worktrees.enabled` is `true`
- **THEN** the output contains a step to run `ito worktree ensure --change <id>` and a note to use the returned path as the working directory

#### Scenario: Worktrees disabled — guidance absent

- **WHEN** `ito agent instruction apply --change <id>` is generated and `worktrees.enabled` is `false`
- **THEN** no `ito worktree ensure` step is present in the output
<!-- ITO:END -->
