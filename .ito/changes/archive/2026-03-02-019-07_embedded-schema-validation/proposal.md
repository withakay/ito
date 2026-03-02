<!-- ITO:START -->
## Why

Ito already supports schema-driven validation via an optional `validation.yaml` placed next to a workflow schema's `schema.yaml`, but the embedded schemas shipped with Ito do not include `validation.yaml`.

This creates confusing validation behavior (legacy special-casing for some schemas and “manual validation required” for others) and makes it harder for agents to trust `ito validate` output.

## What Changes

- Ship `validation.yaml` alongside each embedded workflow schema so `ito validate` uses schema-driven validation consistently.
- Ensure schema export (`ito templates schemas export`) includes `validation.yaml` (by virtue of exporting embedded schema directories).
- Add/adjust tests so schema export assertions cover `validation.yaml` for at least the default schemas.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `ito-schemas`: embedded schemas MUST include `validation.yaml` when Ito expects schema-driven validation.

## Impact

- **Templates**: add `validation.yaml` files under `ito-rs/crates/ito-templates/assets/schemas/**`.
- **Validation**: `ito validate` will report “Using schema validation.yaml” for embedded schemas that ship it.
- **Compatibility**: additive; improves determinism and reduces reliance on legacy special cases.
<!-- ITO:END -->
