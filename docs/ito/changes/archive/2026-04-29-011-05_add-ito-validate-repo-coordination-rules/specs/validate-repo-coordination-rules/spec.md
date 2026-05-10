<!-- ITO:START -->
## ADDED Requirements

### Requirement: Rule coordination/symlinks-wired enforces worktree symlink layout

When `changes.coordination_branch.storage = "worktree"`, the system SHALL evaluate every directory in `coordination::COORDINATION_DIRS` (`changes`, `specs`, `modules`, `workflows`, `audit`) under `.ito/` and emit an `ERROR` issue for any directory that is not a symlink resolving to the corresponding directory inside the resolved coordination worktree.

- **Requirement ID**: validate-repo-coordination-rules:symlinks-wired

#### Scenario: All symlinks healthy passes

- **GIVEN** coordination storage is `worktree`
- **AND** each coordination directory under `.ito/` is a symlink resolving to the matching directory inside the coordination worktree
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it SHALL emit no issues

#### Scenario: Real directory fails the rule

- **GIVEN** coordination storage is `worktree`
- **AND** `.ito/changes` is a real directory rather than a symlink
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it SHALL emit an `ERROR` issue identifying `.ito/changes`
- **AND** the issue message SHALL state What (real directory found), Why (storage mode requires symlinks), and How (run `ito sync` or `ito-update-repo` to repair)

#### Scenario: Symlink to wrong target fails the rule

- **GIVEN** coordination storage is `worktree`
- **AND** `.ito/specs` is a symlink resolving to a path outside the coordination worktree
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it SHALL emit an `ERROR` issue including both the actual target and the expected target

#### Scenario: Embedded storage skips the rule

- **GIVEN** coordination storage is `embedded`
- **WHEN** the engine filters rules
- **THEN** rule `coordination/symlinks-wired` SHALL be reported as skipped

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
