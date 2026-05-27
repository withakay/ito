# Workflow

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-05-27
source_refs:
  - docs/ito/specs/cli-change/spec.md
  - docs/ito/specs/cli-tasks/spec.md
  - docs/ito/specs/cli-archive/spec.md
  - docs/ito/specs/requirement-traceability/spec.md
  - docs/ito/specs/archive-completion-validation/spec.md
known_gaps: []
```

Ito work is organized around proposed changes with tasks, spec deltas, and
validation. Proposals describe intent and impact, tasks provide ordered
implementation checkpoints, and specs capture accepted behavior through
requirements and scenarios.

## Lifecycle

- Create or update a change under `docs/ito/changes/active/`.
- Track work through `tasks.md`; complete tasks only after their verification
  commands or review criteria are satisfied.
- Promote accepted behavior into `docs/ito/specs/` during archive.
- Keep archive follow-through tied to specs, modules, research, demos, and
  workflow documentation rather than treating archive as file movement only.

## Traceability

Requirement IDs and task references are used to connect implementation work to
spec intent. When a change adds or modifies behavior, prefer explicit
requirement IDs and scenario coverage that future validation can inspect.
