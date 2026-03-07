## ADDED Requirements

### Requirement: ModuleRepository supports runtime-selected implementations

`ModuleRepository` SHALL support both filesystem-backed and remote-backed implementations, with callers resolving module data through the selected implementation for the current persistence mode.

#### Scenario: Remote mode lists modules through selected repository

- **GIVEN** remote persistence mode is active
- **WHEN** a caller requests modules through `ModuleRepository`
- **THEN** the repository returns module summaries from the selected remote-backed implementation

#### Scenario: Remote mode resolves a module without local markdown

- **GIVEN** remote persistence mode is active
- **AND** local `.ito/modules/` markdown is absent
- **WHEN** a caller loads a module by ID or name through `ModuleRepository`
- **THEN** the repository returns the module from the selected remote-backed implementation
