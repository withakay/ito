<!-- ITO:START -->
## Context

Ordering behavior for ID-bearing lists is currently mixed: some commands sort by ID, some sort by recency or parser order, and some persisted state uses non-deterministic map key ordering. This creates operator confusion and noisy diffs, especially in `.ito/workflows/.state/change-allocations.json` where key order changes are not semantically meaningful but still produce merge conflicts.

This change is cross-cutting across `ito-core` and `ito-cli` list/task/show/create paths, and needs explicit policy decisions so behavior is consistent in both human and JSON output.

## Goals / Non-Goals

**Goals:**

- Define one deterministic ordering rule for ID-bearing lists: ascending canonical ID unless a command explicitly supports alternate ordering.
- Ensure all CLI and JSON list surfaces that expose module/change/task/spec IDs apply deterministic sorting.
- Canonicalize `change-allocations.json` serialization order to eliminate arbitrary key ordering churn.
- Preserve compatibility of allocation state storage without introducing unnecessary migration complexity.

**Non-Goals:**

- Migrating allocation state to JSONL in this change.
- Redesigning task dependency semantics, wave execution semantics, or non-ID ranking algorithms unrelated to list output.
- Reworking every textual section in Markdown files that are not ID-bearing list surfaces.

## Decisions

### Decision: ID-bearing lists MUST sort ascending by canonical ID

- Applies to module IDs (`NNN`), change IDs (`NNN-CC_name`), spec IDs, and task IDs (`wave.task`).
- For commands with alternate sorts (for example `--sort recent`), deterministic ID tie-breakers will still be required.
- Rationale: users requested consistency and lowest cognitive load when scanning lists by ID.

**Alternatives considered**

- Keep mixed sort semantics (status quo): rejected because it preserves inconsistency.
- Keep recency default globally: rejected because it conflicts with explicit requirement for ascending IDs.

### Decision: Keep `change-allocations` as JSON snapshot state

- Continue reading/writing `.ito/workflows/.state/change-allocations.json` as JSON.
- Canonicalize key ordering (module IDs ascending) and use deterministic serialization.
- Rationale: current read/update usage is snapshot-oriented and small; JSONL adds replay/compaction complexity without enough benefit for this change.

**Alternatives considered**

- Migrate to JSONL now: rejected for this change due to migration complexity, backward compatibility overhead, and additional parser/reducer logic.

### Decision: Normalize module change checklist ordering

- When updating `module.md` change checklist entries, ensure IDs are emitted in ascending order.
- Rationale: module changes are frequently touched and append-order drift causes unnecessary merge conflicts and visual inconsistency.

**Alternatives considered**

- Preserve insertion order: rejected because it does not satisfy consistency requirement.

## Risks / Trade-offs

- [Behavioral change in `ito list` default ordering] -> Mitigation: mark as BREAKING in proposal, update specs and tests, and preserve explicit alternate sort flags.
- [Task list ordering may change from execution/file order to ID order in some outputs] -> Mitigation: scope ordering changes to list surfaces and verify `next/start` execution semantics remain dependency-driven.
- [State serialization changes may alter diff shape] -> Mitigation: add deterministic order tests and keep state schema unchanged.

## Migration Plan

1. Update spec deltas for affected capabilities (`cli-list`, `cli-tasks`, `cli-show`, `change-creation`).
2. Implement ordering helpers in core where appropriate and apply them in CLI adapters.
3. Canonicalize allocation state map ordering during write/read update cycles.
4. Update tests for ordering guarantees across human and JSON output.
5. Run strict validation and full checks before merge.

Rollback:

- Revert ordering policy changes in list/task/show adapters and core sorting helpers.
- Revert allocation-state serialization changes while keeping file format JSON-compatible.

## Open Questions

- Should rank-based suggestion lists (for example fuzzy suggestions) remain rank-first with deterministic secondary ID sort, or be forced to ID-first everywhere?
- Should any non-ID list surfaces be explicitly excluded and documented as semantic-order lists?
<!-- ITO:END -->
