## Context

The backend now stores canonical change, task, spec, and archive state, but migration into that state still depends on ad hoc flows and archived implementation history. A direct `ito backend import` command is needed so teams can import repo-local active and archived changes whenever backend mode is introduced or repaired, without coupling the operation to initialization.

## Goals / Non-Goals

**Goals:**

- Add a direct backend import command for local active and archived changes.
- Preserve lifecycle fidelity, including archived status and imported artifact content.
- Make the import operation idempotent, resumable, and previewable via `--dry-run`.
- Produce enough summary output for users to verify what moved and what still needs attention.

**Non-Goals:**

- Bidirectional synchronization between divergent local and backend state.
- Automatic cleanup of local artifacts after import.
- Merging conflicting edited copies of the same change from both local and backend sources.

## Decisions

- Decision: Reuse the existing backend artifact bundle abstractions instead of creating a second import payload shape.
  - Rationale: keeps import/export semantics aligned and lets repository-backed storage stay the single transport boundary.
  - Alternative considered: bespoke import-only DTOs. Rejected because they would duplicate lifecycle and artifact mapping logic.

- Decision: Import active and archived changes in one command with lifecycle-sensitive summaries.
  - Rationale: backend cutover should preserve full project history, not just active work.
  - Alternative considered: separate active and archived commands. Rejected because operators would need to coordinate two migrations and parity checks.

- Decision: Treat reruns as upserts keyed by change ID and artifact revision.
  - Rationale: network failures and partial migrations must be recoverable without manual cleanup.
  - Alternative considered: fail on any existing backend item. Rejected because it makes resumed imports brittle.

- Decision: Keep cleanup out of scope for this proposal.
  - Rationale: direct import should be safe to run before a user decides how and when to remove local artifacts.
  - Alternative considered: delete imported local artifacts automatically. Rejected because it increases blast radius for a first-class migration command.

## Risks / Trade-offs

- [Local and backend copies diverge before import] -> Report skipped/conflicting items clearly and keep import rerunnable.
- [Large repos make import output noisy] -> Default to concise summaries with optional per-item detail only where needed.
- [Archived items lose lifecycle fidelity] -> Validate archived read parity in repository-level integration tests.
- [Dry-run and real import drift] -> Build both modes on the same discovery/orchestration pipeline and only swap the write step.

## Migration Plan

1. Add core import orchestration that scans local active and archived changes and builds artifact bundles.
2. Add backend write/reconciliation behavior that upserts imported artifacts and preserves archived lifecycle state.
3. Wire `ito backend import` with `--dry-run` and migration summary output.
4. Add integration tests covering active+archived import, idempotent reruns, and backend read parity.
5. Document when to run import before switching teams fully to backend mode.

## Open Questions

- Should import report per-change conflicts as warnings, or fail the overall command once any backend divergence is detected?
- Should a future follow-up add optional local cleanup after import, or keep that entirely in a separate change?
