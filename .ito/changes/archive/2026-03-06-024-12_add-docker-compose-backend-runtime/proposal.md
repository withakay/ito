<!-- ITO:START -->
## Why

The backend API is available in source, but there is no standard one-command runtime for local testing. Contributors currently need ad-hoc setup to run the backend while validating backend-enabled workflows, which slows iteration and creates inconsistent environments.

## What Changes

- Add a Docker Compose runtime definition for the Ito backend so developers can start and stop a local backend consistently during testing.
- Define the expected local runtime workflow (compose up/down, health verification, and required environment inputs).
- Keep Homebrew service and systemd service integration explicitly out of scope for this change, with follow-up work tracked separately.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `backend-state-api`: Add requirements for a supported Docker Compose-based local runtime path for backend testing.

## Impact

- Affected code: backend runtime/developer operations assets (Compose manifest and related docs/scripts).
- Affected systems: local developer backend test workflow.
- Dependencies: Docker Engine with Docker Compose plugin.
<!-- ITO:END -->
