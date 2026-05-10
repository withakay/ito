<!-- ITO:START -->
## Why

Ito's `validate` workflow is currently schema-agnostic and assumes Ito delta-spec markdown plus `tasks.md`. As Ito adopts additional schemas (including third-party schemas), validation can become misleading (false failures) or incomplete (validating the wrong tracking file).

## What Changes

- Add an optional `validation.yaml` file that sits next to a schema's `schema.yaml` and defines how artifacts for that schema should be validated.
- Update `ito validate <change>` to resolve the change's schema and use schema-defined validation when `validation.yaml` is present.
- When `validation.yaml` is absent, avoid Ito-specific delta/task assumptions and instead perform minimal safe checks plus emit an explicit "manual validation required" validation issue for agents/users.
- Introduce stable, versioned validator identifiers (e.g. `ito.delta-specs.v1`, `ito.tasks-tracking.v1`) so schemas can reference validation behavior without coupling to internal implementation details.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `ito-schemas`: schemas MAY include `validation.yaml` to define required artifacts, validator selection, and tracking-file validation.
- `cli-validate`: `ito validate` becomes schema-aware, reports whether validation ran in schema-driven vs legacy mode, and avoids false delta-spec failures for schemas without a validation spec.

## Impact

- Validation logic in `ito-core` will change to resolve schemas and apply schema-provided validation rules.
- CLI output/JSON may be extended to include resolved schema name/source and validation mode (legacy vs schema-driven).
- Built-in/embedded schemas may need companion `validation.yaml` files to preserve existing validation behavior for Ito-native workflows.
<!-- ITO:END -->
