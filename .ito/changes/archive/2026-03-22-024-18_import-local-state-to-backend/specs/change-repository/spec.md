## MODIFIED Requirements

### Requirement: Backend export operations can read full change history

For backend export and import operations, `ChangeRepository` SHALL provide access to all changes needed for a full-history transfer, including active and archived lifecycle states.

#### Scenario: Export reads active and archived changes

- **GIVEN** backend-managed state includes active and archived changes
- **WHEN** Ito prepares a backend export archive
- **THEN** `ChangeRepository` returns all change artifacts required for export across both lifecycle states

#### Scenario: Imported archived changes remain archived in backend reads

- **GIVEN** archived local changes have been imported into backend-managed state
- **WHEN** Ito reads changes through the backend-backed `ChangeRepository`
- **THEN** imported archived changes are returned as archived history
- **AND** imported active changes remain active
