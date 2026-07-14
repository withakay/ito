## ADDED Requirements

### Requirement: Serve Ito artifacts locally

The CLI SHALL provide a local web server for browsing Ito artifacts and project documentation.

#### Scenario: Start server with defaults
- **WHEN** the user runs `ito serve start` with no flags and no project overrides
- **THEN** the server listens on `127.0.0.1:9009`
- **AND** the command prints a URL to open in a browser

#### Scenario: Configurable bind and port
- **GIVEN** `.ito/config.json` sets `serve.bind` and/or `serve.port`
- **WHEN** the user runs `ito serve start`
- **THEN** the server uses the configured values

#### Scenario: Port fallback when in use
- **GIVEN** the configured/default port is already bound
- **WHEN** the user runs `ito serve start`
- **THEN** the CLI selects the next available port by incrementing (e.g. 9009, 9010, 9011...)
- **AND** prints the final chosen URL

### Requirement: Dependency checks

The CLI MUST check required external dependencies before starting the server.

#### Scenario: Caddy not installed
- **WHEN** the user runs `ito serve start` and `caddy` is not available on PATH
- **THEN** the CLI prints an actionable install hint
- **AND** exits with code 1

### Requirement: Served content scope

The server SHALL only expose a curated set of project paths needed for Ito browsing.

#### Scenario: Allowed directories are accessible
- **WHEN** the server is running
- **THEN** the browser UI can load Markdown from `.ito/changes/`, `.ito/specs/`, `.ito/modules/`
- **AND** also from `.ito/planning/`, `.ito/research/`, `docs/`, and `documents/` if those directories exist

### Requirement: Server lifecycle management

The CLI SHALL provide a way to start and stop a running server for the current project.

#### Scenario: Start server explicitly
- **WHEN** the user runs `ito serve start`
- **THEN** the server process is started (or reused if already running)
- **AND** the CLI prints the running URL

#### Scenario: Stop server
- **GIVEN** the server is running for the current project
- **WHEN** the user runs `ito serve stop`
- **THEN** the server process is terminated
- **AND** the CLI prints a confirmation

#### Scenario: Stop when not running
- **GIVEN** the server is not running
- **WHEN** the user runs `ito serve stop`
- **THEN** the CLI prints that no server is running
- **AND** exits with code 0

### Requirement: Tokenized URL for non-loopback binding

The CLI SHALL support a tokenized URL for non-loopback bindings to reduce casual exposure.

#### Scenario: Bind to 0.0.0.0 with token
- **WHEN** the user configures `serve.bind` to a non-loopback address
- **THEN** the CLI includes a token in the printed URL using a path prefix (e.g. `/t/<token>/`)
- **AND** the server requires the token for requests
