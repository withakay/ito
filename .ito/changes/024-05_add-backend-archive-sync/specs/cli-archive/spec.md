## ADDED Requirements

### Requirement: Archive syncs from backend and marks archived in backend mode

When backend mode is enabled, the archive command SHALL pull the canonical backend artifacts for the change, perform the normal local archive flow (validation, spec updates, and move), and then mark the change archived on the backend.

#### Scenario: Backend-mode archive produces committable repo state

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito archive <change-id>`
- **THEN** Ito updates `.ito/specs/` as in filesystem mode
- **AND** archives the change under `.ito/changes/archive/`
- **AND** prints an explicit reminder to commit the archived change and updated specs
