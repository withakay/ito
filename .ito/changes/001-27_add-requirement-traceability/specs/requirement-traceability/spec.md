## ADDED Requirements

### Requirement: Change-local requirement references

The system SHALL support change-local requirement references so delta requirements can be linked to planned implementation work without requiring current-truth spec migrations.

#### Scenario: Requirement reference is available within a change

- **GIVEN** a change delta requirement declares a requirement reference id
- **WHEN** Ito loads that change's proposal artifacts
- **THEN** the requirement reference is available to change-local validation and review workflows

### Requirement: Requirement coverage is computed from enhanced tasks

The system SHALL compute requirement coverage for a change by matching declared requirement references to enhanced task references in that change's tracking file.

#### Scenario: Referenced requirement is covered by an active task

- **GIVEN** a change declares requirement reference `tasks-tracking:enhanced-requirements`
- **AND** an enhanced task declares that requirement reference
- **WHEN** Ito computes traceability for the change
- **THEN** the requirement is reported as covered by that task

#### Scenario: Shelved task does not satisfy coverage

- **GIVEN** the only task referencing a declared requirement is shelved
- **WHEN** Ito computes traceability for the change
- **THEN** the requirement is reported as uncovered

### Requirement: Unresolved task references are surfaced

The system SHALL surface task references that do not resolve to declared requirement references in the same change.

#### Scenario: Task references unknown requirement id

- **GIVEN** an enhanced task declares requirement reference `delta-specs:missing`
- **AND** the change declares no such requirement reference
- **WHEN** Ito computes traceability for the change
- **THEN** the result identifies that task reference as unresolved
