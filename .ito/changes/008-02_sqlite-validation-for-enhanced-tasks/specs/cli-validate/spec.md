## ADDED Requirements

### Requirement: Validate duplicate numeric change IDs

The `ito validate` command SHALL treat `NNN-NN` as the canonical change identity and SHALL fail validation if multiple change directories share the same numeric identity.

#### Scenario: Duplicate numeric change IDs

- **GIVEN** `.ito/changes/008-01_foo/` exists
- **AND** `.ito/changes/008-01_bar/` exists
- **WHEN** executing `ito validate --changes`
- **THEN** validation reports an error for duplicate change ID `008-01`
- **AND** the error includes both directory paths
- **AND** the error suggests renaming/removing one directory

### Requirement: Validate canonical change directory naming

The `ito validate` command SHALL require that change directories match the canonical pattern `NNN-NN_<slug>`.

#### Scenario: Missing slug in change directory

- **GIVEN** `.ito/changes/008-01/` exists
- **WHEN** executing `ito validate --changes`
- **THEN** validation reports an error indicating the slug is required
- **AND** the error suggests renaming the directory to `008-01_<slug>`
