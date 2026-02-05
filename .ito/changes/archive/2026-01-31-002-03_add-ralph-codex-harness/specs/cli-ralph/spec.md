## MODIFIED Requirements

### Requirement: Codex harness

The `ito ralph` command SHALL support selecting the Codex harness.

#### Scenario: Select codex harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness codex`
- **THEN** the system invokes the Codex CLI to execute the prompt
- **AND** captures output for completion promise detection

#### Scenario: Pass model to codex harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness codex --model <model>`
- **THEN** the system passes `<model>` to the Codex harness (when supported)
