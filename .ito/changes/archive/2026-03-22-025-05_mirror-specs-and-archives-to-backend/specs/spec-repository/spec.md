## ADDED Requirements

### Requirement: SpecRepository provides repository-backed access to promoted specs

Ito SHALL provide a `SpecRepository` abstraction for reading promoted truth specs through the selected persistence implementation.

#### Scenario: Filesystem mode reads promoted specs from Git projection

- **GIVEN** filesystem persistence mode is active
- **WHEN** a caller requests promoted specs through `SpecRepository`
- **THEN** the repository reads the promoted specs from the local `.ito/specs/` projection

#### Scenario: Remote mode reads promoted specs from backend-managed state

- **GIVEN** remote persistence mode is active
- **WHEN** a caller requests promoted specs through `SpecRepository`
- **THEN** the repository returns promoted specs from the selected remote-backed implementation

### Requirement: Backend-managed state retains promoted specs and archived changes for reconciliation

The backend-managed persistence state SHALL retain promoted specs and archived change history so clients can query and reconcile full project history.

#### Scenario: Query archived project history from backend-managed state

- **GIVEN** archived changes and promoted specs have been mirrored into backend-managed state
- **WHEN** a client requests full project history for reconciliation or export
- **THEN** the backend-managed state includes archived changes and promoted specs needed for that query
