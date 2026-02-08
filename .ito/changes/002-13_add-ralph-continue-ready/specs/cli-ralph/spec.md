## ADDED Requirements

### Requirement: Continuous ready-change mode

The system SHALL support a repo-wide continuation mode that selects the next available eligible change and runs Ralph repeatedly until no further eligible work remains.

Eligible changes are those in `Ready` or `InProgress` work status.

#### Scenario: Continue-ready drains ready changes in deterministic order

- **GIVEN** the repository contains multiple changes in `Ready` or `InProgress` work status
- **WHEN** executing `ito ralph --continue-ready ...`
- **THEN** the system SHALL select the lowest change ID among eligible changes as the execution target
- **AND** after each completed change run, the system SHALL refresh readiness and continue with the next lowest-ID eligible change

#### Scenario: Continue-ready exits successfully when no work remains

- **GIVEN** the repository contains no changes in `Ready` work status
- **AND** all changes are `Complete`
- **WHEN** executing `ito ralph --continue-ready ...`
- **THEN** the command SHALL exit successfully

#### Scenario: Continue-ready fails when blocked work remains

- **GIVEN** the repository contains no changes in `Ready` or `InProgress` work status
- **AND** at least one change is not `Complete`
- **WHEN** executing `ito ralph --continue-ready ...`
- **THEN** the command SHALL fail
- **AND** the error SHALL identify remaining non-complete changes

#### Scenario: Continue-ready reorients on readiness drift

- **GIVEN** `ito ralph --continue-ready` is running
- **AND** another process changes task state between selection and run start
- **WHEN** Ralph performs preflight readiness revalidation
- **THEN** the system SHALL re-select the current lowest-ID ready change
