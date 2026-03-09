## Why

Task workflows still assume local `tasks.md` editing even when backend-backed task state is supposed to exist. That keeps backend mode as a best-effort sync path instead of a real persistence mode and encourages agents to mutate markdown directly.

## What Changes

- Extend task persistence so task reads and mutations flow through a runtime-selected task persistence path.
- Keep filesystem-backed task behavior intact while making remote mode repository-backed instead of markdown-edit-first.
- Replace command-path assumptions that task mutations must edit local `tasks.md` before any backend synchronization happens.
- Preserve deterministic task output while surfacing conflicts or unsupported local-edit assumptions clearly.

## Impact

- Affected specs: `task-repository`, `cli-tasks`
- Affected code: task repository contracts/adapters, task mutation orchestration, CLI task handlers
- Behavioral change: remote mode task mutations use the selected persistence implementation instead of direct local markdown edits

## Execution Guidance

- Start after `025-04_add-repository-runtime-factory` has settled the repository bundle/factory shape.
- This change can run in parallel with `025-01_wire-change-repository-backends` and `025-03_wire-module-repository-backends`.
- `025-06_improve-agent-backend-workflows` should wait until this change has clarified the supported CLI mutation flow.
