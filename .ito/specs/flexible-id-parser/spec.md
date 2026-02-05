# flexible-id-parser Specification

## Purpose

TBD - created by archiving change 001-01_flexible-id-parsing. Update Purpose after archive.

## Requirements

### Requirement: Parse loose module ID formats

The system SHALL accept loose module ID formats and normalize them to canonical 3-digit padded format.

#### Scenario: Single digit module ID

- **WHEN** user provides module ID `1`
- **THEN** system normalizes to `001`

#### Scenario: Two digit module ID

- **WHEN** user provides module ID `01`
- **THEN** system normalizes to `001`

#### Scenario: Three digit module ID (already canonical)

- **WHEN** user provides module ID `001`
- **THEN** system returns `001` unchanged

#### Scenario: Module ID with name suffix

- **WHEN** user provides module ID `1_foo` or `001_foo`
- **THEN** system extracts module number and normalizes to `001`

### Requirement: Parse loose change ID formats

The system SHALL accept loose change ID formats and normalize them to canonical `NNN-NN_name` format.

#### Scenario: Minimal change ID

- **WHEN** user provides change ID `1-2_bar`
- **THEN** system normalizes to `001-02_bar`

#### Scenario: Mixed padding change ID

- **WHEN** user provides change ID `1-00003_bar`
- **THEN** system normalizes to `001-03_bar`

#### Scenario: Full padding change ID (already canonical)

- **WHEN** user provides change ID `001-02_bar`
- **THEN** system returns `001-02_bar` unchanged

#### Scenario: Excessive padding change ID

- **WHEN** user provides change ID `0001-00002_baz`
- **THEN** system normalizes to `001-02_baz`

### Requirement: Reject invalid ID formats

The system SHALL reject IDs that don't match expected patterns and provide helpful error messages.

#### Scenario: Invalid module ID format

- **WHEN** user provides module ID `abc` (non-numeric)
- **THEN** system returns error with message explaining expected format

#### Scenario: Invalid change ID format - missing name

- **WHEN** user provides change ID `001-02` (no name suffix)
- **THEN** system returns error indicating name is required

#### Scenario: Invalid change ID format - bad separator

- **WHEN** user provides change ID `001_02_bar` (wrong separator)
- **THEN** system returns error with correct format example

### Requirement: Implement parser as reusable utility

The parser SHALL be implemented as a standalone utility function that can be used across all CLI commands.

#### Scenario: Parser exported for CLI use

- **WHEN** CLI command needs to parse a module or change ID
- **THEN** it can import and use the `parseModuleId` and `parseChangeId` functions

#### Scenario: Parser returns structured result

- **WHEN** parsing a valid change ID like `1-2_bar`
- **THEN** parser returns object with `{ moduleId: "001", changeNum: "02", name: "bar", canonical: "001-02_bar" }`

### Requirement: Comprehensive test coverage

The parser SHALL have comprehensive unit tests covering all edge cases.

#### Scenario: Test suite covers all input variations

- **WHEN** running parser test suite
- **THEN** tests cover: single digits, multi-digits, excessive padding, with/without names, invalid formats

#### Scenario: Test suite achieves minimum coverage

- **WHEN** running coverage report on parser module
- **THEN** coverage is at least 90% for lines, branches, and functions
