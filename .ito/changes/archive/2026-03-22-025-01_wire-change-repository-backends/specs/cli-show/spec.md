## ADDED Requirements

### Requirement: `ito show <change-id>` resolves changes through ChangeRepository

When showing a change, `ito show` SHALL load the target change through the runtime-selected `ChangeRepository` implementation.

#### Scenario: Remote mode shows a change without local markdown artifacts

- **GIVEN** remote persistence mode is active
- **AND** the requested change exists in the selected remote-backed repository
- **WHEN** the user runs `ito show <change-id>`
- **THEN** Ito renders the change from repository-backed data
- **AND** the command succeeds even if local active-change markdown is absent
