## MODIFIED Requirements

### Requirement: Schema Directory Structure

The system SHALL support self-contained schema directories with co-located templates.

#### Scenario: Schema with templates

- **WHEN** a schema directory contains `schema.yaml` and `templates/` subdirectory
- **THEN** artifacts can reference templates relative to the schema's templates directory

#### Scenario: Project-local schema override

- **WHEN** a schema directory exists at `.ito/templates/schemas/<name>/`
- **THEN** the system uses that directory in preference to all non-project schema sources

#### Scenario: User schema override

- **WHEN** a schema directory exists at `${XDG_DATA_HOME}/ito/schemas/<name>/`
- **THEN** the system uses that directory instead of built-in defaults when no project-local override exists

#### Scenario: Embedded built-in schema fallback

- **WHEN** no project-local or user override exists for a schema
- **THEN** the system loads the built-in schema directory from embedded assets in `ito-templates/assets/schemas/<name>/`

#### Scenario: Legacy package path fallback during migration

- **WHEN** an embedded built-in schema is unavailable and a package `schemas/<name>/` directory exists
- **THEN** the system MAY use the package directory as a temporary compatibility fallback

#### Scenario: List available schemas

- **WHEN** listing schemas
- **THEN** the system returns schema names from project-local, user, and built-in sources
