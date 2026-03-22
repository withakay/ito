<!-- ITO:START -->
## Context

Ito's current workflow already points reviewers toward "coverage of all spec requirements," but that coverage is inferred manually from prose. As changes become larger and enhanced tasks become the preferred execution format, Ito needs a lightweight, machine-checkable way to answer a basic workflow question: which planned tasks cover which change requirements?

This proposal focuses on change-package-local traceability. It should improve review and validation for active changes, and allow archived change bundles to be traced in historical mode, without forcing a migration of archived/current-truth specs or expanding immediately into test/QC provenance.

## Goals / Non-Goals

**Goals:**

- Make requirement-to-task coverage machine-checkable for active spec-driven changes.
- Preserve that same traceability view for archived changes when their archived bundles contain the needed metadata.
- Keep author overhead low by embedding references in existing spec and task artifacts rather than introducing a separate matrix file.
- Integrate traceability into `ito validate` and `ito agent instruction review` so coverage gaps are visible before implementation proceeds.
- Expose computed traceability through a dedicated `ito trace <change-id>` command so authors and reviewers can inspect coverage without reading raw artifacts.
- Preserve backward compatibility for existing changes that still use checkbox tracking.

**Non-Goals:**

- Tracking requirements through tests, QC, or archived truth specs in this change.
- Building current-truth lineage from archived/promoted specs back to historical tasks after archive.
- Requiring historical active/archive changes to backfill requirement ids.
- Replacing review judgment with fully automatic workflow approval.

## Decisions

### Change-local requirement references

- Delta requirement blocks gain an optional metadata line: `- **Requirement ID**: <id>`.
- Requirement ids are opaque author-supplied tokens and SHOULD be stable within the change; templates SHOULD suggest capability-prefixed ids such as `tasks-tracking:enhanced-requirements`.
- The first iteration keeps ids scoped to the active change package so archiving does not need to rewrite or preserve them in current-truth specs.
- Archived change bundles keep their existing delta/task artifacts, so the same ids can still drive historical trace output after archive without changing promoted truth specs.
- Once any delta requirement in a change declares a requirement id, every delta requirement in that change must declare one so computed coverage cannot silently ignore untagged requirements.

### Enhanced tasks carry requirement coverage links

- Enhanced task blocks gain an optional metadata line: `- **Requirements**: <id>, <id>`.
- Tasks without requirement references remain valid so setup/refactor chores can stay unlinked.
- Checkbox tracking remains supported but does not participate in computed traceability in the first iteration.
- Traceability computation MUST use the change's active tracking file resolution path rather than assuming `tasks.md`, so it remains compatible with `001-25_tracking-file-support`.

### Traceability computation and coverage semantics

- Ito computes a change-local traceability summary from declared requirement ids and enhanced task references.
- A change is trace-ready when every delta requirement declares a requirement id and the active tracking file uses enhanced task encoding.
- A requirement counts as covered when at least one non-shelved enhanced task references it.
- Unknown task references and duplicate requirement ids are hard validation failures.
- Uncovered requirement ids produce a warning in default validation and an error in `--strict` mode when the change is trace-ready.
- If a change declares requirement ids but is not trace-ready (for example, it still uses checkbox tasks), validation reports that computed coverage is unavailable instead of pretending every requirement is uncovered.

### Dedicated trace command and review integration

- `ito trace <change-id>` reuses the same computed summary used by validation/review and renders a human-readable report.
- `ito trace <change-id>` resolves the change by canonical ID across active and archived lifecycle states and labels archived output as historical.
- `ito trace <change-id> --json` returns machine-readable status, lifecycle state, declared requirements, covered requirements, uncovered requirements, unresolved references, and unavailable reasons.
- `ito agent instruction review --change <id>` includes a traceability summary when the change declares requirement ids and uses enhanced tasks.
- The summary should highlight covered requirements, uncovered requirements, and unresolved task references so review can focus on real gaps.

## Alternatives Considered

- Derive references from requirement headings automatically.
  - Rejected because heading text is easier to rename accidentally and harder to reference from task metadata without brittle normalization rules.
- Add a separate `traceability.md` artifact.
  - Rejected because it creates a second source of truth and pushes authors into maintaining a manual matrix.
- Fold traceability into `ito show` instead of adding a dedicated command.
  - Rejected because traceability is a workflow/debugging surface with different output needs from generic artifact display and deserves a clear user entrypoint.
- Expand directly to requirement -> task -> test -> QC linkage.
  - Rejected for now because Ito does not yet have a first-class QC artifact and would take on too much surface area in one change.

## Risks / Trade-offs

- [Risk] Authoring traceability metadata adds friction to spec and task writing.
  -> Mitigation: keep references lightweight, add them to templates/instructions, and limit strict failures to cases where explicit traceability is incomplete or invalid.

- [Risk] Supporting both enhanced and checkbox task formats could make rollout inconsistent.
  -> Mitigation: compute traceability only for enhanced tasks, expose explicit "trace unavailable" output through validation and `ito trace`, and avoid failing legacy changes that never opted in.

- [Risk] This change could drift from `001-25_tracking-file-support` and accidentally hard-code `tasks.md` assumptions.
  -> Mitigation: require traceability computation to consume the same active tracking file resolution path used by validation and tasks operations.

- [Risk] Change-local ids may need a future migration if Ito later wants archived or cross-change traceability.
  -> Mitigation: keep ids opaque and stable so archived historical trace can use them as-is now and a later change can preserve or remap them for current-truth lineage without changing author intent.

- [Risk] Users may confuse archived historical trace with current-truth lineage after the change is promoted.
  -> Mitigation: label archived `ito trace` output as historical and keep current-truth lineage explicitly out of scope in the command output and docs.

## Migration Plan

- Update templates and instruction artifacts so newly authored spec-driven changes naturally include requirement ids and enhanced task references.
- Keep existing changes valid; traceability enforcement applies only when a change opts into requirement ids or task requirement references, and computed coverage only activates for trace-ready enhanced-task changes.
- Ship `ito trace` in the same change so authors can inspect readiness and unavailable states while adoption is still incremental, including historical output for newly archived traced changes.

## Open Questions

- Should future promoted/current-truth specs preserve requirement ids so `ito trace` can evolve from archived historical mode into cross-change or post-archive lineage reporting?
- Should uncovered requirements become hard errors even in non-strict mode once templates and tasks generation fully adopt requirement references?
<!-- ITO:END -->
