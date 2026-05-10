## Why

Workflow schema resolution currently depends on filesystem package paths that are brittle across install modes and can fail when schemas are not shipped next to the binary. We should embed built-in schemas in `ito-templates`, add a project-local override location, and provide an explicit export command so users can customize schemas from a known baseline.

## What Changes

- Move built-in workflow schemas into `ito-rs/crates/ito-templates/assets/schemas` and load them from embedded assets.
- Extend schema resolution precedence to include project-local overrides at `.ito/templates/schemas/<name>/` in addition to existing user/global override paths.
- Add CLI support to export built-in schemas to disk for customization, including `ito templates schemas export -f '.ito/templates/schemas'`.
- Ensure exported schema directories include `schema.yaml` plus `templates/*.md` with deterministic output.
- Keep backward compatibility during migration by preserving legacy path behavior where practical.

## Capabilities

### New Capabilities

- `cli-templates-schemas`: CLI command group behavior for exporting built-in workflow schema bundles.

### Modified Capabilities

- `artifact-graph`: schema directory resolution and fallback order for built-in and override schema sources.

## Impact

- Affected code:
  - `ito-rs/crates/ito-templates` (embed schema assets)
  - `ito-rs/crates/ito-core/src/workflow` (schema resolution/loading)
  - `ito-rs/crates/ito-cli` (new templates/schemas export command surface)
  - repository `schemas/` layout and migration glue
- Developer UX: reliable defaults on fresh installs, plus easy local schema customization.
- Packaging/distribution: removes runtime dependence on colocated repo `schemas/` for defaults.
