<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Repository-tracked generated config schema artifact

The system SHALL generate a canonical JSON schema artifact for Ito configuration and store it in the repository so editors can resolve it without runtime schema generation. The schema SHALL include the `tools` namespace, including `tools.tmux.enabled`.

#### Scenario: Build generates schema artifact

- **WHEN** the project build/check workflow runs schema generation
- **THEN** it writes a JSON schema file at `schemas/ito-config.schema.json`
- **AND** the file content is derived from the current Rust configuration types
- **AND** the schema includes `tools.tmux.enabled` as a boolean with default `true`

#### Scenario: Schema artifact is committed

- **WHEN** contributors change configuration types or schema metadata
- **THEN** they regenerate `schemas/ito-config.schema.json`
- **AND** the updated schema file is committed in the same change

#### Scenario: Build detects stale schema artifact

- **WHEN** generated schema output differs from the committed `schemas/ito-config.schema.json`
- **THEN** verification fails with guidance to regenerate and commit the schema
<!-- ITO:END -->
