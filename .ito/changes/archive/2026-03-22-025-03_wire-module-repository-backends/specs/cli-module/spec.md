## ADDED Requirements

### Requirement: module CLI commands use ModuleRepository

Module-oriented CLI commands SHALL resolve module data through the runtime-selected `ModuleRepository` implementation.

#### Scenario: List modules in remote mode

- **GIVEN** remote persistence mode is active
- **WHEN** the user runs `ito list --modules`
- **THEN** Ito lists modules from the selected remote-backed `ModuleRepository`
- **AND** the command does not require local `.ito/modules/` markdown to exist

#### Scenario: Show module in remote mode

- **GIVEN** remote persistence mode is active
- **WHEN** the user runs `ito show module <id>`
- **THEN** Ito renders the module from the selected remote-backed `ModuleRepository`
