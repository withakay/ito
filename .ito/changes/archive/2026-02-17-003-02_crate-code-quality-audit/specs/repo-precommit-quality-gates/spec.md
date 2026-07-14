## ADDED Requirements

### Requirement: Repo enforces Rust workspace coverage target

The repository MUST maintain >= 80% overall test coverage for the Rust workspace.

#### Scenario: Workspace coverage report meets target

- **WHEN** a contributor runs `make test-coverage`
- **THEN** the reported overall coverage is >= 80%

### Requirement: Repo limits Rust source file size

The repository MUST keep Rust source files under 1000 lines to encourage modularity and testability.

#### Scenario: Oversized Rust files are detected

- **WHEN** a contributor audits the workspace Rust sources
- **THEN** no Rust source file exceeds 1000 lines, or exceptions are documented with justification
