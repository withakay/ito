## ADDED Requirements

### Requirement: Init hints about project setup when incomplete

`ito init` SHALL print a hint to run project setup when `.ito/project.md` indicates project setup is incomplete.

#### Scenario: Init prints hint when marker is present

- **WHEN** `ito init` completes
- **AND** `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:INCOMPLETE -->`
- **THEN** the CLI prints a hint describing how to run project setup (e.g. `/ito-project-setup` or `ito agent instruction project-setup`)

#### Scenario: Init does not print hint when marker is absent

- **WHEN** `ito init` completes
- **AND** `.ito/project.md` does not contain the incomplete marker
- **THEN** the CLI does not print the project setup hint
