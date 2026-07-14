## Why

Audit events and change-coordination state have different write patterns, retention requirements, and failure tolerances. If both share one branch, high-frequency event writes can add churn and conflict risk to proposal coordination.

## What Changes

- Add an optional remote audit mirror workflow that syncs audit events to a dedicated internal branch.
- Keep audit mirroring independent from change-coordination branch operations and failure handling.
- Add configuration for enabling audit mirroring and overriding the mirror branch name.
- Define best-effort behavior so audit mirror failures do not block core CLI workflows.

## Capabilities

### New Capabilities

- `audit-remote-mirroring`: Optional synchronization of local audit events to a dedicated internal remote branch.

### Modified Capabilities

- `config`: Add configuration keys for audit mirroring enablement and branch naming.

## Impact

- **Config and defaults**: New `audit.mirror.*` keys with safe defaults.
- **Audit pipeline**: Mirror step added after local event append and validation.
- **Git integration**: Dedicated sync path for audit branch (separate from coordination branch).
