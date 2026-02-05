## ADDED Requirements

### Requirement: Rust init matches TypeScript init interaction model

`itors init` SHALL follow the same interaction model as the TypeScript CLI `ito init` as defined by the `cli-init` capability, specifically:

- If `--tools` is not provided and the command is running interactively, `itors init` SHALL prompt the user to select tools.
- If `--tools` is provided, `itors init` SHALL run non-interactively and MUST NOT prompt.

#### Scenario: Interactive selection when tools not provided

- **WHEN** the user runs `itors init` in an interactive session without `--tools`
- **THEN** `itors` prompts for which tools to configure and installs only the selected tools

#### Scenario: Non-interactive init when tools are provided

- **WHEN** the user runs `itors init --tools all`
- **THEN** `itors` configures all supported tools without prompting

### Requirement: Rust init supports the same --tools values and validation

`itors init` SHALL accept the same `--tools` values and validation rules as the TypeScript CLI:

- `all`
- `none`
- a comma-separated list of tool IDs

`itors init` MUST fail with a clear error message when `--tools` is provided but empty, or when any tool ID is unknown.

#### Scenario: Empty --tools value is rejected

- **WHEN** the user runs `itors init --tools ""`
- **THEN** the command fails with an error describing valid `--tools` values

#### Scenario: Unknown tool ID is rejected

- **WHEN** the user runs `itors init --tools "not-a-tool"`
- **THEN** the command fails with an error naming the unknown ID and listing available tool IDs

### Requirement: Rust init supports fresh and extend modes

`itors init` SHALL support both:

- **Fresh init**: `.ito/` does not exist yet.
- **Extend mode**: `.ito/` exists and additional tools can be configured without reinitializing everything.

#### Scenario: Extend mode keeps existing tools configured

- **WHEN** `.ito/` already exists and the user runs `itors init` (interactive) and selects additional tools
- **THEN** already-configured tools remain configured and only the newly selected tools are added/updated
