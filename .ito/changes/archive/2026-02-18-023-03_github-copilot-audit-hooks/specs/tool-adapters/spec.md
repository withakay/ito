<!-- ITO:START -->
## ADDED Requirements

### Requirement: GitHub Copilot Preflight Audit Validation

The system SHALL configure GitHub Copilot repository agent setup steps to validate Ito audit state before the agent runs.

#### Scenario: Setup steps run audit validation

- **GIVEN** the repository includes the Copilot setup steps workflow installed by `ito init --tools github-copilot`
- **WHEN** Copilot begins an agent session
- **THEN** the setup steps SHALL run `ito audit validate`

#### Scenario: Audit validation failure blocks agent run

- **GIVEN** `ito audit validate` fails
- **WHEN** Copilot begins an agent session
- **THEN** the setup steps SHALL fail
- **AND** the failure SHALL include an actionable message indicating audit validation failed
<!-- ITO:END -->
