<!-- ITO:START -->
## Why

Ito supports multiple workflow schemas (for example: `spec-driven`, `minimalist`, `tdd`, `event-driven`), but agents and users almost always default to `spec-driven` because schema options are not surfaced at the moment a change is created.

This leads to underuse of schemas and inconsistent change proposal quality when a different workflow would be a better fit.

## What Changes

- Add a new instruction artifact `ito agent instruction schemas` (and `--json`) that lists available schemas and describes when to choose each one.
- Update change-proposal guidance so that when creating a change proposal (via the `ito-write-change-proposal` skill), the agent asks the user which schema they want to use.
- Update bootstrap/tooling guidance to explain what schemas are, how to list them, and how schema selection affects generated artifacts.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `agent-instructions`: Add `schemas` as a supported instruction type (and define its JSON output contract).

## Impact

- **CLI**: `ito agent instruction schemas` becomes available; `ito create change --schema <name>` remains the mechanism for applying the selection.
- **Templates**: Add an instruction template for schema listing/selection guidance.
- **Skills**: Update `ito-write-change-proposal` to ask for schema selection up front, using the new instruction artifact.
- **Compatibility**: Additive. Default behavior stays `spec-driven` when no schema is chosen.
<!-- ITO:END -->
