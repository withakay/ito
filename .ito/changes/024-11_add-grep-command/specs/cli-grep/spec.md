## ADDED Requirements

### Requirement: CLI provides an ito grep command

The system SHALL provide an `ito grep` command that searches Ito change artifacts using a regular expression.

The command MUST work consistently whether Ito artifacts are available locally on disk (`.ito/`) or are backed by a remote backend with a local cache.

#### Scenario: Grep a single change by ID

- **GIVEN** a change with ID `005-01_add-auth` exists
- **WHEN** the user runs `ito grep 005-01_add-auth "Requirement:"`
- **THEN** the system prints matching lines from that change’s artifacts

### Requirement: Grep supports scope selection for change, module, and project

The `ito grep` command SHALL support searching within different scopes:

- a single change
- a single module (across all changes in that module)
- the entire project (across all changes)

#### Scenario: Grep is scoped to a module

- **GIVEN** module `024` has multiple changes
- **WHEN** the user runs `ito grep --module 024 "Backend"`
- **THEN** the system searches artifacts for all changes in module `024`

#### Scenario: Grep is scoped to the entire project

- **WHEN** the user runs `ito grep --all "Scenario:"`
- **THEN** the system searches artifacts across all changes in the project

### Requirement: Grep output is line-oriented and includes file locations

The grep output MUST be line-oriented and MUST include a stable file location prefix so agents can navigate results.

#### Scenario: Output includes path and line number

- **WHEN** the user runs `ito grep <target> "foo"`
- **THEN** each match line begins with `<path>:<line>:`

### Requirement: Grep supports limiting output

The `ito grep` command SHALL support limiting output to avoid overwhelming agents.

#### Scenario: Limit returned matches

- **WHEN** the user runs `ito grep <target> "foo" --limit 20`
- **THEN** the system prints at most 20 matching lines

### Requirement: Backend mode uses local cache and conditional requests

When backend client mode is enabled, `ito grep` MUST use an on-disk local cache and MUST avoid downloading unchanged artifacts by using conditional HTTP requests (`ETag` / `If-None-Match`).

#### Scenario: Cached artifacts are reused when unchanged

- **GIVEN** backend mode is enabled
- **AND** cached artifacts exist locally for a change
- **WHEN** the user runs `ito grep <change-id> "foo"`
- **THEN** the client revalidates cached artifacts using conditional requests
- **AND** does not re-download artifact bodies when the server returns `304 Not Modified`
