## Context

Backend mode introduces a remote system of record for changes, tasks, and artifacts. Existing projects already contain local active changes and archived changes that represent real workflow history. To cut over safely, we need migration to happen as part of `ito init` backend setup, before backend mode is considered usable.

Migration must preserve lifecycle semantics: active changes remain editable, archived changes are imported as immutable history. The import flow must also handle large projects and interruption scenarios without forcing users to restart from scratch.

## Goals / Non-Goals

**Goals:**

- Add an `ito init` flow that lets users choose local or backend storage mode.
- If backend mode is selected and local changes are present, require a migration choice and import execution before enabling backend mode.
- Preserve change lifecycle state during import (active vs archived).
- Make import idempotent and resumable so retries are safe.
- Remove local change artifacts after successful verified import so only backend state remains authoritative.
- Produce clear migration output (imported/skipped/failed counts) to support cutover decisions.

**Non-Goals:**

- Bi-directional merge conflict resolution between divergent local and remote histories.
- Ongoing continuous replication from filesystem to backend after cutover.
- Local cleanup that bypasses parity validation or backup safeguards.

## Decisions

- Decision: Gate backend-mode initialization inside `ito init` behind explicit storage-mode selection.
  - Rationale: makes cutover intent explicit and prevents accidental backend activation with incomplete migration.
  - Alternative considered: infer backend mode automatically from config defaults. Rejected because users can unknowingly end up in mixed local/remote state.

- Decision: During backend-mode init, prompt for import when local changes are detected, and block backend mode on "No".
  - Rationale: backend mode must not start with missing historical artifacts, and user acknowledgment is required for migration.
  - Alternative considered: allow backend mode without import and warn. Rejected because this creates a split-brain workflow.

- Decision: Define explicit non-interactive flags for init migration policy.
  - Rationale: CI and scripted setup need deterministic behavior without prompts.
  - Selected behavior: `--backend`/`--local` choose storage mode; `--import-local-changes`/`--no-import-local-changes` choose import policy; `--backend --yes` with existing local changes fails unless one import-policy flag is provided.
  - Alternative considered: infer import policy from `--yes`. Rejected because destructive cleanup must require explicit intent.

- Decision: Treat import as an idempotent upsert per change/artifact revision.
  - Rationale: allows safe retries after network or process failures and simplifies large-project migration.
  - Alternative considered: fail-fast on any pre-existing backend change. Rejected because partial imports become operationally fragile.

- Decision: Import archived changes with backend archived status from the start.
  - Rationale: preserves lifecycle semantics and prevents archived items from being reallocated or mutated after migration.
  - Alternative considered: import everything as active and archive later. Rejected because it creates a temporary correctness gap and extra operator steps.

- Decision: Expose import through `ito backend import` and invoke the same orchestration from `ito init`.
  - Rationale: keeps migration logic reusable while making init the default cutover path.
  - Alternative considered: implement import only inside init. Rejected because manual reruns/recovery need a direct command.

- Decision: Provide `--dry-run` preview output before stateful migration.
  - Rationale: reduces cutover risk by letting teams verify scope and expected counts first.
  - Alternative considered: no preview mode. Rejected because migration blast radius is harder to assess.

- Decision: Remove local `.ito/changes/` artifacts after verified import success.
  - Rationale: avoids dual-state confusion and enforces backend as the only active source of truth.
  - Alternative considered: keep local copies indefinitely. Rejected because stale local files can mislead users and tools.

## Risks / Trade-offs

- [Destructive local cleanup removes migration source too early] -> Cleanup only after parity checks pass and keep an out-of-repo backup snapshot.
- [Divergent local and backend copies for the same change] -> Report per-change conflict details and require explicit user follow-up before forcing overwrite.
- [Long-running imports fail midway] -> Use resumable idempotent operations and summary checkpoints so reruns continue safely.
- [Archived state imported incorrectly] -> Validate lifecycle mapping in integration tests for active and archived source paths.
- [Large migrations create noisy output] -> Provide concise summary plus optional verbose per-change detail.

## Migration Plan

1. Add core import orchestration that scans local active/archived change sets and prepares upload batches.
2. Add backend client operations for idempotent change/artifact upsert with lifecycle status mapping.
3. Wire `ito backend import` in CLI with `--dry-run` and migration summary output.
4. Wire `ito init` storage-mode selection and import gating prompt; block backend mode when import is declined.
5. Add parity validation and local artifact cleanup flow (plus backup snapshot) after successful import.
6. Add integration tests covering init prompt paths, active+archived import, rerun idempotency, partial-failure resume, and cleanup behavior.
7. Document cutover sequence: choose backend in init, import, verify parity, cleanup local artifacts, then run backend-first workflow.

## Open Questions

- Should cleanup remove local artifacts immediately or require an explicit post-import confirmation flag in non-interactive environments?
- Should archived imports preserve original archive timestamps as first-class backend metadata, or only archived status?
