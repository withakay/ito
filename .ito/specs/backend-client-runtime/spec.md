# Backend Client Runtime

## Purpose

This spec defines the current behavior and requirements for backend client runtime.

## Requirements

### Requirement: Backend runtime provides repository-ready remote clients

When remote persistence mode is active, Ito SHALL resolve backend runtime state that is sufficient to construct remote-backed repository implementations for change, task, module, and spec access.

The same resolved runtime SHALL also be sufficient to construct remote-backed artifact mutation clients for active-work change/spec artifact updates.

#### Scenario: Remote runtime is reused across repository implementations

- **GIVEN** remote persistence mode is active and runtime resolution succeeds
- **WHEN** Ito constructs remote-backed repositories
- **THEN** those repositories share the resolved runtime context instead of performing command-local backend setup independently

#### Scenario: Remote runtime is reused for artifact mutation clients

- **GIVEN** remote persistence mode is active and runtime resolution succeeds
- **WHEN** Ito constructs remote-backed artifact mutation clients
- **THEN** those clients share the same resolved runtime context as the repository readers
- **AND** command handlers do not perform independent backend configuration for artifact mutation operations

### Requirement: Backend runtime is compiled only by explicit feature selection

Backend HTTP clients, remote repositories, backend synchronization, backend event forwarding, backend authentication helpers, server integration, and backend-only command handlers SHALL be compiled only when the backend Cargo feature is enabled. The `ito-backend` crate MUST explicitly enable the corresponding `ito-core` backend feature with core default features disabled.

- **Requirement ID**: backend-client-runtime:explicit-feature-propagation

#### Scenario: Backend crate enables core backend support

- **WHEN** Cargo builds the `ito-backend` crate
- **THEN** its manifest explicitly enables `ito-core`'s backend feature
- **AND** does not depend on an implicit default feature or incidental feature unification from another workspace member

#### Scenario: Default CLI omits backend implementation

- **WHEN** Cargo builds the default `ito-cli` binary
- **THEN** backend implementation modules are not compiled into that binary
- **AND** the optional `ito-backend` crate is not present in the binary's normal dependency graph

### Requirement: Compiled-out backend requests fail explicitly

When parsed configuration or an invoked operation requests backend support from a binary built without the backend feature, Ito MUST return a typed feature-unavailable error. Ito MUST NOT fall back to filesystem persistence, embedded persistence, or another backend.

- **Requirement ID**: backend-client-runtime:compiled-out-error

#### Scenario: Legacy backend configuration reaches a default binary

- **GIVEN** a project configuration contains `backend.enabled: true`
- **AND** the active Ito binary was built without the backend feature
- **WHEN** a stateful command resolves its repository runtime
- **THEN** Ito returns a typed feature-unavailable error identifying `backend`
- **AND** the error identifies the configuration that requested the feature
- **AND** no filesystem mutation occurs through fallback persistence

#### Scenario: Explicit backend command reaches a default binary

- **GIVEN** the active Ito binary was built without the backend feature
- **WHEN** command dispatch requests backend-only behavior through a retained compatibility path
- **THEN** Ito returns the same typed feature-unavailable error contract
- **AND** provides actionable guidance for using an experimental build or migrating configuration
