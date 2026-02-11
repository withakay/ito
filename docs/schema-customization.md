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
- `templates/*.md`

Example output layout:

```text
.ito/templates/schemas/
  spec-driven/
    schema.yaml
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
3. Commit project-local schema overrides if they are team conventions.
4. Keep personal-only customizations in `${XDG_DATA_HOME}/ito/schemas/`.

## Notes

- `$schema` JSON metadata in config files is unrelated to workflow schema resolution.
- If a schema cannot be found, run `ito workflow list` to inspect what is currently resolvable.
