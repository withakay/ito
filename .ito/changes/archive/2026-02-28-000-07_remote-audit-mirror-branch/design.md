## Context

Local audit logging already exists and should remain the source of truth for reliability and offline support. We want team-visible audit history in remote repositories, but without coupling this high-volume append stream to change-coordination metadata.

This design introduces a dedicated branch for audit mirroring so proposal/task coordination stays focused and low-noise while audit data can still be synchronized for compliance and collaboration.

## Goals / Non-Goals

**Goals:**

- Add optional, dedicated branch mirroring for audit events.
- Keep mirroring best-effort and non-blocking for normal CLI operations.
- Separate branch and config surface from change-coordination behavior.

**Non-Goals:**

- Replacing local audit log storage.
- Guaranteeing exactly-once remote delivery in all failure modes.
- Introducing a centralized lock service.

## Decisions

- **Decision: Use a dedicated audit mirror branch (`ito/internal/audit`) by default.**
  - Rationale: isolates high-frequency audit writes from proposal coordination.
  - Alternative considered: mirror into `ito/internal/changes`. Rejected to avoid churn and coupling.

- **Decision: Mirror is optional and best-effort.**
  - Rationale: CLI commands must not fail due to telemetry/audit transport issues.
  - Alternative considered: hard-fail on mirror errors. Rejected for poor UX and operational fragility.

- **Decision: Keep local JSONL append as primary write path.**
  - Rationale: preserves offline behavior and deterministic local audit trail.
  - Alternative considered: remote-only audit branch writes. Rejected due to availability dependencies.

## Risks / Trade-offs

- **Mirror branch growth** -> Add rotation/compaction strategy in follow-up work.
- **Concurrent mirror writes conflict** -> Retry with fetch/rebase, then surface actionable warning if unresolved.
- **Sensitive metadata concerns** -> Keep existing privacy-preserving event fields; avoid adding raw paths/secrets.

## Migration Plan

1. Add `audit.mirror.enabled` and `audit.mirror.branch` config keys and defaults.
2. Implement mirror sync step after local audit append.
3. Add integration tests for enabled, disabled, offline, and non-fast-forward conflict paths.
4. Roll out disabled by default initially, then consider enabling by default after stability review.

Rollback:

- Set `audit.mirror.enabled=false` to disable remote mirroring without losing local audit behavior.

## Open Questions

- Should mirror trigger on every event append or in periodic batch windows?
- Should branch compaction/rotation be part of this change or a follow-up proposal?
