## MODIFIED Requirements

### Requirement: Configuration schema

The CLI SHALL support a well-defined configuration schema that allows for tool-specific, agent-specific, and harness-specific settings.

Notes:

- This extends the existing config system to add harness and agent model configuration.
- Existing cascading config behavior (ito.json → .ito.json → .ito/config.json → $PROJECT_DIR/config.json) is preserved.
- Global config at `~/.config/ito/config.json` is also supported.

#### Scenario: Configuration schema supports harnesses

- **WHEN** reading or writing configuration
- **THEN** support the following harness configuration structure:
  - `harnesses.<harness-id>`: Harness-specific settings
    - `provider`: Provider constraint (null for any, or specific provider name)
    - `agents`: Object mapping agent tier to model configuration
- **AND** support harness IDs: `opencode`, `claude-code`, `codex`, `github-copilot`

#### Scenario: Configuration schema supports agent tiers

- **WHEN** reading or writing configuration
- **THEN** support agent tier keys: `ito-quick`, `ito-general`, `ito-thinking`
- **AND** each tier value can be:
  - A string (model ID shorthand)
  - An object with `model` and extended options

#### Scenario: Configuration schema supports cache

- **WHEN** reading or writing configuration
- **THEN** support the following cache settings:
  - `cache.ttl_hours`: Number of hours before model cache expires

#### Scenario: Configuration merges with defaults

- **WHEN** loading configuration
- **THEN** merge user config with centralized defaults
- **AND** user values override defaults at the leaf level
- **AND** unspecified values use defaults

#### Scenario: Global and project config merge

- **WHEN** both global (`~/.config/ito/config.json`) and project config exist
- **THEN** merge configs with project values winning on conflict
- **AND** harness and agent configurations merge at the agent tier level
