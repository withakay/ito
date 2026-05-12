# Schema Customization

This document describes how Ito resolves workflow schemas and how to export and customize them.

## Resolution Order

When Ito resolves a schema name (for example `spec-driven`), it checks locations in this order:

1. **Project-local override**: `.ito/templates/schemas/<name>/`
2. **User override**: `${XDG_DATA_HOME}/ito/schemas/<name>/` (or `~/.local/share/ito/schemas/<name>/`)
3. **Embedded built-in**: bundled with Ito in `ito-templates/assets/schemas/<name>/`
4. **Legacy package fallback**: package `schemas/<name>/` (compatibility path)

This means a repo can ship a shared project override while each developer can still keep personal overrides.

## Export Built-in Schemas

Use the templates command to write embedded defaults to disk:

```bash
ito templates schemas export -f ".ito/templates/schemas"
```

The export writes one directory per schema, each containing:

- `schema.yaml`
- `validation.yaml` when the built-in schema ships validator configuration
- `templates/*.md`

Example output layout:

```text
.ito/templates/schemas/
  spec-driven/
    schema.yaml
    validation.yaml
    templates/
      proposal.md
      spec.md
      design.md
      tasks.md
  tdd/
    schema.yaml
    templates/
      spec.md
      test.md
      implementation.md
      docs.md
```

## Overwrite Behavior

- Without `--force`, existing files are preserved and reported as skipped.
- With `--force`, existing files are overwritten with embedded defaults.

```bash
# Safe export (no overwrites)
ito templates schemas export -f ".ito/templates/schemas"

# Force overwrite
ito templates schemas export -f ".ito/templates/schemas" --force
```

## Typical Workflow

1. Export built-ins into project-local path.
2. Edit `.ito/templates/schemas/<name>/schema.yaml` and template files.
3. If you want opt-in validation rules, edit `.ito/templates/schemas/<name>/validation.yaml`.
4. Commit project-local schema overrides if they are team conventions.
5. Keep personal-only customizations in `${XDG_DATA_HOME}/ito/schemas/`.

## Validation Rules Extension

Schema validation configs can opt into additional checks without changing validator IDs. Add a `rules:` map under an artifact entry, and use the optional top-level `proposal:` entry when proposal-only checks are needed. Domain-discovery rules can run from either `proposal.rules` or an artifact rule such as `artifacts.specs.rules`, so schemas without `proposal.md` can still validate a `domain-discovery.md` handoff.

```yaml
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: error
      ui_mechanics: warning
      contract_refs: warning
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
    ubiquitous_language_consistency: warning
    context_boundary_consistency: warning
    domain_documentation_consistency: warning
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
  rules:
    task_quality: error
```

Current v1 rule names:

- `scenario_grammar`: require `WHEN`/`THEN`, recommend `GIVEN`, and warn on oversized scenarios
- `ui_mechanics`: warn when non-UI requirements describe click/wait/selector mechanics
- `contract_refs`: validate requirement-level `Contract Refs` syntax and related proposal anchors
- `capabilities_consistency`: compare proposal capability lists against change-local deltas and baseline `.ito/specs/`
- `ubiquitous_language_consistency`: compare rejected aliases from `domain-discovery.md` against proposal, spec, design, and task language
- `context_boundary_consistency`: warn when cross-context discovery omits affected contexts, ownership, relationship framing, or translation boundaries
- `domain_documentation_consistency`: compare proposed `CONTEXT.md`, `CONTEXT-MAP.md`, and ADR term definitions against `domain-discovery.md`
- `task_quality`: enforce enhanced-task quality checks for `Files`, `Action`, `Verify`, `Done When`, `Requirements`, `Status`, and `Updated At`

The domain-language and boundary rules are quiet unless the change includes `domain-discovery.md` with the relevant DDD handoff tables populated. `ubiquitous_language_consistency` reads rejected aliases from `## Rejected Aliases / Overloaded Terms`; `context_boundary_consistency` reads `## Domain Discovery Summary`, `## Bounded Context Map`, and `## Model Ownership`; `domain_documentation_consistency` compares `## Ubiquitous Language` term definitions against proposed domain docs.

Built-in `spec-driven` defaults stay quiet in v1: the shipped schema exports the rule machinery, but it does not enable any of these new rules until you opt in through a project-local `validation.yaml` override.

## Notes

- `$schema` JSON metadata in config files is unrelated to workflow schema resolution.
- If a schema cannot be found, run `ito workflow list` to inspect what is currently resolvable.
