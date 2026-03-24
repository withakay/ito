<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Parse loose change ID formats

The system SHALL accept both plain module change ID formats (`NNN-NN_name`) and sub-module change ID formats (`NNN.SS-NN_name`), normalizing all components to their canonical zero-padded widths.

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

#### Scenario: Sub-module change ID with loose components

- **WHEN** user provides change ID `24.1-3_foo`
- **THEN** system normalizes to `024.01-03_foo`

#### Scenario: Sub-module change ID already canonical

- **WHEN** user provides change ID `024.01-03_foo`
- **THEN** system returns `024.01-03_foo` unchanged

#### Scenario: Sub-module change ID with excessive padding

- **WHEN** user provides change ID `0024.001-0003_foo`
- **THEN** system normalizes to `024.01-03_foo`

### Requirement: Implement parser as reusable utility

The parser SHALL be implemented as a standalone utility function that can be used across all CLI commands.

#### Scenario: Parser exported for CLI use

- **WHEN** CLI command needs to parse a module, sub-module, or change ID
- **THEN** it can import and use reusable parse helpers instead of duplicating inline string splitting logic

#### Scenario: Parser returns structured result for module change ID

- **WHEN** parsing a valid module change ID like `1-2_bar`
- **THEN** parser returns object with `{ module_id: "001", sub_module_id: null, change_num: "02", name: "bar", canonical: "001-02_bar" }`

#### Scenario: Parser returns structured result for sub-module change ID

- **WHEN** parsing a valid sub-module change ID like `24.1-3_foo`
- **THEN** parser returns object with `{ module_id: "024", sub_module_id: "024.01", change_num: "03", name: "foo", canonical: "024.01-03_foo" }`

## ADDED Requirements

### Requirement: Parse loose sub-module ID formats

The system SHALL accept loose sub-module ID formats (`NNN.SS` or `NNN.SS_name`) and normalize to canonical `NNN.SS` form.

#### Scenario: Loose sub-module ID `24.1`

- **WHEN** user provides sub-module ID `24.1`
- **THEN** system normalizes to `024.01`

#### Scenario: Sub-module ID with name suffix `024.01_auth`

- **WHEN** user provides sub-module ID `024.01_auth`
- **THEN** system extracts and returns `024.01`

#### Scenario: Canonical sub-module ID already correct

- **WHEN** user provides sub-module ID `024.01`
- **THEN** system returns `024.01` unchanged

### Requirement: Reject invalid sub-module ID formats

The system SHALL reject malformed sub-module IDs and sub-module change IDs with helpful errors.

#### Scenario: Invalid sub-module ID format

- **WHEN** user provides sub-module ID `024..01`
- **THEN** system returns an error explaining the expected `NNN.SS` format

#### Scenario: Invalid sub-module change ID separator

- **WHEN** user provides change ID `024_01-03_foo`
- **THEN** system returns an error explaining the expected `NNN.SS-NN_name` format
<!-- ITO:END -->
