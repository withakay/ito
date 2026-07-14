<!-- ITO:START -->
## ADDED Requirements

### Requirement: Update flags cover refreshable config settings

`ito update` SHALL expose non-interactive flags for refreshable setup/config settings that can be safely changed during project updates, and SHALL document settings that are intentionally not updateable by flag.

- **Requirement ID**: cli-update:refreshable-config-flag-coverage

#### Scenario: Missing update flag is detected by tests

- **WHEN** a config setting is classified as refreshable by `ito update`
- **AND** no update flag or documented exclusion exists for that setting
- **THEN** the update/config coverage test fails

#### Scenario: Existing explicit config is not overwritten by default

- **GIVEN** a project config contains explicit values for refreshable settings
- **WHEN** the user runs `ito update` without overriding flags
- **THEN** Ito preserves the explicit config values

#### Scenario: Update flag intentionally overrides config

- **GIVEN** a project config contains an explicit value for a refreshable setting
- **WHEN** the user runs `ito update` with a flag for that setting
- **THEN** Ito writes the flag-selected value to config
<!-- ITO:END -->
