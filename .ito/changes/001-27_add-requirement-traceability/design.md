<!-- ITO:START -->
## Context

Ito's current workflow already points reviewers toward "coverage of all spec requirements," but that coverage is inferred manually from prose. As changes become larger and enhanced tasks become the preferred execution format, Ito needs a lightweight, machine-checkable way to answer a basic workflow question: which planned tasks cover which change requirements?

This proposal focuses on change-local traceability only. It should improve review and validation for active changes without forcing a migration of archived specs or expanding immediately into test/QC provenance.

## Goals / Non-Goals

**Goals:**

- Make requirement-to-task coverage machine-checkable for active spec-driven changes.
- Keep author overhead low by embedding references in existing spec and task artifacts rather than introducing a separate matrix file.
- Integrate traceability into `ito validate` and `ito agent instruction review` so coverage gaps are visible before implementation proceeds.
- Preserve backward compatibility for existing changes that still use checkbox tracking.

**Non-Goals:**

- Tracking requirements through tests, QC, or archived truth specs in this change.
- Requiring historical active/archive changes to backfill requirement ids.
- Replacing review judgment with fully automatic workflow approval.

## Decisions

### Change-local requirement references

- Delta requirement blocks gain an optional metadata line: `- **Requirement ID**: <id>`.
- Requirement ids are opaque author-supplied tokens and SHOULD be stable within the change; templates SHOULD suggest capability-prefixed ids such as `tasks-tracking:enhanced-requirements`.
- The first iteration keeps ids scoped to the active change package so archiving does not need to rewrite or preserve them in current-truth specs.

### Enhanced tasks carry requirement coverage links

- Enhanced task blocks gain an optional metadata line: `- **Requirements**: <id>, <id>`.
- Tasks without requirement references remain valid so setup/refactor chores can stay unlinked.
- Checkbox tracking remains supported but does not participate in computed traceability in the first iteration.

### Traceability computation and coverage semantics

- Ito computes a change-local traceability summary from declared requirement ids and enhanced task references.
- A requirement counts as covered when at least one non-shelved enhanced task references it.
- Unknown task references and duplicate requirement ids are hard validation failures.
- Uncovered requirement ids produce a warning in default validation and an error in `--strict` mode.

### Review integration before dedicated reporting

- `ito agent instruction review --change <id>` includes a traceability summary when the change declares requirement ids and uses enhanced tasks.
- The summary should highlight covered requirements, uncovered requirements, and unresolved task references so review can focus on real gaps.
- A standalone `ito trace` surface is explicitly deferred; this change should first establish the underlying model and validation behavior.

## Alternatives Considered

- Derive references from requirement headings automatically.
  - Rejected because heading text is easier to rename accidentally and harder to reference from task metadata without brittle normalization rules.
- Add a separate `traceability.md` artifact.
  - Rejected because it creates a second source of truth and pushes authors into maintaining a manual matrix.
- Expand directly to requirement -> task -> test -> QC linkage.
  - Rejected for now because Ito does not yet have a first-class QC artifact and would take on too much surface area in one change.

## Risks / Trade-offs

- [Risk] Authoring traceability metadata adds friction to spec and task writing.
  -> Mitigation: keep references lightweight, add them to templates/instructions, and limit strict failures to cases where explicit traceability is incomplete or invalid.

- [Risk] Supporting both enhanced and checkbox task formats could make rollout inconsistent.
  -> Mitigation: compute traceability only for enhanced tasks and surface clear validation/review messaging when a change cannot provide computed coverage.

- [Risk] Change-local ids may need a future migration if Ito later wants archived or cross-change traceability.
  -> Mitigation: keep ids opaque and stable so a later change can preserve or remap them without changing author intent.

## Migration Plan

- Update templates and instruction artifacts so newly authored spec-driven changes naturally include requirement ids and enhanced task references.
- Keep existing changes valid; traceability enforcement applies only when a change provides the relevant metadata and enhanced tasks structure.
- Revisit a standalone traceability report/command after the underlying model proves useful in review and validation.

## Open Questions

- Should a future change expose the computed traceability graph through `ito show` or a dedicated `ito trace` command?
- Should uncovered requirements become hard errors even in non-strict mode once templates and tasks generation fully adopt requirement references?
<!-- ITO:END -->
