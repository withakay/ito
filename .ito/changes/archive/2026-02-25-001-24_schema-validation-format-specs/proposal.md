<!-- ITO:START -->
## Why

Ito validates two author-facing markdown formats today (delta specs and tasks tracking), but those formats are implicit and unversioned. This makes compatibility and evolution unclear, and it prevents schema validation and error messages from pointing authors at a stable, canonical specification.

## What Changes

- Add two versioned, normative format specifications: `delta-specs` and `tasks-tracking`.
- Assign stable validator ids (`ito.delta-specs.v1`, `ito.tasks-tracking.v1`) so schemas and validators can reference these formats.
- Update validation issues for these formats to cite the validator id so authors can find the correct spec.
- Keep v1 aligned with current parser/validator behavior (no breaking changes intended).

## Capabilities

### New Capabilities

- `delta-specs`: Versioned specification of delta spec markdown used under `.ito/changes/<change-id>/specs/**`.
- `tasks-tracking`: Versioned specification of `tasks.md` tracking markdown (checkbox format + enhanced wave-based format).

### Modified Capabilities

<!-- None -->

## Impact

- Documentation: new normative spec docs will be added under `.ito/specs/`.
- Validation: schema/validator wiring and error messages will be updated to reference validator ids and point to specs.
- Compatibility: existing changes and tasks files remain valid; v1 definitions track current behavior.
<!-- ITO:END -->
