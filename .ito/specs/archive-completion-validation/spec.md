# Archive Completion Validation Specification

## Purpose

Define the `archive-completion-validation` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Pre-archive validation execution

The ito archive command MUST execute validation checks before proceeding with the archive operation. Validation SHALL check that all required artifacts are present and that the change is in a complete state.

#### Scenario: Archive with validation enabled

- **WHEN** user runs 'ito archive change-123'
- **THEN** command executes validation before archiving
- **AND** validation checks all required artifacts (proposal.md, spec.md with scenarios)
- **AND** validation checks implementation completion status
- **AND** archive proceeds only if validation passes

#### Scenario: Archive with --force flag bypasses validation

- **WHEN** user runs 'ito archive change-123 --force'
- **THEN** validation is skipped
- **AND** archive proceeds regardless of completion status
- **AND** warning is displayed indicating bypass of validation

### Requirement: Change completeness criteria

Validation MUST define clear criteria for what constitutes a complete change. A change is considered complete when all required artifacts exist and implementation is finished.

#### Scenario: Proposal presence check

- **WHEN** validation runs on a change
- **THEN** validation checks for proposal.md file
- **AND** if proposal.md is missing, change fails validation
- **AND** error message indicates missing proposal

#### Scenario: Specs with scenarios check

- **WHEN** validation runs on a change
- **THEN** validation checks for at least one spec.md file
- **AND** validation verifies each spec contains at least one "#### Scenario:" block
- **AND** if no scenarios exist, change fails validation
- **AND** error message indicates missing or incomplete specs

#### Scenario: Implementation completion check

- **WHEN** validation runs on a change
- **THEN** validation checks if implementation is complete
- **AND** if implementation has pending tasks, change fails validation
- **AND** error message indicates incomplete implementation with task count

### Requirement: Validation status reporting

Validation MUST provide clear, actionable feedback about why a change fails validation. Error messages SHALL indicate what is missing and suggest appropriate next steps.

#### Scenario: Missing proposal error

- **WHEN** validation detects missing proposal.md
- **THEN** error message states "Missing proposal.md artifact"
- **AND** suggests running 'ito instructions proposal --change <id>'

#### Scenario: Missing specs error

- **WHEN** validation detects missing or incomplete specs
- **THEN** error message states "Specs are missing or incomplete"
- **AND** suggests running 'ito spec create <name> --change <id>'

#### Scenario: Incomplete implementation error

- **WHEN** validation detects incomplete implementation
- **THEN** error message states "Implementation is incomplete: X tasks remaining"
- **AND** suggests running 'ito status --change <id>' to view remaining tasks

### Requirement: Validation exit codes

The validation process MUST return appropriate exit codes to indicate success or failure. This allows scripts and CI/CD pipelines to handle validation failures appropriately.

#### Scenario: Validation passes

- **WHEN** all validation checks pass
- **THEN** validation returns exit code 0
- **AND** archive proceeds

#### Scenario: Validation fails

- **WHEN** any validation check fails
- **THEN** validation returns non-zero exit code
- **AND** archive is aborted
- **AND** error details are displayed

### Requirement: Validation in strict mode

When validation is run in strict mode (--strict flag), additional checks SHALL be performed to enforce higher quality standards.

#### Scenario: Strict mode additional checks

- **WHEN** validation runs with --strict flag
- **THEN** validation performs standard completeness checks
- **AND** validation additionally checks spec formatting compliance
- **AND** validation verifies scenario testability
- **AND** if strict checks fail, change fails validation

### Requirement: Validation caching

For performance, validation MUST cache results of expensive checks (e.g., implementation status) and reuse them when multiple archive operations are requested in sequence.

#### Scenario: Cache hit for repeated validation

- **WHEN** validation is run on the same change twice within cache TTL
- **THEN** cached validation results are used
- **AND** expensive checks are skipped
- **AND** validation completes faster
