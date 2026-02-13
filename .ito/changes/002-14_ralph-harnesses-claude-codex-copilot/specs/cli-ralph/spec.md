<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph supports additional harness names

The system SHALL support selecting additional harness integrations via `--harness`:

- `claude`
- `codex`
- `github-copilot`
- `copilot` (alias for `github-copilot`)

#### Scenario: Claude harness selected

- **WHEN** executing `ito ralph --harness claude --change <change-id> --no-interactive "<prompt>"`
- **THEN** the system SHALL run the Ralph loop using the Claude Code harness integration

#### Scenario: Codex harness selected

- **WHEN** executing `ito ralph --harness codex --change <change-id> --no-interactive "<prompt>"`
- **THEN** the system SHALL run the Ralph loop using the Codex harness integration

#### Scenario: GitHub Copilot harness selected

- **WHEN** executing `ito ralph --harness github-copilot --change <change-id> --no-interactive "<prompt>"`
- **THEN** the system SHALL run the Ralph loop using the GitHub Copilot harness integration

#### Scenario: Copilot harness alias selected

- **WHEN** executing `ito ralph --harness copilot --change <change-id> --no-interactive "<prompt>"`
- **THEN** the system SHALL run the Ralph loop using the GitHub Copilot harness integration

### Requirement: Unknown harness produces a clear error

The system SHALL fail with a clear error when an unknown harness name is provided.

#### Scenario: Unknown harness rejected

- **WHEN** executing `ito ralph --harness does-not-exist --change <change-id> --no-interactive "<prompt>"`
- **THEN** the system SHALL exit non-zero
- **AND** the error message SHALL include the unknown harness name
<!-- ITO:END -->
