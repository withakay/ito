<!-- ITO:START -->
## Why

Docker Compose covers local backend bring-up, but contributors and operators still need host-native service management for long-running development and self-hosted environments. Adding Homebrew and systemd service support now provides predictable lifecycle management on common platforms without requiring manual wrapper scripts.

## What Changes

- Add a Homebrew-backed service path for running the Ito backend as a managed local service.
- Add a systemd unit path for running the Ito backend as a managed Linux service.
- Document startup, shutdown, status, and log-discovery workflows for both service managers.
- Define configuration expectations so service-managed backend instances run `ito serve-api` with the intended runtime settings.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `backend-state-api`: Extend runtime/startup requirements to include Homebrew and systemd managed-service entry points for backend operation.

## Impact

- Affected code: service manager assets and packaging/runtime docs for backend operations.
- Affected systems: local macOS (Homebrew) and Linux (systemd) backend runtime workflows.
- Dependencies: Homebrew (macOS) and systemd (Linux) on supported hosts.
<!-- ITO:END -->
