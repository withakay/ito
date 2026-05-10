## Context

Backend mode makes the backend canonical for active change artifacts, but Ito's long-term truth still lives in git: canonical specs under `.ito/specs/` and archived change proposals under `.ito/changes/archive/`. Without an explicit archive sync workflow, teams risk losing the evidence trail (final proposal/tasks/deltas) if the backend disappears.

This change wires `ito archive` so that archiving produces two durable signals:

1. Backend state is updated to show the change is archived and therefore immutable.
2. The repo contains an immutable archived change folder plus updated canonical specs suitable for committing.

## Goals / Non-Goals

**Goals:**

- Archive marks backend change lifecycle as archived.
- Archive materializes the final backend artifact bundle into the repo before moving to `.ito/changes/archive/...`.
- Archive leaves the repo in a state where the user can commit the archived change + updated specs.

**Non-Goals:**

- Forcing git commits automatically.
- Server-side git integration.
- Archive of partially-complete changes.

## Decisions

- Decision: Backend-mode archive pulls from backend as the first step.
  - Rationale: ensures local archive content matches the canonical backend state.

- Decision: Backend-mode archive marks backend archived only after local archive succeeds.
  - Rationale: prevents backend from freezing a change that failed to archive locally.

- Decision: Archived changes are treated as immutable by backend and clients.
  - Rationale: aligns with the expectation that archived changes are final, and protects long-term history.

## Risks / Trade-offs

- [Backend unavailable at archive time] -> Fail with actionable guidance; do not partially mark archived.
- [Local repo diverges from backend] -> Always pull final bundle before archiving.
- [Users forget to commit] -> Print explicit post-archive reminder listing paths to commit.

## Migration Plan

1. Implement a backend-aware archive orchestration service in `ito-core`.
2. Update `ito-cli` archive command to call the service when backend mode is enabled.
3. Add integration tests for backend archive path (happy path + backend unavailable).
