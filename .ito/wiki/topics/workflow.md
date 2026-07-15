# Workflow

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-14
source_refs:
- .ito/specs/cli-change/spec.md
- .ito/specs/cli-tasks/spec.md
- .ito/specs/cli-archive/spec.md
- .ito/specs/requirement-traceability/spec.md
- .ito/specs/archive-completion-validation/spec.md
- .ito/specs/main-first-implementation/spec.md
- .ito/specs/ito-authority-cutover/spec.md
- .ito/changes/archive/2026-07-14-031-06_migrate-ito-authority-and-release/proposal.md
known_gaps: []
```

Ito work is organized around proposed changes with tasks, spec deltas, and
validation. Proposals describe intent and impact, tasks provide ordered
implementation checkpoints, and specs capture accepted behavior through
requirements and scenarios.

## Lifecycle

- Create or update a proposal package under `.ito/changes/<change-id>/`.
- Review and integrate the proposal package into main before implementation;
  use pull-request authority by default or explicitly configure direct merge.
- Create implementation work from the captured main authority and require the
  execute preflight before mutating tasks or running iterations.
- Track work through `tasks.md`; complete tasks only after their verification
commands or review criteria are satisfied.
- Promote accepted behavior into `.ito/specs/` during archive.
- Keep archive follow-through tied to specs, modules, research, demos, and
  workflow documentation rather than treating archive as file movement only.

## Traceability

Requirement IDs and task references are used to connect implementation work to
spec intent. When a change adds or modifies behavior, prefer explicit
requirement IDs and scenario coverage that future validation can inspect.
