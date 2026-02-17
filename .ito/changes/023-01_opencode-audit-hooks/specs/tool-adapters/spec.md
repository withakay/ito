<!-- ITO:START -->
## ADDED Requirements

### Requirement: OpenCode Pre-Tool Audit Hook

The system SHALL install an OpenCode plugin that executes Ito audit checks before tool execution.

#### Scenario: Pre-tool audit runs and provides context

- **GIVEN** OpenCode is configured via `ito init --tools opencode`
- **WHEN** the agent attempts to execute any tool
- **THEN** the plugin SHALL run `ito audit validate` before the tool executes
- **AND** the plugin SHALL inject a short audit status line into the session context when drift or validation failures are detected

#### Scenario: Plugin delegates to Ito CLI

- **GIVEN** the plugin is installed
- **WHEN** audit behavior is required
- **THEN** the plugin SHALL delegate audit behavior to the Ito CLI (`ito audit ...`)
- **AND** the plugin SHALL NOT embed long workflow or policy text

#### Scenario: Audit failure blocks tool execution

- **GIVEN** `ito audit validate` returns a hard failure
- **WHEN** the agent attempts tool execution
- **THEN** the plugin SHALL block the tool execution
- **AND** the plugin SHALL surface an actionable error to the agent/user
<!-- ITO:END -->
