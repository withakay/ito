<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Worktree creation uses configured strategy

When creating a worktree, the system SHALL use Worktrunk to create or switch to the change branch while preserving Ito's configured worktree path semantics. The system SHALL derive the target worktree root from `worktrees.strategy` and `worktrees.layout` (for strategies that use a layout), run Worktrunk with a local worktree path configuration that maps the change branch to that target path, branch the worktree from `worktrees.default_branch`, and name the branch after the change id.

- **Requirement ID**: `worktree-lifecycle:strategy-aware-creation`

#### Scenario: BareControlSiblings strategy

- **WHEN** `worktrees.strategy` is `bare_control_siblings` and the worktree does not exist
- **THEN** Worktrunk creates the worktree as a sibling of the main worktree directory under the configured `ito-worktrees` root with a branch named after the change id

#### Scenario: CheckoutSiblings strategy

- **WHEN** `worktrees.strategy` is `checkout_siblings` and the worktree does not exist
- **THEN** Worktrunk creates the worktree as a sibling of the current checkout under the configured `ito-worktrees` root with a branch named after the change id

#### Scenario: CheckoutSubdir strategy

- **WHEN** `worktrees.strategy` is `checkout_subdir` and the worktree does not exist
- **THEN** Worktrunk creates the worktree inside the configured subdirectory under the current checkout

#### Scenario: Existing Ito worktree path convention retained

- **WHEN** the default layout directory is `ito-worktrees` and `ito worktree ensure --change 012-08_replace-raw-git-worktrees-with-worktrunk` creates a worktree
- **THEN** the created worktree path matches Ito's configured `ito-worktrees/012-08_replace-raw-git-worktrees-with-worktrunk` layout instead of Worktrunk's global default layout

## ADDED Requirements

### Requirement: Local Worktrunk path configuration

The system SHALL provide or use a local Worktrunk configuration for Ito-managed worktree operations so the effective Worktrunk `worktree-path` template maps the branch name to Ito's configured worktree root and change ID path.

- **Requirement ID**: `worktree-lifecycle:local-worktrunk-path-config`

#### Scenario: Project has no committed Worktrunk config

- **WHEN** `ito worktree ensure --change <id>` runs in a project without `.config/wt.toml`
- **THEN** Ito supplies an operation-local Worktrunk configuration or equivalent environment override that points Worktrunk at the resolved Ito worktree path template

#### Scenario: Project has a committed Worktrunk config

- **WHEN** `.config/wt.toml` exists and contains project Worktrunk settings
- **THEN** Ito preserves those project settings while ensuring the worktree path used for Ito-managed change worktrees remains the resolved Ito worktree path

#### Scenario: User global Worktrunk path differs

- **WHEN** the user's global Worktrunk config sets `worktree-path` to a non-Ito location
- **THEN** `ito worktree ensure --change <id>` still creates or resolves the worktree at Ito's configured `ito-worktrees/<id>` location

### Requirement: Worktrunk command failure diagnostics

When a Worktrunk command used by Ito fails, the system SHALL report what failed, include the command context, preserve Worktrunk's stderr/stdout detail, and provide a concrete remediation.

- **Requirement ID**: `worktree-lifecycle:worktrunk-failure-diagnostics`

#### Scenario: Worktrunk is not installed

- **WHEN** `ito worktree ensure --change <id>` needs to create a worktree and `wt` cannot be executed
- **THEN** the command exits non-zero with an error explaining that Worktrunk is required and how to install or make `wt` available on `PATH`

#### Scenario: Worktrunk rejects worktree creation

- **WHEN** Worktrunk exits non-zero during change worktree creation
- **THEN** the error includes the change id, target path, base branch, and Worktrunk output
<!-- ITO:END -->
