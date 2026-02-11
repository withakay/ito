## ADDED Requirements

### Requirement: Repository-tracked generated config schema artifact

The system SHALL generate a canonical JSON schema artifact for Ito configuration and store it in the repository so editors can resolve it without runtime schema generation.

#### Scenario: Build generates schema artifact

- **WHEN** the project build/check workflow runs schema generation
- **THEN** it writes a JSON schema file at `schemas/ito-config.schema.json`
- **AND** the file content is derived from the current Rust configuration types

#### Scenario: Schema artifact is committed

- **WHEN** contributors change configuration types or schema metadata
- **THEN** they regenerate `schemas/ito-config.schema.json`
- **AND** the updated schema file is committed in the same change

#### Scenario: Build detects stale schema artifact

- **WHEN** generated schema output differs from the committed `schemas/ito-config.schema.json`
- **THEN** verification fails with guidance to regenerate and commit the schema

### Requirement: Config files reference committed schema for editor completion

Project configuration files SHALL support referencing the committed schema artifact via `$schema` so JSON editors provide completion and validation.

#### Scenario: Project config references local schema file

- **WHEN** a project config file includes a `$schema` property
- **THEN** the value can point to the committed local schema path (for example, `../schemas/ito-config.schema.json` from `.ito/config.json`)
- **AND** editors can provide schema-driven completion and validation from that local file

#### Scenario: Loader ignores schema metadata

- **WHEN** config files include `$schema`
- **THEN** config loading ignores `$schema` metadata
- **AND** runtime behavior is unchanged except for editor/tooling integration
