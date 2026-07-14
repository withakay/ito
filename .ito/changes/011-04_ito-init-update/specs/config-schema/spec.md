<!-- ITO:START -->
## ADDED Requirements

### Requirement: Setup coverage classification for config fields

The config schema support code SHALL provide a maintainable classification of config fields that identifies which fields are init-managed, update-refreshable, runtime-only, or intentionally excluded from setup/update handling.

- **Requirement ID**: config-schema:setup-coverage-classification

#### Scenario: Config field has setup classification

- **WHEN** Ito runs the config coverage check
- **THEN** every project config field has an init/update coverage classification or an explicit exclusion

#### Scenario: Coverage classification supports CLI parity checks

- **WHEN** the init/update coverage tests compare config fields to CLI prompts and flags
- **THEN** they use the classification to fail only for fields that should be surfaced by init or update
<!-- ITO:END -->
