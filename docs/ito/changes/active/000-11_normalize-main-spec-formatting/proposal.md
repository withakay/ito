<!-- ITO:START -->
## Why

The `.ito/specs/` directory is the source of truth for what is built, but many specs currently read like change deltas (starting with `## ADDED Requirements` / `## MODIFIED Requirements`) and several have inconsistent titles and placeholder `TBD` purpose text.

This blurs the distinction between:

- `.ito/specs/**` (current truth)
- `.ito/changes/<change-id>/specs/**` (delta proposals)

It makes specs harder to scan, increases cognitive load for contributors, and raises the risk of authors copying the wrong structure when writing new deltas.

## What Changes

- Define a canonical markdown structure for main specs under `.ito/specs/<capability>/spec.md` (title, purpose, requirements).
- Normalize existing `.ito/specs/**/spec.md` files to the canonical structure without changing requirement semantics:
  - Ensure each main spec has an H1 title and a non-placeholder `## Purpose` section.
  - Replace top-level delta operation sections (`## ADDED|MODIFIED|REMOVED|RENAMED Requirements`) with a single `## Requirements` section.
  - Standardize H1 formatting so spec titles are consistent across the tree.
- (Optional) Add validation that detects delta-only formatting inside `.ito/specs/` and reports it as a warning (or an error in strict mode).

## Capabilities

### New Capabilities

- `spec-formatting`: Canonical structure rules for main specs and normalization constraints.

### Modified Capabilities

<!-- None -->

## Impact

- No runtime behavior changes; documentation-only changes to `.ito/specs/**`.
- Improves spec readability and makes the truth-vs-delta boundary clearer.
- Primary risk is tooling assumptions about main spec headings; mitigation is to run `ito validate --strict` and spot-check `ito list --specs` / `ito show <spec>` before and after normalization.
<!-- ITO:END -->
