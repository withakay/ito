<!-- ITO:START -->
## ADDED Requirements

### Requirement: Rule audit/mirror-branch-set enforces a non-empty mirror branch

When `audit.mirror.enabled = true`, the system SHALL emit a `WARNING` issue if `audit.mirror.branch` is empty or absent. The rule SHALL emit an additional `WARNING` issue when the configured branch does not start with `ito/internal/` to keep audit mirrors inside the workspace's internal namespace.

- **Requirement ID**: validate-repo-audit-rules:mirror-branch-set

#### Scenario: Empty mirror branch fails

- **GIVEN** `audit.mirror.enabled = true`
- **AND** `audit.mirror.branch` is the empty string
- **WHEN** rule `audit/mirror-branch-set` runs
- **THEN** it SHALL emit a `WARNING` issue identifying `audit.mirror.branch` as the affected config key

#### Scenario: Non-conventional name emits an additional warning

- **GIVEN** `audit.mirror.enabled = true`
- **AND** `audit.mirror.branch` is `mirror/audit`
- **WHEN** rule `audit/mirror-branch-set` runs
- **THEN** it SHALL emit a `WARNING` noting the convention is `ito/internal/*`

#### Scenario: Disabled mirror skips the rule

- **GIVEN** `audit.mirror.enabled = false`
- **WHEN** the engine filters rules
- **THEN** rule `audit/mirror-branch-set` SHALL be reported as skipped

### Requirement: Rule audit/mirror-branch-distinct-from-coordination prevents single-branch reuse

When `audit.mirror.enabled = true` AND `changes.coordination_branch.storage = "worktree"`, the system SHALL emit an `ERROR` issue if `audit.mirror.branch` and `changes.coordination_branch.name` are equal. A single branch must not be re-used for both audit mirroring and coordination data.

- **Requirement ID**: validate-repo-audit-rules:mirror-branch-distinct-from-coordination

#### Scenario: Same branch fails

- **GIVEN** `audit.mirror.enabled = true`
- **AND** `changes.coordination_branch.storage = "worktree"`
- **AND** `audit.mirror.branch = changes.coordination_branch.name = "ito/internal/changes"`
- **WHEN** rule `audit/mirror-branch-distinct-from-coordination` runs
- **THEN** it SHALL emit an `ERROR` issue
- **AND** the issue's `fix` metadata SHALL recommend distinct branch names (for example `ito/internal/audit` for the mirror)

#### Scenario: Distinct branches pass

- **GIVEN** `audit.mirror.enabled = true`
- **AND** `audit.mirror.branch = "ito/internal/audit"`
- **AND** `changes.coordination_branch.name = "ito/internal/changes"`
- **WHEN** rule `audit/mirror-branch-distinct-from-coordination` runs
- **THEN** it SHALL emit no issues

#### Scenario: Embedded coordination skips the rule

- **GIVEN** `audit.mirror.enabled = true`
- **AND** `changes.coordination_branch.storage = "embedded"`
- **WHEN** the engine filters rules
- **THEN** rule `audit/mirror-branch-distinct-from-coordination` SHALL be reported as skipped
<!-- ITO:END -->
