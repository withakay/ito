# Config Defaults Specification

## Purpose

Define the `config-defaults` capability: a single source of truth for Ito configuration defaults.

## Requirements

### Requirement: Centralized configuration defaults

The system SHALL provide a single source of truth for all Ito configuration defaults.

#### Scenario: Defaults defined in Rust code

- **WHEN** building the ito binary
- **THEN** all default configuration values are defined in `ito-core/src/config/defaults.rs`
- **AND** defaults are organized by section (harnesses, cache, agents, etc.)
- **AND** defaults are type-safe and documented

#### Scenario: Defaults used when config missing

- **WHEN** loading configuration with missing keys
- **THEN** the system uses centralized defaults for missing values
- **AND** partial configs merge with defaults (user values override defaults)

#### Scenario: Defaults exported for schema generation

- **WHEN** generating the JSON schema
- **THEN** default values are included in the schema
- **AND** schema consumers can see what defaults apply

### Requirement: Agent model defaults

The system SHALL provide default agent model configurations for each harness.

#### Scenario: OpenCode agent defaults

- **WHEN** no user configuration exists for OpenCode agents
- **THEN** use these defaults:
  - `ito-quick`: `anthropic/claude-haiku-4-5`, temperature: 0.3
  - `ito-general`: `openai/gpt-5.2-codex`, variant: "high", temperature: 0.3
  - `ito-thinking`: `openai/gpt-5.2-codex`, variant: "xhigh", temperature: 0.5

#### Scenario: Claude Code agent defaults

- **WHEN** no user configuration exists for Claude Code agents
- **THEN** use these defaults:
  - `ito-quick`: `haiku`
  - `ito-general`: `sonnet`
  - `ito-thinking`: `opus`

#### Scenario: Codex agent defaults

- **WHEN** no user configuration exists for Codex agents
- **THEN** use these defaults:
  - `ito-quick`: `openai/gpt-5.1-codex-mini`
  - `ito-general`: `openai/gpt-5.2-codex`, reasoningEffort: "high"
  - `ito-thinking`: `openai/gpt-5.2-codex`, reasoningEffort: "xhigh"

#### Scenario: GitHub Copilot agent defaults

- **WHEN** no user configuration exists for GitHub Copilot agents
- **THEN** use these defaults:
  - `ito-quick`: `github-copilot/claude-haiku-4.5`
  - `ito-general`: `github-copilot/gpt-5.2-codex`
  - `ito-thinking`: `github-copilot/gpt-5.2-codex`

### Requirement: Cache defaults

The system SHALL provide default cache configuration.

#### Scenario: Cache TTL default

- **WHEN** no user configuration exists for cache
- **THEN** use default `ttl_hours`: 24
