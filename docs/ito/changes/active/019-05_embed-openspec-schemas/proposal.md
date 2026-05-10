<!-- ITO:START -->
## Why

Ito already supports workflow schemas, but the built-in set is small and Ito lacks real-world third-party schemas to drive schema-validation and template-export UX. Embedding a curated set of OpenSpec schemas improves out-of-the-box workflow options while providing concrete inputs for validating Ito's schema-driven validation design.

## What Changes

- Vendor selected OpenSpec-style schemas (at least `minimalist` and `event-driven`) into Ito's embedded schema assets so they are available without any install step.
- Ensure the embedded schemas appear in schema listing/selection and in `ito templates schemas export ...` output.
- Add clear, in-tree attribution for the upstream `openspec-schemas` repository and comply with its license requirements.
- Ship Ito-authored `validation.yaml` files alongside each embedded OpenSpec schema so `ito validate` produces non-misleading results (presence checks + an explicit manual-validation note for semantic content).

## Capabilities

### New Capabilities

- `embedded-openspec-schemas`: Embed OpenSpec schemas with attribution and schema-appropriate validation configuration.

### Modified Capabilities

- (none)

## Impact

- Embedded assets: add new schema directories under `ito-rs/crates/ito-templates/assets/schemas/`.
- Tooling UX: schema listing/selection and `ito templates schemas export` will include the newly embedded schemas.
- Compliance: add a repository-tracked attribution artifact (for example `THIRD_PARTY_NOTICES.md`) and include upstream license text or references as required.
- Validation: add `validation.yaml` next to each embedded OpenSpec schema's `schema.yaml`.
<!-- ITO:END -->
