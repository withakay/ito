## ADDED Requirements

### Requirement: Ito writes structured execution logs to a central location

Ito SHALL record structured execution events to a per-user central log directory.

#### Scenario: Logs are written for a successful command

- **WHEN** a user runs a supported Ito CLI entrypoint
- **THEN** Ito appends structured JSONL execution events to the central log directory
- **AND** events are stored under a versioned path (e.g. `<config_dir>/ito/logs/execution/v1/`)
- **AND** events are grouped by `project_id` and `session_id` (e.g. `projects/<project_id>/sessions/<session_id>.jsonl`)
- **AND** the event includes at least: `timestamp`, `command_id`, `session_id`, `project_id`, and `outcome`

### Requirement: Logging is best-effort and must not break commands

Ito MUST NOT fail a command solely because execution logging failed.

#### Scenario: Log directory is not writable

- **WHEN** Ito cannot create or write to the log directory
- **THEN** the command continues to run
- **AND** Ito exits with the same outcome it would have produced without logging

### Requirement: Project grouping does not record raw paths by default

Ito MUST NOT record the full absolute working directory path in execution logs by default.

#### Scenario: Project id is privacy-preserving

- **WHEN** Ito records an execution event
- **THEN** it stores a derived `project_id` for grouping
- **AND** `project_id` is computed from the project path using a per-user secret salt
- **AND** the raw absolute path is not recorded

### Requirement: Session identity is stable within a project session

Ito SHALL provide a `session_id` that remains stable across multiple commands within the same project session.

#### Scenario: Session id is reused for subsequent commands

- **WHEN** a user runs multiple Ito commands within the same project and session
- **THEN** Ito records the same `session_id` for each event
- **AND** a new session id is created when a new session begins
