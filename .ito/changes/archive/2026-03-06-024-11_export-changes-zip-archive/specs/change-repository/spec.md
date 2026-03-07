## ADDED Requirements

### Requirement: Repository export source includes complete lifecycle coverage

For backend export operations, `ChangeRepository` SHALL provide access to all changes needed for a full-history archive, including active and archived lifecycle states.

#### Scenario: Export enumerates active changes from backend

- **GIVEN** backend mode is enabled
- **WHEN** export orchestration requests active changes
- **THEN** `ChangeRepository` returns active change summaries and artifacts from backend state

#### Scenario: Export enumerates archived changes from backend

- **GIVEN** backend mode is enabled
- **WHEN** export orchestration requests archived changes
- **THEN** `ChangeRepository` returns archived change summaries and artifacts from backend state

#### Scenario: Export enumeration is stable for deterministic packaging

- **WHEN** export orchestration requests all changes for packaging
- **THEN** `ChangeRepository` returns changes in deterministic ID order
- **AND** repeated exports over unchanged state produce the same file set in the archive
