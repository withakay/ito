## MODIFIED Requirements

### Requirement: Backend supports Homebrew-managed service runtime

The project SHALL provide a Homebrew service workflow that runs the Ito backend API as a managed service process on supported macOS hosts.

The Homebrew service command MUST start the backend via `ito serve-api --service`.

The `serve-api --service` command SHALL silently initialize backend auth in the global config file when auth is missing, then continue startup without printing tokens.

#### Scenario: Start backend service via Homebrew

- **WHEN** a developer executes `brew services start ito`
- **THEN** Homebrew starts the backend service successfully
- **AND** the backend API becomes reachable on the documented endpoint

#### Scenario: First service start bootstraps missing auth

- **GIVEN** `backendServer.auth` is absent from the global config file
- **WHEN** the Homebrew service starts `ito serve-api --service`
- **THEN** the CLI generates and persists backend auth tokens
- **AND** starts the backend without printing the generated tokens

#### Scenario: Service start reuses existing auth

- **GIVEN** `backendServer.auth` already contains a non-empty admin token
- **WHEN** the Homebrew service starts `ito serve-api --service`
- **THEN** the CLI leaves the existing auth config unchanged
- **AND** starts the backend successfully

#### Scenario: Service start fails on malformed config

- **GIVEN** the global config file exists but `backendServer` or `backendServer.auth` is malformed
- **WHEN** the Homebrew service starts `ito serve-api --service`
- **THEN** the CLI exits with an error describing the invalid config
- **AND** does not start the backend with partial auth state
