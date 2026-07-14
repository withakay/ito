<!-- ITO:START -->
## Why

Ito already supports multiple change schemas, but the current proposal flow still pushes users and agents toward a neutral `ito-proposal` path that usually lands on `spec-driven`, even when a smaller fix or a supporting platform/tooling change would fit `minimalist` or `tdd` better.

This makes the front door to change creation feel heavier than necessary for fixes, while still failing to give enough structure when users are exploring a new feature. Ito needs opinionated intake and routing so users can express intent first and let the workflow bias the proposal shape appropriately.

## What Changes

- Add a dedicated proposal-intake capability that clarifies problem, scope, constraints, and success before scaffolding a change.
- Add intent-biased proposal entrypoints so `ito-fix` and `ito-feature` drive different question patterns and schema recommendations, while `ito-proposal` remains the neutral fallback.
- Upgrade schema selection guidance from a flat schema list to decision support based on change shape, including localized bug fixes and supporting platform/infrastructure work.
- Define how intake summaries hand off into proposal creation so the workflow does not rediscover the same context twice.

## Capabilities

### New Capabilities

- `proposal-intake`: A pre-proposal intake flow that gathers intent, boundaries, and readiness before change scaffolding.
- `change-request-routing`: Opinionated `ito-fix` and `ito-feature` entrypoints that bias proposal creation without removing user override.
- `schema-selection-guidance`: Decision rules that recommend the right existing schema for features, fixes, and supporting platform/tooling changes.

### Modified Capabilities

<!-- None -->

## Impact

- **Skills and commands**: Add new intake and intent-biased workflow assets, and update existing proposal guidance.
- **Templates and instructions**: Expand embedded guidance so agents can recommend `minimalist`, `spec-driven`, or `tdd` intentionally instead of defaulting to `spec-driven`.
- **Workflow UX**: Introduce a clearer split between feature discovery, fix-oriented change creation, and neutral proposal authoring.
- **Validation and tests**: Update template and instruction coverage for the new routing and schema recommendation rules.
<!-- ITO:END -->
