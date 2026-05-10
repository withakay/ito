<!-- ITO:START -->
## ADDED Requirements

### Requirement: Rule worktrees/no-write-on-control rejects staged commits in the control checkout

When `worktrees.enabled = true` and the engine runs in staged mode, the system SHALL emit an `ERROR` issue if the current checkout is the configured control / default-branch worktree and the staged-files snapshot contains any entry. The rule SHALL reuse the branch and worktree detection helpers from `ito-core::worktree_validate` rather than re-implementing them.

- **Requirement ID**: validate-repo-worktrees-rules:no-write-on-control

#### Scenario: Staged files in a change worktree pass

- **GIVEN** `worktrees.enabled = true`
- **AND** the current checkout is a change worktree distinct from the configured `default_branch`
- **AND** the staged-files snapshot includes one or more entries
- **WHEN** rule `worktrees/no-write-on-control` runs
- **THEN** it SHALL emit no issues

#### Scenario: Staged files in the control checkout fail

- **GIVEN** `worktrees.enabled = true`
- **AND** the current checkout is the configured `default_branch` (the main / control checkout)
- **AND** the staged-files snapshot includes one or more entries
- **WHEN** rule `worktrees/no-write-on-control` runs
- **THEN** it SHALL emit an `ERROR` issue
- **AND** the issue's `fix` metadata SHALL instruct the user to switch to a change worktree before committing

#### Scenario: No staged entries skips the rule even on control

- **GIVEN** `worktrees.enabled = true`
- **AND** the current checkout is the control checkout
- **AND** the staged-files snapshot is empty
- **WHEN** rule `worktrees/no-write-on-control` runs
- **THEN** it SHALL emit no issues

#### Scenario: Worktrees disabled skips the rule

- **GIVEN** `worktrees.enabled = false`
- **WHEN** the engine filters rules
- **THEN** rule `worktrees/no-write-on-control` SHALL be reported as skipped regardless of staged context

### Requirement: Rule worktrees/layout-consistent enforces minimal layout invariants

When `worktrees.enabled = true`, the system SHALL emit issues describing layout drift relative to the resolved configuration: a `WARNING` if `worktrees.layout.dir_name` is empty, and a `WARNING` when `worktrees.strategy = "checkout_subdir"` and `worktrees.layout.dir_name` is not listed in `.gitignore`.

- **Requirement ID**: validate-repo-worktrees-rules:layout-consistent

#### Scenario: Empty dir_name fails

- **GIVEN** `worktrees.enabled = true`
- **AND** `worktrees.layout.dir_name` is the empty string
- **WHEN** rule `worktrees/layout-consistent` runs
- **THEN** it SHALL emit a `WARNING` issue

#### Scenario: checkout_subdir without gitignore entry fails

- **GIVEN** `worktrees.enabled = true`
- **AND** `worktrees.strategy = "checkout_subdir"`
- **AND** `worktrees.layout.dir_name = "ito-worktrees"`
- **AND** `.gitignore` does not contain `ito-worktrees/`
- **WHEN** rule `worktrees/layout-consistent` runs
- **THEN** it SHALL emit a `WARNING` issue with a `fix` recommending the gitignore entry

#### Scenario: bare_control_siblings strategy does not require gitignore entry

- **GIVEN** `worktrees.enabled = true`
- **AND** `worktrees.strategy = "bare_control_siblings"`
- **WHEN** rule `worktrees/layout-consistent` runs
- **THEN** it SHALL NOT emit a gitignore-related issue
<!-- ITO:END -->
