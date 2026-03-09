## ADDED Requirements

### Requirement: ChangeRepository provides lifecycle-aware canonical access

`ChangeRepository` SHALL provide a canonical view of change data across both active and archived lifecycle states, independent of whether the underlying implementation is filesystem-backed or remote-backed.

#### Scenario: List active changes through selected implementation

- **GIVEN** Ito resolves a repository implementation for the current persistence mode
- **WHEN** a caller requests active changes
- **THEN** `ChangeRepository` returns only active changes from that implementation

#### Scenario: List archived changes through the same repository

- **GIVEN** archived changes exist in the selected persistence implementation
- **WHEN** a caller requests archived changes
- **THEN** `ChangeRepository` returns those archived changes without requiring a separate archive repository type

#### Scenario: Resolve a change by canonical ID regardless of lifecycle

- **GIVEN** a canonical change ID exists in either active or archived state
- **WHEN** a caller resolves or loads that change through `ChangeRepository`
- **THEN** the repository returns the matching change from the selected persistence implementation

#### Scenario: Remote mode ignores stray local active-change markdown

- **GIVEN** remote persistence mode is active
- **AND** stale or stray markdown exists under local `.ito/changes/`
- **WHEN** `ChangeRepository` serves change reads
- **THEN** it uses the remote-backed implementation as the canonical source
- **AND** it does not merge in local active-change markdown implicitly
