## ADDED Requirements

### Requirement: Splash Screen Display

The CLI SHALL display a new, stylized ASCII art banner containing the text "ITO" when the application initializes. The art MUST fit within a standard 80-column terminal width to ensure it displays correctly on most screens without wrapping.

#### Scenario: Application Startup

- **WHEN** the user runs the `ito` command
- **THEN** the CLI outputs the new ASCII art banner before any other text

#### Scenario: Terminal Width Compatibility

- **WHEN** the terminal width is set to 80 columns
- **THEN** the ASCII art banner displays completely on single lines without wrapping to the next line
