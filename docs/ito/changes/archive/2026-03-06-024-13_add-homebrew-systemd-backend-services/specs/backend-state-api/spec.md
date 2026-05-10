## ADDED Requirements

### Requirement: Backend supports Homebrew-managed service runtime

The project SHALL provide a Homebrew service workflow that runs the Ito backend API as a managed service process on supported macOS hosts.

#### Scenario: Start backend service via Homebrew

- **WHEN** a developer executes the documented Homebrew service start workflow for Ito backend
- **THEN** the backend service starts successfully
- **AND** the backend API becomes reachable on the documented endpoint

#### Scenario: Stop backend service via Homebrew

- **WHEN** a developer executes the documented Homebrew service stop workflow for Ito backend
- **THEN** the backend service stops cleanly

### Requirement: Backend supports systemd-managed service runtime

The project MUST provide a systemd service workflow that runs the Ito backend API as a managed service process on supported Linux hosts.

#### Scenario: Start backend service via systemd

- **WHEN** a developer or operator executes the documented systemd start workflow for Ito backend
- **THEN** the service enters an active state
- **AND** the backend API becomes reachable on the documented endpoint

#### Scenario: Stop backend service via systemd

- **WHEN** a developer or operator executes the documented systemd stop workflow for Ito backend
- **THEN** the backend service stops cleanly

### Requirement: Service manager runtimes expose operational verification steps

Homebrew and systemd backend runtime documentation MUST include status and logs verification commands so users can confirm service health and diagnose startup failures.

#### Scenario: User verifies service health and logs

- **GIVEN** the backend is managed by Homebrew or systemd
- **WHEN** the user runs documented status and logs commands
- **THEN** the commands provide sufficient evidence to determine whether backend startup succeeded
