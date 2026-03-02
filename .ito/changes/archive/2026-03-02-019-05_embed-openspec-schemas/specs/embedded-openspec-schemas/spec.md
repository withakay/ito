## ADDED Requirements

### Requirement: Embed curated OpenSpec schemas as built-in assets

Ito MUST embed a curated set of OpenSpec schemas as built-in schema assets.

At minimum, the embedded set MUST include schemas named `minimalist` and `event-driven`.

#### Scenario: Embedded schemas are available without installation

- **GIVEN** a user has installed Ito with its built-in assets
- **WHEN** the user lists available schemas
- **THEN** `minimalist` and `event-driven` are included in the available schema names

#### Scenario: Embedded schemas can be exported

- **WHEN** the user runs `ito templates schemas export -f '.ito/templates/schemas'`
- **THEN** the export output includes `.ito/templates/schemas/minimalist/`
- **AND** the export output includes `.ito/templates/schemas/event-driven/`

### Requirement: Embedded OpenSpec schemas include unambiguous attribution

When Ito embeds OpenSpec schemas from an upstream third-party repository, Ito MUST include explicit, repository-tracked attribution that names the upstream project and URL and complies with the upstream license requirements.

#### Scenario: Attribution exists in-tree

- **WHEN** inspecting the Ito repository
- **THEN** an attribution artifact exists that credits `https://github.com/intent-driven-dev/openspec-schemas`
- **AND** the artifact indicates which schemas were vendored

### Requirement: Embedded OpenSpec schemas ship Ito validation configuration

Each embedded OpenSpec schema MUST ship an Ito-authored `validation.yaml` alongside its `schema.yaml`.

The validation configuration MUST avoid Ito delta-spec assumptions and MUST provide a clear manual-validation signal for schema semantics.

#### Scenario: Embedded OpenSpec schema includes validation.yaml

- **GIVEN** the embedded schema directory `schemas/<name>/` exists in built-in assets
- **WHEN** inspecting the schema directory
- **THEN** it contains `schema.yaml`
- **AND** it contains `validation.yaml`

#### Scenario: Validate emits manual validation note

- **GIVEN** a change selects schema `minimalist`
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation does not fail due to missing Ito delta specs
- **AND** validation includes an informational issue indicating semantic validation is manual
