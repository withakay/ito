## ADDED Requirements

### Requirement: `ito list` resolves change data through ChangeRepository

When listing changes, `ito list` SHALL resolve change summaries through the runtime-selected `ChangeRepository` implementation instead of constructing a filesystem repository directly.

#### Scenario: Remote mode lists repository-backed changes

- **GIVEN** remote persistence mode is active
- **WHEN** the user runs `ito list`
- **THEN** the command lists change summaries from the selected remote-backed `ChangeRepository`
- **AND** it does not surface stray local active-change markdown as in-scope changes
