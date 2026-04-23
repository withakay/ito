## MODIFIED Requirements

### Requirement: Configuration schema

The CLI SHALL support a well-defined configuration schema that allows for tool-specific, agent-specific, harness-specific, and backend-specific settings.

Notes:

- This extends the existing config system to add harness, agent model, and backend configuration.
- Existing cascading config behavior (ito.json -> .ito.json -> .ito/config.json -> $PROJECT_DIR/config.json) is preserved.
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

#### Scenario: Configuration schema supports backend settings

- **WHEN** reading or writing configuration
- **THEN** support the following backend configuration structure:
  - `backend.url`: Base URL for the backend API (e.g., `http://127.0.0.1:9010`)
  - `backend.token`: Authentication token for backend API access
  - `backend.enabled`: Boolean to enable/disable backend integration (default: false)
- **AND** backend settings participate in the normal cascading merge (project overrides global)
