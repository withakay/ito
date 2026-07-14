<!-- ITO:START -->
## ADDED Requirements

### Requirement: Claude Code Pre-Tool Audit Hook

The system SHALL install a Claude Code hook configuration that runs Ito audit checks before tool execution.

#### Scenario: PreToolUse triggers audit validation

- **GIVEN** Claude Code is configured via `ito init --tools claude`
- **WHEN** the agent attempts to execute a tool matched by the hook configuration
- **THEN** the hook SHALL run `ito audit validate` before tool execution

#### Scenario: Drift is surfaced to the agent

- **GIVEN** the audit log is valid but drift exists between audit state and file state
- **WHEN** the pre-tool hook runs
- **THEN** the hook SHALL run `ito audit reconcile`
- **AND** the hook SHALL inject a short warning into the session context

#### Scenario: Hard audit failure blocks tool execution

- **GIVEN** `ito audit validate` indicates the audit log is invalid or corrupted
- **WHEN** the agent attempts tool execution
- **THEN** the hook SHALL block the tool execution
- **AND** the hook SHALL surface an actionable error message
<!-- ITO:END -->
