## ADDED Requirements

### Requirement: Bacon configuration file exists

A `bacon.toml` configuration file SHALL exist in the `ito-rs/` directory with project-specific jobs.

#### Scenario: Running bacon with default job

- **GIVEN** bacon is installed (`cargo install --locked bacon`)
- **WHEN** user runs `bacon` in the `ito-rs/` directory
- **THEN** bacon starts in watch mode with the default check job
- **AND** displays compilation errors/warnings as files change

### Requirement: Standard development jobs configured

The bacon configuration SHALL include jobs for common development workflows.

#### Scenario: Check job (default)

- **WHEN** user runs `bacon` or `bacon check`
- **THEN** bacon runs `cargo check --workspace --all-targets`
- **AND** shows errors and warnings

#### Scenario: Clippy job

- **WHEN** user runs `bacon clippy`
- **THEN** bacon runs `cargo clippy --workspace --all-targets`
- **AND** shows clippy lints in addition to errors/warnings

#### Scenario: Test job

- **WHEN** user runs `bacon test`
- **THEN** bacon runs `cargo test --workspace`
- **AND** shows test failures and compilation errors

#### Scenario: Coverage job (optional)

- **WHEN** user runs `bacon coverage`
- **THEN** bacon runs the coverage command (e.g., `cargo llvm-cov`)
- **AND** shows coverage output

### Requirement: Agent-consumable output

Bacon SHALL support an export mode or configuration that produces structured output suitable for agent consumption.

#### Scenario: Export errors for agent

- **WHEN** bacon runs with `--export-locations` or similar flag
- **THEN** errors are written to a known file location (e.g., `.bacon-locations`)
- **AND** the format is parseable (file:line:column format)

### Requirement: Documentation updated

Development documentation SHALL include bacon setup and usage instructions.

#### Scenario: Developer reads setup docs

- **WHEN** a new developer reads CONTRIBUTING.md or README.md
- **THEN** they find instructions for installing bacon
- **AND** they find instructions for running bacon during development

#### Scenario: Agent reads AGENTS.md

- **WHEN** an AI agent reads AGENTS.md
- **THEN** it finds guidance on how to use bacon output
- **AND** it understands how to parse bacon export format
