# Delta Specs Specification

## Purpose

Define the v1 delta spec markdown format used for change deltas under `.ito/changes/<change-id>/specs/**`.

This specification exists so validators and schema validation can reference the format via a stable validator id.

## Requirements

### Requirement: Delta specs have a stable validator id

The v1 validator id for the delta specs format SHALL be `ito.delta-specs.v1`.

#### Scenario: Author can locate the normative spec

- **GIVEN** a validation issue cites `ito.delta-specs.v1`
- **WHEN** an author searches this repository
- **THEN** they find this normative spec at `.ito/specs/delta-specs/spec.md`

### Requirement: Delta specs declare change operations

Delta specs MUST express changes using operation sections.

Canonical operation section headers SHALL be:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

#### Scenario: Delta spec contains at least one operation section

- **WHEN** a change delta spec file is validated
- **THEN** it MUST contain at least one canonical operation section

### Requirement: Delta specs use requirement blocks

Each operation section MUST contain one or more requirement blocks.

Each requirement block MUST begin with a level-3 heading of the form `### Requirement: <name>`.

Each requirement's normative statement MUST contain at least one of: `SHALL`, `MUST`.

#### Scenario: Requirement statement uses normative language

- **GIVEN** a delta spec requirement block
- **WHEN** its requirement text is validated
- **THEN** validation fails if the text contains neither `SHALL` nor `MUST`

### Requirement: Delta specs include scenario blocks

Each requirement block MUST include at least one scenario block.

Each scenario block MUST start with a level-4 heading of the form `#### Scenario: <name>`.

#### Scenario: Missing scenario heading fails validation

- **GIVEN** a delta spec requirement block
- **WHEN** it contains zero `#### Scenario:` headings
- **THEN** validation fails

### Requirement: Validation issues cite the delta specs validator id

Validation issues attributable to delta spec markdown SHALL cite `ito.delta-specs.v1`.

#### Scenario: Error message includes validator id

- **GIVEN** a delta spec fails validation
- **WHEN** a validation issue is reported
- **THEN** the issue message includes `ito.delta-specs.v1`
