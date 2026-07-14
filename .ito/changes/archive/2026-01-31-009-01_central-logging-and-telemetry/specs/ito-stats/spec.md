## ADDED Requirements

### Requirement: Ito provides a local-only stats command

Ito SHALL provide a CLI command that summarizes local usage from execution logs.

#### Scenario: User views command usage

- **WHEN** a user runs `ito stats`
- **THEN** Ito reads local execution logs
- **AND** prints aggregated usage counts grouped by `command_id`

### Requirement: Stats can report unused commands

Ito SHALL be able to report commands with zero observed usage.

#### Scenario: Known commands are enumerated

- **WHEN** `ito stats` renders usage
- **THEN** it includes `command_id` entries for the known CLI entrypoints
- **AND** shows zero counts for commands not present in the logs

### Requirement: Stats are offline and do not require network

`ito stats` MUST operate solely on local data.

#### Scenario: Network unavailable

- **WHEN** a user runs `ito stats` without network connectivity
- **THEN** the command completes successfully (assuming local log access)
