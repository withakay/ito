## Why

Module-aware commands still assume local `.ito/modules/` state even though backend mode is supposed to make persistence implementation selectable. Without a backend-backed `ModuleRepository`, module listing and lookup remain filesystem-bound and prevent fully repository-backed command routing.

## What Changes

- Add a remote-backed `ModuleRepository` path for client use while preserving the filesystem implementation.
- Route module-oriented command paths through the selected `ModuleRepository` implementation.
- Ensure module reads work even when local module markdown is absent in remote mode.
- Keep module IDs, summaries, and deterministic ordering stable across implementations.

## Impact

- Affected specs: `module-repository`, `cli-module`
- Affected code: module repository adapters, module-aware CLI handlers, module resolution utilities
- Behavioral change: module reads come from the selected repository implementation instead of assuming local `.ito/modules/`

## Execution Guidance

- Start after `025-04_add-repository-runtime-factory` has settled the repository bundle/factory shape.
- This change can run in parallel with `025-01_wire-change-repository-backends` and `025-02_wire-task-repository-backends`.
- `025-06_improve-agent-backend-workflows` should wait until this change has clarified the supported module-oriented CLI flow.
