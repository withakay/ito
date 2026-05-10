## Why

The backend command surface is now large enough that server lifecycle should live under the `ito backend` namespace instead of a one-off top-level dev command. Promoting `ito backend serve` now keeps backend workflows discoverable and consistent before `serve-api` becomes entrenched in scripts.

## What Changes

- Add `ito backend serve` as the canonical command for starting the multi-tenant backend server.
- Remove `ito serve-api` as a supported entrypoint rather than carrying it forward as a hidden compatibility shim.
- Keep the current serve flags and configuration behavior under the new command path.
- Update docs, help text, completions, QA scripts, and agent guidance to reference `ito backend serve`.

## Capabilities

### New Capabilities

- `backend-server-cli`: Canonical CLI surface for starting and configuring the backend server.

### Modified Capabilities

- `backend-agent-instructions`: Update backend guidance and examples to reference `ito backend serve` instead of `ito serve-api`.

## Impact

- **Affected code**: `ito-cli` command definitions and dispatch, backend QA scripts/tests, docs/templates, and command help/completion snapshots.
- **Affected workflows**: developers and automation will use `ito backend serve` as the only supported command to start the backend server.
- **CLI impact**: top-level `serve-api` is removed before it becomes stable, so backend server lifecycle is grouped under `ito backend` from the start.
