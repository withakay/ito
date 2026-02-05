## ADDED Requirements

### Requirement: JSON schema for configuration

The system SHALL provide a JSON schema for validating Ito configuration files.

#### Scenario: Schema file location

- **WHEN** looking for the Ito config schema
- **THEN** the schema is available at:
  - Embedded in binary (for offline use)
  - Published at `https://ito.dev/schemas/config.schema.json` (future)
  - Generated locally via `ito config schema`

#### Scenario: Schema covers all config sections

- **WHEN** validating a config file against the schema
- **THEN** the schema includes definitions for:
  - `projectPath`: string
  - `harnesses`: object with harness configurations
  - `harnesses.<harness>.provider`: string or null
  - `harnesses.<harness>.agents`: object with agent tier configurations
  - `cache`: object with cache settings
  - `defaults`: object with testing and other defaults

#### Scenario: Schema includes descriptions

- **WHEN** an editor loads the schema
- **THEN** each property has a `description` field explaining its purpose
- **AND** enum values have descriptions where applicable

#### Scenario: Schema includes defaults

- **WHEN** an editor loads the schema
- **THEN** properties with defaults have `default` values in the schema
- **AND** users can see what value will be used if omitted

### Requirement: Schema generation command

The CLI SHALL provide a command to output the JSON schema.

#### Scenario: Generate schema to stdout

- **WHEN** executing `ito config schema`
- **THEN** output the JSON schema to stdout
- **AND** format as pretty-printed JSON

#### Scenario: Generate schema to file

- **WHEN** executing `ito config schema --output <path>`
- **THEN** write the JSON schema to the specified file
- **AND** create parent directories if needed

### Requirement: Schema reference in config files

Config files SHALL support `$schema` field for editor integration.

#### Scenario: Config file with schema reference

- **WHEN** a config file contains `"$schema": "./path/to/schema.json"`
- **THEN** editors supporting JSON schema provide autocomplete and validation
- **AND** the `$schema` field is ignored during config loading

#### Scenario: Init creates config with schema reference

- **WHEN** running `ito init`
- **THEN** created config files include a `$schema` field pointing to the schema
- **AND** the schema path is relative or uses a URL

### Requirement: Harness agent config schema

The schema SHALL define the structure for harness agent configurations.

#### Scenario: Agent config as string shorthand

- **WHEN** an agent value is a string (e.g., `"ito-quick": "anthropic/claude-haiku-4-5"`)
- **THEN** the schema validates the string as a model ID

#### Scenario: Agent config as object

- **WHEN** an agent value is an object
- **THEN** the schema validates:
  - `model` (required): string
  - `temperature` (optional): number, 0.0-1.0
  - `variant` (optional): string
  - `top_p` (optional): number, 0.0-1.0
  - `steps` (optional): integer
  - `reasoningEffort` (optional): enum ["none", "minimal", "low", "medium", "high", "xhigh"]
  - `textVerbosity` (optional): enum ["low", "medium", "high"]
- **AND** additional properties are allowed (passthrough to provider)

#### Scenario: Harness-specific validation

- **WHEN** validating harness configurations
- **THEN** the schema enforces:
  - `opencode`: provider can be null or any string
  - `claude-code`: provider must be "anthropic" if specified
  - `codex`: provider must be "openai" if specified
  - `github-copilot`: provider must be "github-copilot" if specified
