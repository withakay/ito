## ADDED Requirements

### Requirement: Process execution is centralized in core

`ito-core` SHALL provide a single process execution boundary (for example: a `ProcessRunner` component) used by production code that needs to execute external commands.

The process execution boundary SHALL return structured results including exit status and captured stdout/stderr.

`ito-domain` MUST NOT execute external commands.

#### Scenario: ProcessRunner boundary exists

- **WHEN** inspecting `ito-core` public API
- **THEN** it MUST expose a process execution boundary (for example: `ProcessRunner`)

#### Scenario: Domain does not spawn processes

- **WHEN** running `make arch-guardrails`
- **THEN** it MUST fail if `std::process::Command` is referenced under `ito-rs/crates/ito-domain/`
