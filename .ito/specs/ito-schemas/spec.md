# Spec: ito-schemas

## Purpose

Define the `ito-schemas` capability and its current-truth behavior for workflow schema metadata and schema-driven validation.

## Requirements

### Requirement: Schemas may define validation.yaml

Ito MUST allow a workflow schema directory to include a `validation.yaml` file next to `schema.yaml` to declare validation rules for that schema's artifacts.

#### Scenario: Schema includes validation.yaml

- **GIVEN** a schema directory contains `schema.yaml`
- **WHEN** the directory also contains `validation.yaml`
- **THEN** Ito MUST treat `validation.yaml` as the schema's validation configuration

### Requirement: validation.yaml uses versioned validator identifiers

Schema validation rules MUST reference validators using stable, versioned identifier strings (for example, `ito.delta-specs.v1`).

#### Scenario: Unknown validator identifier

- **GIVEN** `validation.yaml` references a validator identifier that Ito does not recognize
- **WHEN** validating a change that uses this schema
- **THEN** validation MUST report an error indicating the validator is unknown

### Requirement: validation.yaml uses snake_case keys

The `validation.yaml` format MUST use `snake_case` keys.

#### Scenario: validation.yaml uses snake_case

- **WHEN** parsing `validation.yaml`
- **THEN** Ito MUST accept `snake_case` field names as the canonical format

### Requirement: Embedded schemas ship validation.yaml

Ito SHALL ship a `validation.yaml` file alongside each embedded workflow schema's `schema.yaml` when that schema is intended to be validated automatically by `ito validate`.

#### Scenario: Embedded schema directory includes validation.yaml

- **GIVEN** a schema is embedded in the Ito binary
- **WHEN** the embedded schema includes `schema.yaml`
- **THEN** the embedded schema directory SHALL also include `validation.yaml`
- **AND** `ito validate <change-id>` SHALL report that it is using schema validation
