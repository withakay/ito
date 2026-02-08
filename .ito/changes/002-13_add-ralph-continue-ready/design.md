## Context

Ralph already supports:

- Running against an explicit change (`--change`).
- Running within a module (`--module`) with `--continue-module` to progress through module-ready changes.

This change adds an analogous mode at the repository scope: keep selecting and running the next ready change until no further progress can be made.

## Goals / Non-Goals

- Goals:
  - Provide a single command invocation that drains the ready change queue.
  - Revalidate readiness before each change selection to tolerate task-state drift.
  - Keep selection deterministic.
- Non-Goals:
  - Auto-starting draft changes (missing planning artifacts).
  - Auto-unshelving paused changes.
  - Parallel execution across changes.

## Decisions

- Flag shape:
  - Add `--continue-ready` to `ito ralph`.
  - `--continue-ready` is mutually exclusive with `--change`, `--module`, `--status`, `--add-context`, `--clear-context`.
- Candidate changes:
  - A change is eligible iff its derived `ChangeWorkStatus` is `Ready` or `InProgress`.
  - `Draft`, `Paused`, and `Complete` are not eligible.
- Selection order:
  - Always pick the lowest change ID (lexicographic) among eligible changes.
- Exit behavior:
  - If ready changes exist: continue until none remain.
  - If no ready changes exist and all changes are `Complete`: exit 0.
  - If no ready changes exist but at least one change is not `Complete`: exit non-zero and list remaining non-complete changes grouped by work status.

## Risks / Trade-offs

- “All changes complete” may be unattainable if the repo contains draft/paused changes; the command should treat this as “blocked” and fail clearly rather than looping.

## Migration Plan

- Add the new flag and behavior without changing defaults.
- Extend help text and tests to cover the new mode.

## Open Questions

- (resolved) `InProgress` changes are eligible so `--continue-ready` can resume started work and avoid reporting “blocked” while work is underway.
