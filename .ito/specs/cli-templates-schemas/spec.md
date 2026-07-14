<!-- ITO:START -->
# Cli Templates Schemas

## Purpose

This spec defines the current behavior and requirements for cli templates schemas.

## Requirements

These requirements keep built-in workflow schema templates aligned with the validators that Ito actually runs and ensure the export command remains a faithful starting point for project-local customization.

### Requirement: Export built-in schema bundles

The CLI SHALL provide a command to export embedded built-in workflow schemas to a target directory for local customization.

#### Scenario: Export schemas to explicit directory

- **WHEN** the user runs `ito templates schemas export -f '.ito/templates/schemas'`
- **THEN** the CLI writes each available schema as `.ito/templates/schemas/<name>/`
- **AND** each exported schema directory contains `schema.yaml` and `templates/*.md`

#### Scenario: Export creates missing directories

- **WHEN** the export target directory does not exist
- **THEN** the CLI creates required parent directories before writing files

#### Scenario: Export output is deterministic

- **WHEN** export is run multiple times with unchanged embedded schemas
- **THEN** output file content is byte-for-byte identical

### Requirement: Export conflict behavior

The CLI SHALL define predictable behavior when export targets already contain files.

#### Scenario: Export without force preserves existing files

- **WHEN** export target files already exist and `--force` is not provided
- **THEN** existing files are not overwritten
- **AND** the CLI reports which files were skipped

#### Scenario: Export with force overwrites existing files

- **WHEN** export target files already exist and `--force` is provided
- **THEN** existing schema files are overwritten with embedded defaults
- **AND** the CLI reports overwritten files

### Requirement: Discoverability of templates schemas commands

The CLI SHALL make schema export functionality discoverable under the templates command surface.

#### Scenario: Templates help shows schemas export

- **WHEN** the user runs `ito templates --help` or `ito templates schemas --help`
- **THEN** help output includes `schemas export` usage and flags

### Requirement: Built-in schema templates match configured validators

Built-in workflow schema templates MUST use a markdown shape that is accepted by the validators declared in the same schema directory's `validation.yaml`.

- **Requirement ID**: cli-templates-schemas:template-validator-alignment

#### Scenario: Minimalist specs parse as deltas

- **GIVEN** the built-in `minimalist` schema configures `specs` as `validate_as: ito.delta-specs.v1`
- **WHEN** the spec template is rendered into a new change
- **THEN** the rendered file uses `## ADDED Requirements`, `### Requirement:`, and `#### Scenario:` headers (delta-spec shape)
- **AND** does not use `## Stories` or `### Story:` headers

#### Scenario: Event-driven specs parse as deltas

- **GIVEN** the built-in `event-driven` schema configures `specs` as `validate_as: ito.delta-specs.v1`
- **WHEN** the spec template is rendered into a new change
- **THEN** the rendered file uses delta requirement headers and does not use story-shaped headers

#### Scenario: Rendered samples pass strict validation

- **GIVEN** a synthetic minimal change is generated from each built-in schema using only its templates
- **WHEN** `ito validate <change-id> --strict` runs against that synthetic change
- **THEN** validation does not fail because of template/validator format incompatibility

### Requirement: Exported schemas include validation configuration

The `ito templates schemas export` command SHALL include `validation.yaml` (when present) for each exported schema directory.

- **Requirement ID**: cli-templates-schemas:export-validation-assets

#### Scenario: Export includes validation.yaml

- **GIVEN** a built-in schema directory contains `validation.yaml`
- **WHEN** the user runs `ito templates schemas export -f <target>`
- **THEN** the exported directory contains `validation.yaml` alongside `schema.yaml` and `templates/`

#### Scenario: Export remains deterministic

- **GIVEN** the export command runs twice with no embedded changes
- **WHEN** the user inspects the resulting `validation.yaml` files
- **THEN** their content is byte-for-byte identical between runs
<!-- ITO:END -->
