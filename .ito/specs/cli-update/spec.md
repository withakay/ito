<!-- ITO:START -->
## ADDED Requirements

### Requirement: Update Is Idempotent and Non-Destructive

The system SHALL make `ito update` idempotent and non-destructive for user-owned files.

#### Scenario: Repeated update is stable

- **GIVEN** a project has already been updated
- **WHEN** `ito update` is executed again
- **THEN** the resulting installed files SHALL be unchanged

#### Scenario: Update does not require force

- **GIVEN** a project contains pre-existing files
- **WHEN** `ito update` is executed
- **THEN** the update SHALL complete without requiring `--force`
- **AND** SHALL only change files that are Ito-managed or marker-managed
<!-- ITO:END -->
