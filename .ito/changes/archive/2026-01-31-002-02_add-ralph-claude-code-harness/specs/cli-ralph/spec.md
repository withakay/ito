## MODIFIED Requirements

### Requirement: Claude Code harness

The `ito ralph` command SHALL support selecting the Claude Code harness.

#### Scenario: Select claude-code harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness claude-code`
- **THEN** the system invokes the Claude Code CLI to execute the prompt
- **AND** captures output for completion promise detection

#### Scenario: Pass model to claude-code harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness claude-code --model <model>`
- **THEN** the system passes `<model>` to the Claude Code harness (when supported)
