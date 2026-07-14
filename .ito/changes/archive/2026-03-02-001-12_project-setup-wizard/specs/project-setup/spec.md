## ADDED Requirements

### Requirement: Wizard-style project setup workflow

Ito SHALL provide a `project-setup` instruction artifact that guides an agent through initializing project-specific development commands.

#### Scenario: Setup workflow can be retrieved

- **WHEN** a user runs `ito agent instruction project-setup`
- **THEN** Ito outputs a template that guides project setup steps

#### Scenario: Setup workflow generates dev command scaffolding

- **WHEN** an agent follows the project setup workflow
- **THEN** it produces a default set of dev command entrypoints (build/test/lint/help)
- **AND** it does not overwrite an existing `Makefile` without explicit confirmation

#### Scenario: Setup workflow marks setup as complete

- **GIVEN** `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->`
- **WHEN** an agent completes the project setup workflow
- **THEN** `.ito/project.md` is updated to contain `<!-- ITO:PROJECT_SETUP:COMPLETE -->`

#### Scenario: Setup workflow supports Windows-friendly entrypoint

- **WHEN** the project setup workflow targets Windows environments
- **THEN** it includes guidance for a PowerShell entrypoint that mirrors build/test/lint/help tasks

### Requirement: Project setup completion marker

Ito SHALL define a machine-checkable marker in `.ito/project.md` indicating whether project setup is complete.

#### Scenario: Marker indicates incomplete setup

- **WHEN** `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->`
- **THEN** the system treats project setup as incomplete

#### Scenario: Marker indicates complete setup

- **WHEN** `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:COMPLETE -->`
- **THEN** the system treats project setup as complete
