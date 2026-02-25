<!-- ITO:START -->
## Context

Ito currently validates changes using logic that is effectively coupled to the `spec-driven` workflow:

- change specs are parsed as Ito delta specs (operation headers + `### Requirement:` + `#### Scenario:`)
- task tracking is validated by attempting to parse `.ito/changes/<change-id>/tasks.md`

Ito also supports multiple workflow schemas (`schema.yaml`) that define which artifacts exist and how apply-stage progress is tracked (`apply.tracks`). As Ito adopts additional schemas (including third-party schemas), validation must avoid false failures (e.g. "no deltas") and must validate the correct tracking file.

## Goals / Non-Goals

**Goals:**

- Make `ito validate <change-id>` schema-aware.
- Add an optional `validation.yaml` file next to `schema.yaml` to define validation rules for a schema.
- Support schema-driven tracking-file validation using `apply.tracks` (when configured).
- Provide stable, versioned validator identifiers so schemas reference behavior without depending on internal implementation details.
- When no schema validation configuration exists, emit an explicit issue indicating validation is incomplete and requires manual verification.

**Non-Goals:**

- Executing arbitrary external validators (commands) declared in schemas.
- Supporting OpenSpec filesystem paths (`openspec/schemas/...`) as schema search roots.
- Redefining or breaking the existing Ito delta spec format or tasks tracking format.

## Decisions

### Decision: Add `validation.yaml` as a schema companion file

We introduce an optional `validation.yaml` file in the same schema directory as `schema.yaml`.

Rationale:

- Keeps workflow definition (`schema.yaml`) separate from validation policy.
- Allows Ito to ship validation rules for embedded schemas without modifying upstream schema.yaml files.
- Enables incremental adoption: schemas without `validation.yaml` still work.

### Decision: Validation keys use snake_case

The canonical `validation.yaml` format uses `snake_case` keys.

Rationale:

- Matches Ito's current YAML style.
- Minimizes serde rename glue.

### Decision: Validator registry uses stable, versioned ids

Schema validation refers to validators by string ids (for example, `ito.delta-specs.v1`).

Rationale:

- Makes schema definitions portable and forwards-compatible.
- Allows multiple validator versions to coexist.

### Decision: Legacy mode emits an explicit manual validation issue

When a schema has no `validation.yaml`, `ito validate` does not run Ito-specific delta parsing by default.
Instead, it runs minimal schema-independent checks plus an explicit issue stating that manual validation is required.

Rationale:

- Prevents false failures for non-delta schemas.
- Provides a clear signal to agents/users about validation coverage.

## Risks / Trade-offs

- [Risk] Introducing schema-aware validation could change existing output expectations.
  -> Mitigation: keep current behavior for Ito-native schemas by shipping embedded `validation.yaml` that selects the current validators.

- [Risk] Schemas without `validation.yaml` may validate "less" than before.
  -> Mitigation: only reduce validation when schema is not known Ito-native; emit a clear issue so users/agents are not misled.

- [Risk] Tracking file validation can be bypassed if the tracking file is not parseable but validation rules are missing.
  -> Mitigation: include a default warning/info in legacy mode and allow strict mode to fail on missing validation configuration.

<!-- ITO:END -->
