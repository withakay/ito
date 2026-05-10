<!-- ITO:START -->
## ADDED Requirements

### Requirement: ito init emits a repo-validation advisory when at least one rule activates

After `ito init` and `ito init --upgrade` complete their primary work, the system SHALL emit a post-install advisory **only when** the resolved `ItoConfig` would activate at least one rule in the `ito validate repo` engine. The advisory SHALL be skipped on a fully-set-up project where no rule activates and a pre-commit hook for `ito validate repo` is already present.

- **Requirement ID**: ito-init:repo-validation-advisory

#### Scenario: Coordination-worktree project sees the advisory

- **GIVEN** `changes.coordination_branch.storage = "worktree"`
- **AND** no pre-commit hook for `ito validate repo` is configured
- **WHEN** `ito init` finishes
- **THEN** the command SHALL print an advisory mentioning `ito validate repo`
- **AND** the advisory SHALL recommend running the `ito-update-repo` skill in the user's harness to finish setup

#### Scenario: Embedded-storage project with no worktrees skips the advisory

- **GIVEN** `changes.coordination_branch.storage = "embedded"`
- **AND** `worktrees.enabled = false`
- **AND** `audit.mirror.enabled = false`
- **AND** `repository.mode = "filesystem"`
- **AND** `backend.enabled = false`
- **WHEN** `ito init` finishes
- **THEN** the command SHALL NOT print the repo-validation advisory

#### Scenario: --upgrade also surfaces the advisory

- **GIVEN** an already-initialized project with `worktrees.enabled = true`
- **WHEN** the user runs `ito init --upgrade`
- **THEN** the command SHALL emit the advisory once after the upgrade completes

### Requirement: Advisory names the detected pre-commit system

The advisory SHALL invoke `detect_pre_commit_system(project_root)` and include the detected system in its message so the user knows what the agent will configure on their behalf.

- **Requirement ID**: ito-init:advisory-detected-system

#### Scenario: Advisory states the detected prek system

- **GIVEN** the repo root contains `.pre-commit-config.yaml` and a prek toolchain marker
- **WHEN** `ito init` emits the advisory
- **THEN** the message SHALL state that the detected pre-commit system is `prek`

#### Scenario: Advisory states None when no system is detected

- **GIVEN** the repo has no supported pre-commit framework markers
- **WHEN** `ito init` emits the advisory
- **THEN** the message SHALL state that no pre-commit system was detected
- **AND** the message SHALL list the supported systems so the user can choose one

### Requirement: Advisory references the ito-update-repo skill rather than a new slash command

The advisory SHALL direct the user to invoke the existing `ito-update-repo` skill / slash command in their harness; it SHALL NOT introduce a new harness command for pre-commit setup.

- **Requirement ID**: ito-init:advisory-references-update-repo

#### Scenario: Advisory text mentions ito-update-repo

- **WHEN** `ito init` emits the advisory
- **THEN** the printed message SHALL contain the literal string `ito-update-repo` (or the configured slash-command alias)
<!-- ITO:END -->
