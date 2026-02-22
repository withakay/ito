## ADDED Requirements

### Requirement: Ito internal comments are excluded from rendered guidance

Guidance loading SHALL ignore content contained in Ito internal comment blocks when composing instruction guidance text.

Internal comment block delimiters:

- `<!-- ITO:INTERNAL:START -->`
- `<!-- ITO:INTERNAL:END -->`

#### Scenario: Scoped guidance excludes internal scaffold content

- **GIVEN** `.ito/user-prompts/apply.md` contains placeholder scaffold content inside Ito internal comment block delimiters
- **AND** the file contains real guidance content outside those delimiters
- **WHEN** a user runs `ito agent instruction apply --change "<change-id>"`
- **THEN** the rendered output includes only the real guidance content
- **AND** the placeholder scaffold content is not rendered

#### Scenario: Shared guidance excludes internal scaffold content

- **GIVEN** `.ito/user-prompts/guidance.md` contains placeholder scaffold content inside Ito internal comment block delimiters
- **AND** the file contains real guidance content outside those delimiters
- **WHEN** a user runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** composed guidance includes the real shared guidance content
- **AND** the placeholder scaffold content is not rendered
