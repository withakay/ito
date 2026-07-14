<!-- ITO:START -->
## ADDED Requirements

### Requirement: Rule repository/sqlite-db-path-set enforces a resolvable db_path

When `repository.mode = "sqlite"`, the system SHALL emit an `ERROR` issue if `repository.sqlite.db_path` is empty, absent, or resolves outside the project root. The rule SHALL emit a `WARNING` if the configured path is set and resolvable but its parent directory does not exist and cannot be created (for example because of permissions).

- **Requirement ID**: validate-repo-repository-rules:sqlite-db-path-set

#### Scenario: Missing db_path fails

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path` is unset
- **WHEN** rule `repository/sqlite-db-path-set` runs
- **THEN** it SHALL emit an `ERROR` issue identifying `repository.sqlite.db_path` as the affected config key

#### Scenario: Path outside project root fails

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path = "/var/tmp/ito.db"`
- **WHEN** rule `repository/sqlite-db-path-set` runs
- **THEN** it SHALL emit an `ERROR` issue noting that the path resolves outside the project root

#### Scenario: Resolvable path with existing parent passes

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path = ".ito/state/ito.db"`
- **AND** the parent directory `.ito/state/` exists
- **WHEN** rule `repository/sqlite-db-path-set` runs
- **THEN** it SHALL emit no issues

#### Scenario: Filesystem mode skips the rule

- **GIVEN** `repository.mode = "filesystem"`
- **WHEN** the engine filters rules
- **THEN** rule `repository/sqlite-db-path-set` SHALL be reported as skipped

### Requirement: Rule repository/sqlite-db-not-committed enforces gitignore coverage

When `repository.mode = "sqlite"`, the system SHALL emit a `WARNING` issue if `repository.sqlite.db_path` is set and resolvable but is not covered by `.gitignore`. The rule SHALL emit an `ERROR` if the database file is currently tracked by git (i.e. `git ls-files --error-unmatch <path>` would succeed).

- **Requirement ID**: validate-repo-repository-rules:sqlite-db-not-committed

#### Scenario: Tracked database file fails

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path = ".ito/state/ito.db"`
- **AND** the file is tracked by git
- **WHEN** rule `repository/sqlite-db-not-committed` runs
- **THEN** it SHALL emit an `ERROR` issue
- **AND** the issue's `fix` metadata SHALL include the `git rm --cached` command and the gitignore line to add

#### Scenario: Untracked but unignored database file warns

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path = ".ito/state/ito.db"`
- **AND** the file is not tracked but `.gitignore` does not match it
- **WHEN** rule `repository/sqlite-db-not-committed` runs
- **THEN** it SHALL emit a `WARNING` issue with a `fix` recommending the gitignore entry

#### Scenario: Properly ignored database passes

- **GIVEN** `repository.mode = "sqlite"`
- **AND** `repository.sqlite.db_path = ".ito/state/ito.db"`
- **AND** `.gitignore` matches `.ito/state/*.db`
- **WHEN** rule `repository/sqlite-db-not-committed` runs
- **THEN** it SHALL emit no issues
<!-- ITO:END -->
