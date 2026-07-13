<!-- ITO:START -->
# Validate Repo Coordination Rules

## Purpose

This spec defines the current behavior and requirements for validate repo coordination rules.

## Requirements

### Requirement: Rule coordination/symlinks-wired enforces worktree symlink layout
While legacy coordination-worktree storage remains enabled, the system SHALL evaluate each configured coordination directory and report an `ERROR` when its symlink does not resolve to the corresponding legacy coordination-worktree path. Every remediation message SHALL name a direct CLI or emitted instruction and MUST NOT recommend `ito-update-repo`.

#### Scenario: Healthy legacy symlinks pass
- **GIVEN** legacy coordination-worktree storage is enabled
- **AND** every configured coordination directory resolves to its expected target
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it emits no issues

#### Scenario: Real directory receives migration remediation
- **GIVEN** legacy coordination-worktree storage is enabled
- **AND** a configured coordination path is a real directory instead of its expected symlink
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it emits an `ERROR` identifying the path and expected target
- **AND** the remediation names the direct migration instruction or supported synchronization CLI
- **AND** it does not mention `ito-update-repo`

#### Scenario: Legacy coordination is disabled
- **GIVEN** coordination-worktree storage is feature-disabled or not configured
- **WHEN** repository rules are filtered
- **THEN** `coordination/symlinks-wired` is skipped

### Requirement: Rule coordination/gitignore-entries enforces canonical .gitignore lines

When `changes.coordination_branch.storage = "worktree"`, the system SHALL emit a `WARNING` issue for each canonical `.ito/<dir>` entry that is missing from `.gitignore`. The canonical list SHALL come from the same source the auto-wiring helper uses, exposed as a pure function.

- **Requirement ID**: validate-repo-coordination-rules:gitignore-entries

#### Scenario: All canonical entries present passes

- **GIVEN** coordination storage is `worktree`
- **AND** `.gitignore` contains `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit`
- **WHEN** rule `coordination/gitignore-entries` runs
- **THEN** it SHALL emit no issues

#### Scenario: Missing entry triggers a warning per entry

- **GIVEN** coordination storage is `worktree`
- **AND** `.gitignore` lacks `.ito/audit`
- **WHEN** rule `coordination/gitignore-entries` runs
- **THEN** it SHALL emit one `WARNING` issue for `.ito/audit`
- **AND** the issue's `fix` metadata SHALL provide the line to append

### Requirement: Rule coordination/staged-symlinked-paths blocks staged commits under coordination dirs

When `changes.coordination_branch.storage = "worktree"` and the engine runs in staged mode, the system SHALL emit an `ERROR` issue for every staged path under `.ito/{changes,specs,modules,workflows,audit}`.

- **Requirement ID**: validate-repo-coordination-rules:staged-symlinked-paths

#### Scenario: No staged coordination paths passes

- **GIVEN** coordination storage is `worktree`
- **AND** the staged-files snapshot contains no entries under any coordination directory
- **WHEN** rule `coordination/staged-symlinked-paths` runs
- **THEN** it SHALL emit no issues

#### Scenario: Staged change under .ito/changes fails

- **GIVEN** coordination storage is `worktree`
- **AND** the staged-files snapshot includes `.ito/changes/011-05_…/proposal.md`
- **WHEN** rule `coordination/staged-symlinked-paths` runs
- **THEN** it SHALL emit an `ERROR` issue identifying the staged path
- **AND** the message SHALL explain that coordination paths belong to the coordination branch, not the working branch

#### Scenario: Rule is inactive without staged context

- **GIVEN** the engine is run without a staged-files snapshot
- **WHEN** the registry is filtered
- **THEN** rule `coordination/staged-symlinked-paths` SHALL be reported as skipped

### Requirement: Rule coordination/branch-name-set enforces a non-empty coordination branch name

The system SHALL emit a `WARNING` issue when `changes.coordination_branch.name` is empty or absent, regardless of storage mode. The rule SHALL emit an additional `WARNING` issue when the configured name does not start with `ito/internal/` to keep coordination branches inside the workspace's internal namespace.

- **Requirement ID**: validate-repo-coordination-rules:branch-name-set

#### Scenario: Empty branch name fails

- **GIVEN** `changes.coordination_branch.name` is the empty string
- **WHEN** rule `coordination/branch-name-set` runs
- **THEN** it SHALL emit a `WARNING` issue

#### Scenario: Non-conventional name emits a warning

- **GIVEN** `changes.coordination_branch.name` is `coordination/foo`
- **WHEN** rule `coordination/branch-name-set` runs
- **THEN** it SHALL emit a `WARNING` issue noting the convention is `ito/internal/*`
<!-- ITO:END -->
