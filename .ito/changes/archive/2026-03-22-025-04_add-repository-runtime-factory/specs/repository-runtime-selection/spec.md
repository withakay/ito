## ADDED Requirements

### Requirement: Repository implementations are selected centrally at runtime

Ito SHALL resolve the repository implementations for the current persistence mode through a central runtime selection path instead of letting command handlers instantiate concrete repositories directly.

#### Scenario: Filesystem mode selects filesystem repositories

- **GIVEN** filesystem persistence mode is active
- **WHEN** Ito resolves the repository runtime
- **THEN** it returns filesystem-backed repository implementations

#### Scenario: Remote mode selects remote repositories

- **GIVEN** remote persistence mode is active
- **WHEN** Ito resolves the repository runtime
- **THEN** it returns remote-backed repository implementations

#### Scenario: Command handlers do not construct concrete repositories directly

- **WHEN** a CLI command needs change, task, module, or spec data
- **THEN** it obtains the selected repository implementation from the runtime selection path
- **AND** it does not instantiate a concrete filesystem repository in the command handler itself

### Requirement: Remote transport remains behind the repository boundary

The client SHALL treat transport as an implementation detail of remote-backed repositories.

#### Scenario: REST is the current remote implementation

- **GIVEN** remote persistence mode is active
- **WHEN** Ito resolves remote-backed repositories
- **THEN** the current implementation may use REST transport internally
- **AND** command handlers remain transport-agnostic

### Requirement: Direct and server composition reuse the same concrete local repositories

Filesystem-backed and SQLite-backed repository implementations SHALL be shared reusable implementations that can be composed directly in local modes and behind the backend server.

#### Scenario: Filesystem-backed repositories are reused across local and server composition

- **GIVEN** filesystem-backed persistence is selected
- **WHEN** Ito composes repositories for direct/local use and the backend server composes repositories for HTTP-backed use
- **THEN** both compositions use the same concrete filesystem-backed repository implementations
- **AND** only the outer transport/composition layer differs

#### Scenario: SQLite-backed repositories are reused across local and server composition

- **GIVEN** SQLite-backed persistence is selected
- **WHEN** Ito composes repositories for direct/local use and the backend server composes repositories for HTTP-backed use
- **THEN** both compositions use the same concrete SQLite-backed repository implementations
- **AND** only the outer transport/composition layer differs
