<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Interactive viewer selection

The system SHALL prompt the user to select a viewer each time `ito view proposal` is invoked, unless a viewer is specified via flag. The viewer prompt SHALL include `html (browser)` as an option when `pandoc` is detected on the system.

#### Scenario: Viewer prompt shown

- **WHEN** user runs `ito view proposal <change-id>` without `--viewer`
- **THEN** the system presents an interactive selection prompt listing available viewers
- **AND** the prompt includes only viewers whose backing tool is detected on the system

#### Scenario: Viewer flag bypasses prompt

- **WHEN** user runs `ito view proposal <change-id> --viewer <name>`
- **THEN** the system skips the interactive prompt and opens the document directly in the specified viewer

#### Scenario: Specified viewer not installed

- **WHEN** user passes `--viewer <name>` and the backing tool is not found on PATH
- **THEN** the system displays an error naming the missing tool and how to install it
- **AND** exits with a non-zero status code

#### Scenario: No viewers available

- **WHEN** none of the supported viewer tools are detected on the system
- **THEN** the system displays an error listing the supported tools and how to install them
- **AND** exits with a non-zero status code

#### Scenario: HTML viewer appears when pandoc is present

- **WHEN** `pandoc` is detected on PATH
- **THEN** `html (browser)` appears as an option in the interactive viewer prompt
- **AND** `--viewer html` is accepted as a valid flag value
<!-- ITO:END -->
