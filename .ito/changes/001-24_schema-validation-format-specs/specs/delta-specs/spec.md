## ADDED Requirements

### Requirement: Delta spec format has a stable validator id and normative spec

The delta spec markdown format SHALL be documented as a first-class, versioned specification.

The v1 validator id for this format SHALL be `ito.delta-specs.v1`.

#### Scenario: Author discovers the correct spec from a validator id

- **GIVEN** a validation issue references `ito.delta-specs.v1`
- **WHEN** an author searches the repository for the spec
- **THEN** the normative spec document for v1 is discoverable at `.ito/specs/delta-specs/spec.md`

### Requirement: Delta spec operations are expressed as operation sections

Delta specs MUST express changes using operation sections.

Canonical operation section headers SHALL be:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

#### Scenario: Operation sections exist

- **WHEN** a delta spec is authored
- **THEN** it MUST contain one or more operation sections
- **AND** each operation section header MUST match one of the canonical operation headers

### Requirement: Delta specs use requirement blocks with normative language

Each operation section MUST contain one or more requirement blocks.

Each requirement block MUST start with a level-3 heading of the form `### Requirement: <name>`.

Each requirement statement MUST use normative language and include at least one of: `SHALL`, `MUST`.

#### Scenario: Requirement block is structurally valid

- **GIVEN** a delta spec requirement block
- **WHEN** the block begins with `### Requirement: ...`
- **THEN** it is recognized as a requirement
- **AND** the requirement text contains `SHALL` or `MUST`

### Requirement: Delta specs include scenario blocks

Every requirement block MUST include at least one scenario block.

Each scenario block MUST start with a level-4 heading of the form `#### Scenario: <name>`.

#### Scenario: Scenario heading exists

- **GIVEN** a requirement block in a delta spec
- **WHEN** the delta spec is validated
- **THEN** validation fails if the requirement contains zero `#### Scenario:` headings

### Requirement: Delta spec validation issues cite the validator id

Validation issues for delta spec markdown SHALL cite the format validator id.

#### Scenario: Validation issue cites validator id

- **GIVEN** a delta spec fails structural validation
- **WHEN** a validation issue is produced
- **THEN** the issue text (or structured metadata) includes `ito.delta-specs.v1`
