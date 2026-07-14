## ADDED Requirements

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
