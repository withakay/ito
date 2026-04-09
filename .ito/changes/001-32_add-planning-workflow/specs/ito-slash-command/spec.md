<!-- ITO:START -->
## ADDED Requirements

### Requirement: Planning slash command installation

The system SHALL install a dedicated `ito-plan` slash command wrapper during Ito initialization and updates for supported agent harnesses. The wrapper MUST load the `ito-plan` skill and pass the user request through unchanged.

- **Requirement ID**: `ito-slash-command:planning-slash-command-installation`

#### Scenario: Planning slash command installed during init

- **WHEN** a supported harness installs Ito command assets
- **THEN** the harness command directory includes an `ito-plan` command file
- **AND** the command file loads the `ito-plan` skill
- **AND** the command passes the user-provided planning topic or request through to that skill unchanged
- **AND** the installed command is ready for users to invoke with `/ito-plan`

#### Scenario: Planning slash command installed during updates

- **WHEN** a supported harness updates Ito command assets
- **THEN** the harness command directory includes an `ito-plan` command file
- **AND** the command file loads the `ito-plan` skill
- **AND** the command passes the user-provided planning topic or request through unchanged
- **AND** the installed command is ready for users to invoke with `/ito-plan`
<!-- ITO:END -->
