## MODIFIED Requirements

### Requirement: GitHub Copilot harness

The `ito ralph` command SHALL support selecting the GitHub Copilot harness.

#### Scenario: Select github-copilot harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness github-copilot`
- **THEN** the system invokes the GitHub Copilot CLI to execute the prompt
- **AND** captures output for completion promise detection
