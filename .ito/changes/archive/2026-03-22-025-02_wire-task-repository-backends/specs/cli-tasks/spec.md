## ADDED Requirements

### Requirement: `ito tasks` uses selected task persistence in remote mode

When remote persistence mode is active, `ito tasks` commands SHALL read and mutate task state through the selected task persistence implementation instead of assuming direct edits to local tracking markdown.

#### Scenario: Complete task in remote mode without local tasks file

- **GIVEN** remote persistence mode is active
- **AND** the target change has task state in the selected remote-backed persistence implementation
- **WHEN** the user runs `ito tasks complete <change-id> <task-id>`
- **THEN** Ito completes the task through the selected persistence path
- **AND** surfaces success or conflict without requiring a local `tasks.md` file
