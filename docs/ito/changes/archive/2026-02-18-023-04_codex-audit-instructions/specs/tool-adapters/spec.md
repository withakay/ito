<!-- ITO:START -->
## ADDED Requirements

### Requirement: Codex Audit Instructions

The system SHALL install Codex instruction content that makes Ito audit validation a mandatory guardrail.

#### Scenario: Instruction requires audit validation

- **GIVEN** Codex is configured via `ito init --tools codex`
- **WHEN** the agent begins work in a Codex session
- **THEN** the installed instructions SHALL require running `ito audit validate` before stateful work proceeds

#### Scenario: Audit validation failure requires user intervention

- **GIVEN** `ito audit validate` fails
- **WHEN** the agent is operating under the installed instructions
- **THEN** the agent SHALL stop and request guidance rather than continuing with tool use
<!-- ITO:END -->
