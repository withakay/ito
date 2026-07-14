## ADDED Requirements

### Requirement: Capabilities Manifest

Ito SHALL expose a machine-readable capabilities manifest describing the supported CLI surface.

#### Scenario: Emit full manifest

- **WHEN** a user runs `ito capabilities --json`
- **THEN** Ito SHALL emit valid JSON to stdout
- **AND** the JSON SHALL include command paths, summaries, flags, positional arguments, aliases, examples, and JSON-output support.

#### Scenario: Focus command manifest

- **WHEN** a user runs `ito capabilities command tasks --json`
- **THEN** Ito SHALL emit only the `tasks` command tree
- **AND** the response SHALL include all supported tasks subcommands and flags.

### Requirement: Agent Instruction Artifact Discovery

The capabilities manifest SHALL include valid `ito agent instruction` artifact IDs.

#### Scenario: Artifact IDs are discoverable

- **WHEN** a user runs `ito capabilities artifacts --json`
- **THEN** Ito SHALL list supported artifact IDs
- **AND** each artifact SHALL declare required inputs such as `--change`, `--tool`, or no input.

### Requirement: Suggested Replacements

Ito SHALL provide structured suggestions for deprecated, removed, or compatibility commands when a known intent has a preferred surface.

#### Scenario: Deprecated command replacement

- **WHEN** the manifest includes a deprecated command
- **THEN** the command entry SHALL include a replacement command path when one exists
- **AND** the entry SHALL include a short reason.

### Requirement: Stable JSON Schema

The capabilities response SHALL be versioned so prompts and tools can safely consume it.

#### Scenario: Versioned response

- **WHEN** Ito emits capabilities JSON
- **THEN** the root object SHALL include a schema version
- **AND** tests SHALL fail if required fields are removed without updating the schema version.
