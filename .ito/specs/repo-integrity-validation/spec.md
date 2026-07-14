# Repo Integrity Validation Specification

## Purpose

Define repository integrity validation rules for modules, changes, and naming conventions.

## Requirements

### Requirement: Canonical module identity

The validator SHALL derive module identity from module directory prefixes under `.ito/modules/` and SHALL normalize IDs for comparison and diagnostics.

#### Scenario: Normalize a module ID

- **GIVEN** a module directory named `.ito/modules/008_todo-task-system/`
- **WHEN** validating repository integrity
- **THEN** the module is recorded with numeric ID `8` and canonical text ID `008`
- **AND** diagnostics referring to the module use the canonical text ID `008`

### Requirement: Canonical change identity is numeric-only

The validator SHALL treat the numeric prefix `NNN-NN` of a change directory as the change's canonical identity and SHALL treat the slug as required metadata.

#### Scenario: Duplicate numeric change IDs with different slugs

- **GIVEN** `.ito/changes/008-01_foo/` exists
- **AND** `.ito/changes/008-01_bar/` exists
- **WHEN** running `ito validate --changes` or `ito validate --all`
- **THEN** validation fails with an error for duplicate change ID `008-01`
- **AND** the error lists both paths and instructs the user to rename/remove one directory

#### Scenario: Change directory missing slug

- **GIVEN** `.ito/changes/008-01/` exists
- **WHEN** running `ito validate --changes` or `ito validate --all`
- **THEN** validation fails with an error stating the required directory pattern is `NNN-NN_<slug>`
- **AND** the error suggests renaming the directory to include a slug (for example `008-01_example`)

### Requirement: Changes reference an existing module

The validator SHALL require that the module prefix of a change directory corresponds to an existing module directory.

#### Scenario: Change refers to missing module

- **GIVEN** `.ito/changes/999-01_some-change/` exists
- **AND** there is no module directory with prefix `999_` under `.ito/modules/`
- **WHEN** running `ito validate --changes` or `ito validate --all`
- **THEN** validation fails with an error indicating module `999` is missing
- **AND** the error suggests creating the module or moving the change into an existing module

### Requirement: Repository integrity issues include actionable locations

Repository integrity issues SHALL include a precise location and remediation instructions.

#### Scenario: Duplicate change IDs include both directories

- **GIVEN** duplicate numeric change IDs exist
- **WHEN** validation reports the issue
- **THEN** the issue includes both directory paths
- **AND** the issue includes the canonical change ID in the message
- **AND** the issue includes at least one suggested remediation step
