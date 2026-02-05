# delta-migration-utility Specification

## Purpose

TBD - created by archiving change 001-02_interactive-splitting. Update Purpose after archive.

## Requirements

### Requirement: Move delta specs between changes

The system SHALL provide a utility to move spec files representing deltas from one change to another.

#### Scenario: Move entire spec file

- **WHEN** moving a spec file `specs/feature/spec.md` from Change A to Change B
- **THEN** system copies file to Change B `specs/feature/spec.md`
- **AND** system removes file from Change A

#### Scenario: Handle directory creation

- **WHEN** moving spec to Change B where `specs/feature` directory doesn't exist
- **THEN** system creates necessary directories in Change B

#### Scenario: Detect collision

- **WHEN** moving spec `specs/feature/spec.md` to Change B where it already exists
- **THEN** system errors with "Spec already exists in destination" or prompts for rename

### Requirement: Update tasks references (Optional)

The system SHALL attempt to move associated tasks when moving specs.

#### Scenario: Move associated tasks

- **WHEN** moving specs from Change A to Change B
- **THEN** system scans Change A `tasks.md` for tasks referencing the moved specs
- **AND** system moves those tasks to Change B `tasks.md` (appending to list)
- **NOTE**: This is best-effort heuristics based on text matching

### Requirement: Validate both changes post-split

The system SHALL validate both the source and destination changes after a split operation to ensure integrity.

#### Scenario: Post-split validation

- **WHEN** split operation completes
- **THEN** system runs validation on Source Change
- **AND** system runs validation on Destination Change
- **AND** system reports any issues introduced by the split
