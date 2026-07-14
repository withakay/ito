## ADDED Requirements

### Requirement: Archive and promotion update backend-managed history mirrors

When archive/promotion succeeds, Ito SHALL update backend-managed history mirrors for archived changes and promoted specs in addition to producing the Git projection.

#### Scenario: Archive mirrors archived change and promoted specs to backend-managed state

- **GIVEN** remote persistence mode is active
- **WHEN** the user runs `ito archive <change-id>` successfully
- **THEN** Ito updates the backend-managed archived-change history
- **AND** Ito updates the backend-managed promoted-spec state corresponding to the new Git projection
- **AND** Ito still produces the committable Git archive/spec output
