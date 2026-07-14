## MODIFIED Requirements

### Requirement: Architecture Patterns

The completion implementation SHALL follow clean architecture principles with Rust best practices, supporting multiple shells through a plugin-based pattern.

#### Scenario: Shell-specific generators

- **WHEN** implementing completion generators
- **THEN** implement generator types for each shell (Zsh, Bash, Fish, PowerShell)
- **AND** each generator consumes a shared command registry definition

#### Scenario: Shell-specific installers

- **WHEN** implementing completion installers
- **THEN** implement installer types for each shell (Zsh, Bash, Fish, PowerShell)
- **AND** installers manage shell-specific paths and configuration updates

#### Scenario: Factory pattern for shell selection

- **WHEN** selecting shell-specific implementations
- **THEN** use a factory (or equivalent dispatch) keyed by a `SupportedShell` enum
- **AND** adding a new shell requires updating the enum and dispatch

#### Scenario: Dynamic completion providers

- **WHEN** implementing dynamic completions
- **THEN** encapsulate project discovery logic (change IDs, spec IDs) behind a provider
- **AND** implement caching with a short TTL

#### Scenario: Command registry

- **WHEN** defining completable commands
- **THEN** define a centralized command registry structure consumed by all generators

#### Scenario: Type-safe shell detection

- **WHEN** implementing shell detection
- **THEN** use a finite set of supported shells and validate detection against it
