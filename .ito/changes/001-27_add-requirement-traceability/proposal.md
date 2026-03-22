<!-- ITO:START -->
## Why

Ito already expects reviewers to confirm that a change's tasks cover its spec requirements, but change packages do not carry a machine-checkable link between delta requirements and planned work. That makes omissions easy to miss during review and weakens Ito's spec-driven workflow right where validation and enhanced task tracking should provide the most confidence.

## What Changes

- Add a lightweight requirement traceability model for active changes so delta requirements can declare stable references and enhanced tasks can point at the requirements they implement.
- Add a first-class `ito trace <change-id>` command that renders a computed traceability summary for a change, including historical trace output for archived change bundles and clear unavailable-state explanations when computed coverage cannot be produced.
- Extend `ito validate` to detect duplicate or unknown requirement references and to surface uncovered requirements, with stricter enforcement in `--strict` mode.
- Surface computed traceability context in peer-review instructions so reviewers can see requirement coverage gaps without manually cross-reading specs and tasks.
- Update authoring templates and guidance so new spec-driven changes can adopt requirement-to-task traceability without introducing a separate trace matrix.
- Keep the first iteration additive and change-package-local: no archived-spec backfill, no current-truth lineage tracing after archive, no test/QC traceability, and no mandatory migration for existing checkbox-based tracking files.

## Capabilities

### New Capabilities

- `requirement-traceability`: defines how change-local requirement references are declared, linked to tasks, and summarized for validation/review.
- `cli-trace`: exposes a user-facing traceability view for a change.

### Modified Capabilities

- `cli-surface`: expose `trace` as part of the supported top-level CLI surface.
- `delta-specs`: allow delta requirement blocks to declare stable requirement reference ids for change-local traceability.
- `tasks-tracking`: allow enhanced task blocks to declare which requirement references they cover.
- `cli-validate`: validate traceability integrity and report uncovered requirements and unresolved references.
- `peer-review-instruction`: include computed traceability context and review prompts based on explicit requirement-to-task links.

## Impact

- Affected code spans delta-spec parsing, enhanced task parsing/mutation, validation/reporting, and agent instruction rendering in `ito-rs`.
- Affected code also spans CLI command wiring/help output for `ito trace`, archived-change loading, and compatibility with schema-selected tracking files introduced by `001-25_tracking-file-support`.
- This change is additive for existing projects and schemas; old changes remain readable while new spec-driven changes gain a clearer review and validation story.
- No external services or new dependencies are required.

## Success Signals

- `ito validate <change-id>` reports duplicate or unknown requirement references as errors and reports uncovered traced requirements as warnings by default / errors in `--strict` mode.
- `ito agent instruction review --change <id>` includes covered, uncovered, and unresolved traceability context for traced changes.
- `ito trace <change-id>` renders a change-package-local summary for traced active changes, renders a historical summary for traced archived changes, and gives a clear unavailable-status explanation for changes that have not opted into computed traceability.
<!-- ITO:END -->
