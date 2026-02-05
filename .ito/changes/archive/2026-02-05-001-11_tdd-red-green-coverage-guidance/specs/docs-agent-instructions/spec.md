## ADDED Requirements

### Requirement: TDD and coverage guidance is explicit and configurable

AI-facing instructions SHALL explicitly encourage a RED/GREEN/REFACTOR workflow and reference a default coverage target, while describing how projects can override defaults.

#### Scenario: AGENTS guidance includes RED/GREEN and coverage target

- **WHEN** an agent reads `.ito/AGENTS.md`
- **THEN** it includes a concise section describing RED/GREEN/REFACTOR
- **AND** it references a default coverage target of 80%
- **AND** it describes where to configure overrides
