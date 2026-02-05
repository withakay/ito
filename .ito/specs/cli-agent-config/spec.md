# CLI Agent Config Specification

## Purpose

The `ito agent-config` command group provides agent configuration management capabilities, enabling teams to configure per-tool model selection, context budgets, and agent preferences for AI coding assistants.

## Requirements

### Requirement: Agent config file location

The CLI SHALL store project agent configuration in `<itoDir>/config.json`.

Notes:

- `<itoDir>` is the project Ito working directory (default: `.ito/`, but it can be renamed via `projectPath`).
- The CLI MUST NOT hardcode `.ito/` in behavior or messaging.


#### Scenario: Agent config file path is deterministic

- **WHEN** the CLI needs the project agent configuration path
- **THEN** it uses `<itoDir>/config.json`

### Requirement: Agent configuration initialization

The CLI SHALL initialize `<itoDir>/config.json` with default configuration for AI tools.

#### Scenario: Initialize agent configuration

- **WHEN** executing `ito agent-config init`
- **THEN** create `<itoDir>/config.json` if it does not exist
- **AND** populate the file with default configuration structure:
  - `tools`: Dictionary of tool-specific settings
  - `agents`: Dictionary of agent-specific settings
  - `defaults`: Global default settings
- **AND** include default entries for supported tools with placeholder values
- **AND** display a success message indicating the configuration has been initialized
- **AND** skip creating the file if it already exists to preserve existing configuration
- **AND** print a hint to run `ito agent-config summary` to view current configuration

### Requirement: Configuration summary display

The CLI SHALL display a summary of the current agent configuration, including tools, agents, and defaults.

#### Scenario: Show configuration summary

- **WHEN** executing `ito agent-config summary`
- **THEN** read `<itoDir>/config.json` if it exists
- **AND** display a formatted summary showing:
  - Configured tools and their default models
  - Configured agents and their preferences
  - Global default settings
  - Context budget settings
- **AND** group settings by category (tools, agents, defaults) for readability
- **AND** display a message indicating no configuration exists if config.json is missing
- **AND** suggest running `ito agent-config init` to create configuration

### Requirement: Configuration value retrieval

The CLI SHALL retrieve specific configuration values by path and display them in a human-readable format.

#### Scenario: Get a configuration value

- **WHEN** executing `ito agent-config get <path>`
- **THEN** parse the path to identify the configuration location (e.g., `tools.opencode.default_model`)
- **AND** read `<itoDir>/config.json`
- **AND** traverse the configuration structure to find the value at the specified path
- **AND** display the value in a human-readable format
- **AND** display an error if the path does not exist
- **AND** suggest using `ito agent-config summary` to see available paths

#### Scenario: Get nested configuration value

- **WHEN** executing `ito agent-config get <path>` with nested keys
- **THEN** handle nested path resolution (e.g., `tools.opencode.context.budget`)
- **AND** display the nested value in a structured format
- **AND** display an error if any intermediate path component does not exist

### Requirement: Configuration value modification

The CLI SHALL set specific configuration values by path, updating `<itoDir>/config.json` atomically.

#### Scenario: Set a configuration value

- **WHEN** executing `ito agent-config set <path> <value>`
- **THEN** parse the path to identify the configuration location
- **THEN** parse the value as the appropriate type (string, number, boolean, or nested structure)
- **AND** read `<itoDir>/config.json`
- **AND** update or create the configuration at the specified path with the new value
- **AND** create intermediate dictionary keys if they do not exist
- **AND** write the updated configuration back to `<itoDir>/config.json` atomically
- **AND** display a confirmation that the configuration has been updated
- **AND** display an error if the value cannot be parsed
- **AND** suggest valid value types (string, number, boolean, JSON)

#### Scenario: Set nested configuration value

- **WHEN** executing `ito agent-config set <path> <value>` with nested keys
- **THEN** create intermediate dictionary structures as needed
- **AND** preserve existing sibling keys at each level
- **AND** validate that the value type is appropriate for the configuration key

#### Scenario: Set boolean configuration value

- **WHEN** executing `ito agent-config set <path> true|false`
- **THEN** parse the value as a boolean (case-insensitive)
- **AND** accept "true", "false", "1", "0", "yes", "no" as valid boolean inputs

#### Scenario: Set numeric configuration value

- **WHEN** executing `ito agent-config set <path> <number>`
- **THEN** parse the value as a number (integer or float)
- **AND** validate that the number is within valid range if the key has constraints

### Requirement: Configuration validation

The CLI SHALL validate configuration values and provide clear error messages for invalid settings.

#### Scenario: Validate tool model preference

- **WHEN** setting `tools.<tool>.default_model`
- **THEN** validate that the tool ID is in the supported tools list
- **AND** validate that the model ID is a valid model for the specified tool
- **AND** display an error with a list of valid models if validation fails

#### Scenario: Validate agent preference

- **WHEN** setting `agents.<agent>.preference`
- **THEN** validate that the preference value is valid (e.g., "default", "fast", "powerful")
- **AND** display an error with a list of valid preferences if validation fails

#### Scenario: Validate context budget

- **WHEN** setting context budget values
- **THEN** validate that the value is a positive number
- **AND** validate that the value is within reasonable limits (e.g., between 1000 and 200000 tokens)
- **AND** display an error with suggested range if validation fails

### Requirement: Configuration template quality

The CLI SHALL generate a high-quality configuration template with sensible defaults and clear structure.

#### Scenario: Configuration template includes all sections

- **WHEN** generating `<itoDir>/config.json`
- **THEN** include all three top-level sections: `tools`, `agents`, `defaults`
- **AND** include example entries for each supported tool under `tools` using placeholder values
- **AND** include example entries for each agent type under `agents` using placeholder values
- **AND** include global defaults for context budget and model preference under `defaults`
- **AND** set reasonable default values (e.g., context budget of 100000 tokens)

#### Scenario: Tool configuration includes required keys

- **WHEN** generating tool configuration templates
- **THEN** include keys for: `default_model`, `context_budget`, `capabilities`
- **AND** provide placeholder values that can be customized
- **AND** list valid model options in comments

#### Scenario: Agent configuration includes required keys

- **WHEN** generating agent configuration templates
- **THEN** include keys for: `model_preference`, `context_strategy`, `capabilities`
- **AND** provide placeholder values that can be customized
- **AND** list valid preference options in comments

### Requirement: Configuration schema

The CLI SHALL support a well-defined configuration schema that allows for tool-specific and agent-specific settings.

#### Scenario: Configuration schema supports tools

- **WHEN** reading or writing configuration
- **THEN** support the following tool configuration structure:
  - `tools.<tool-id>`: Tool-specific settings
    - `default_model`: Default model ID for this tool
    - `context_budget`: Maximum context budget in tokens
    - `capabilities`: List of enabled capabilities
- **AND** accept any tool ID, but validate when available

#### Scenario: Configuration schema supports agents

- **WHEN** reading or writing configuration
- **THEN** support the following agent configuration structure:
  - `agents.<agent-id>`: Agent-specific settings
    - `model_preference`: Preferred model selection strategy ("default", "fast", "powerful")
    - `context_strategy`: How to manage context ("eager", "conservative", "adaptive")
    - `capabilities`: List of enabled capabilities

#### Scenario: Configuration schema supports defaults

- **WHEN** reading or writing configuration
- **THEN** support the following default settings:
  - `defaults.default_model`: Fallback model for tools without specific settings
  - `defaults.context_budget`: Fallback context budget
  - `defaults.model_preference`: Fallback model preference

### Requirement: Error handling

The CLI SHALL provide clear error messages and recovery suggestions when agent-config commands encounter issues.

#### Scenario: Config file cannot be read

- **WHEN** `<itoDir>/config.json` cannot be read due to permissions or malformed JSON
- **THEN** display an error message explaining the failure
- **AND** suggest checking file permissions or running `ito agent-config init` to recreate
- **AND** exit with code 1

#### Scenario: Config file cannot be written

- **WHEN** `<itoDir>/config.json` cannot be written due to permissions or filesystem errors
- **THEN** display an error message explaining the failure
- **AND** suggest checking directory permissions and disk space
- **AND** exit with code 1

#### Scenario: Invalid configuration path

- **WHEN** executing `ito agent-config get <path>` or `set <path> <value>` with an invalid path
- **THEN** display an error message explaining the path is invalid
- **AND** suggest valid path formats (e.g., `tools.opencode.default_model`)
- **AND** exit with code 2

#### Scenario: Configuration file has syntax errors

- **WHEN** reading `<itoDir>/config.json` and it contains JSON syntax errors
- **THEN** display an error message indicating the specific parse error
- **AND** suggest correcting the JSON or running `ito agent-config init` to recreate
- **AND** exit with code 1

### Requirement: Configuration integration

The CLI SHALL integrate agent configuration with other Ito commands and workflows.

#### Scenario: Use configuration in other commands

- **WHEN** executing other Ito commands that interact with AI tools
- **THEN** read `<itoDir>/config.json` to retrieve tool-specific settings
- **AND** use the configured default_model when generating instructions
- **AND** respect the context_budget setting when managing context
- **AND** apply agent preferences when available

#### Scenario: Use configuration in workflows

- **WHEN** executing `ito workflow run` with a specified tool
- **THEN** read tool-specific configuration from `<itoDir>/config.json`
- **AND** apply the configuration to the generated instructions
- **AND** use the default_model for the specified tool if configured
- **AND** respect context_budget settings in the workflow execution

## Why

Agent configuration management enables teams to customize AI tool behavior, optimize for specific use cases, and maintain consistent settings across the project. These commands provide:

1. **Per-tool settings**: Configure different models and settings for each AI tool (OpenCode, Claude Code, Codex, etc.)
1. **Agent preferences**: Define how agents should select models and manage context
1. **Context optimization**: Set appropriate context budgets for different workflows
1. **Team consistency**: Share configuration through version control
1. **Flexibility**: Override defaults for specific tools or agents without affecting others

Without these tools, teams must manage configuration in ad-hoc ways, leading to inconsistent behavior, wasted tokens, and difficulty collaborating.
