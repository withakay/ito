## ADDED Requirements

### Requirement: macOS/Linux install script installs the correct binary

The project SHALL provide an install script for macOS and Linux that downloads the correct `ito` binary for the caller's OS and architecture.

#### Scenario: User installs via curl

- **WHEN** a user runs the documented `curl | sh` install command on macOS or Linux
- **THEN** the script downloads the correct release asset for that OS/arch
- **AND** installs `ito` into a user-writable bin directory (or a configured destination)

### Requirement: Install script verifies integrity

The install script MUST verify the downloaded artifact against published checksums before installing.

#### Scenario: Checksum verification blocks tampered downloads

- **WHEN** the downloaded artifact checksum does not match the published checksum
- **THEN** the installer aborts with a non-zero exit code
- **AND** it does not install or overwrite the existing `ito` binary

### Requirement: Unsupported platforms fail clearly

The install script MUST fail with a clear error message when run on unsupported platforms.

#### Scenario: User runs installer on Windows

- **WHEN** a user runs the install script on Windows
- **THEN** the script exits non-zero
- **AND** it explains that Windows is not supported by the shell installer
