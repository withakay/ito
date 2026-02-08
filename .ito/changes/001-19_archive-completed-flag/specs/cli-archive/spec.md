## ADDED Requirements

### Requirement: Batch Archive Completed Changes

The archive command SHALL support a `--completed` flag that discovers and archives all changes whose tasks are fully complete.

#### Scenario: Archive all completed changes

- **WHEN** executing `ito archive --completed`
- **THEN** the system SHALL query `ChangeRepository::list_complete()` to find all changes with `ChangeStatus::Complete`
- **AND** archive each change sequentially using the standard single-change archive flow (validation, spec updates, move)
- **AND** display per-change progress (change name and result)
- **AND** display a summary on completion showing total archived and any failures

#### Scenario: No completed changes found

- **WHEN** executing `ito archive --completed` and no changes have `ChangeStatus::Complete`
- **THEN** display a message "No completed changes to archive." and exit successfully

#### Scenario: Combined with --yes flag

- **WHEN** executing `ito archive --completed --yes`
- **THEN** skip all per-change confirmation prompts (task warnings, spec update confirmations)
- **AND** archive each completed change non-interactively

#### Scenario: Combined with --skip-specs flag

- **WHEN** executing `ito archive --completed --skip-specs`
- **THEN** skip spec updates for every archived change

#### Scenario: Combined with --no-validate flag

- **WHEN** executing `ito archive --completed --no-validate`
- **THEN** skip task completion validation for every archived change

#### Scenario: Partial failure during batch archive

- **WHEN** one change fails to archive during batch mode (e.g., archive name collision, filesystem error)
- **THEN** report the error for that change
- **AND** continue archiving remaining changes
- **AND** include the failure in the summary
- **AND** exit with non-zero status if any change failed

### Requirement: Mutual Exclusivity of --completed and CHANGE Argument

The `--completed` flag and the positional `CHANGE` argument SHALL be mutually exclusive.

#### Scenario: Both --completed and CHANGE provided

- **WHEN** executing `ito archive some-change --completed`
- **THEN** the CLI SHALL reject the invocation with an error message explaining the conflict
- **AND** exit with non-zero status

### Requirement: Batch Archive Summary Output

The batch archive mode SHALL provide a clear summary of results.

#### Scenario: All changes archived successfully

- **WHEN** all completed changes are archived without error
- **THEN** display: "Archived N change(s)." followed by the list of archived change names

#### Scenario: Some changes failed

- **WHEN** some changes fail during batch archive
- **THEN** display: "Archived N change(s), M failed." followed by the list of successes and failures
