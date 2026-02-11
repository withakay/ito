## Context

Ito already has schema-producing logic (`ito config schema`) and typed Rust config models, but there is no guaranteed committed schema artifact that editors can resolve consistently from project config files. This creates drift risk and weak developer ergonomics when authoring `ito.json`, `.ito.json`, `.ito/config.json`, or project-level `config.json`.

## Goals / Non-Goals

**Goals:**

- Produce one canonical schema artifact in-repo from Rust config types.
- Make schema generation deterministic and part of build/check workflows.
- Ensure config files can reference the committed schema via `$schema` for completion.
- Add a drift check so source/type updates cannot land without updating schema output.

**Non-Goals:**

- Hosting/publishing schema to an external URL in this change.
- Redesigning the entire config system or merge precedence.
- Adding editor-specific plugins or IDE automation beyond JSON Schema support.

## Decisions

- **Decision: Use the Rust schema source as single source of truth.**
  - Rationale: Avoid duplicate schema definitions and keep behavior aligned with runtime types.
  - Alternative considered: hand-maintained schema file; rejected due to high drift risk.

- **Decision: Commit schema artifact at `schemas/ito-config.schema.json`.**
  - Rationale: Stable, discoverable path that config files and editors can reference locally.
  - Alternative considered: generate only at runtime; rejected because editors need a filesystem artifact.

- **Decision: Add build/check verification for schema drift.**
  - Rationale: Prevent stale committed schema when config types evolve.
  - Alternative considered: best-effort docs reminder; rejected because it is easy to miss.

- **Decision: Keep `$schema` metadata non-functional at runtime.**
  - Rationale: Editor aid only; avoids changing config semantics.

## Risks / Trade-offs

- **Build friction from drift checks** -> Provide a single documented regen command and clear failure output.
- **Path confusion across config file locations** -> Use consistent relative-path examples and template defaults.
- **Schema formatting churn in diffs** -> Keep pretty-printing deterministic and stable.

## Migration Plan

1. Introduce schema generation target and output path in `schemas/`.
2. Wire schema verification into build/check workflow.
3. Update generated/templated config files to include local `$schema` reference where applicable.
4. Regenerate schema artifact and commit it.
5. Validate with targeted tests and full checks.

Rollback: remove schema verification hook and `$schema` template updates; keep runtime config loading unchanged.

## Open Questions

- Should every generated config file include `$schema`, or only project-local files under version control?
- Should `ito config schema` default output path align to `schemas/ito-config.schema.json` when run in a project?
