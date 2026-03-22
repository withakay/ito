## ADDED Requirements

### Requirement: Change-local requirement references

The system SHALL support change-local requirement references so delta requirements can be linked to planned implementation work without requiring current-truth spec migrations.

#### Scenario: Requirement reference is available within a change

- **GIVEN** a change delta requirement declares a requirement reference id
- **WHEN** Ito loads that change's proposal artifacts
- **THEN** the requirement reference is available to change-local validation and review workflows

### Requirement: Traced changes declare complete requirement ids

When a change opts into requirement traceability, every delta requirement in that change MUST declare a requirement reference id.

#### Scenario: Partial requirement id coverage is invalid

- **GIVEN** a change where one delta requirement declares `- **Requirement ID**: ...`
- **AND** another delta requirement in the same change declares no requirement id
- **WHEN** Ito validates or computes traceability for the change
- **THEN** the change is reported as invalid for computed traceability

### Requirement: Computed traceability is explicit about availability

The system SHALL distinguish between traced changes with computed coverage and changes where computed traceability is unavailable.

#### Scenario: Checkbox-only change reports unavailable traceability

- **GIVEN** a change declares requirement ids
- **AND** its active tracking file does not use enhanced task encoding
- **WHEN** Ito computes traceability for the change
- **THEN** the result reports computed coverage as unavailable
- **AND** it explains that enhanced task metadata is required for requirement-to-task coverage

### Requirement: Archived change bundles retain historical traceability

When an archived change bundle contains requirement traceability metadata, the system SHALL compute traceability from the archived change artifacts without requiring promoted current-truth specs to preserve those ids.

#### Scenario: Archived change computes historical traceability

- **GIVEN** an archived change bundle includes traced delta requirements and enhanced task references
- **WHEN** Ito computes traceability for that archived change
- **THEN** the result is derived from the archived change's delta and tracking artifacts
- **AND** the result is labeled as historical rather than current-truth lineage

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
