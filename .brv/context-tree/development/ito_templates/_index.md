---
children_hash: ac3ed2f0527aa62efd8461938712b422cb52a9b652594bf5781de00c2ced850f
compression_ratio: 0.6221804511278195
condensation_order: 1
covers: [template_bundle_retrofit.md]
covers_token_total: 532
summary_level: d1
token_count: 331
type: summary
---
# Template Bundle Retrofit

Structural knowledge for marker standardization across `ito-rs/crates/ito-templates/assets`. The retrofit applied `<!-- ITO:START -->` / `<!-- ITO:END -->` markers to plain markdown files while preserving any file that was already pre-marked.

## Core outcome
- All plain `.md` files under `ito-rs/crates/ito-templates/assets` were retrofitted with ITO markers.
- Pre-marked files were explicitly left unchanged.
- Verification found no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample was modified.

## Key structure and rules
- The process is governed by a simple distinction:
  - **plain markdown** → retrofit markers
  - **already marked markdown** → do not alter
- This preserves compliance without rewriting already correct files.
- The operation spans the template bundle assets directory, with adapter verification treated as a separate check.

## Process pattern
`scan assets -> add markers to plain markdown -> leave pre-marked files unchanged -> verify adapter sample status`

## Drill-down references
- `template_bundle_retrofit.md` — primary retrofit summary and verification facts
- `template_bundle_retrofit.abstract.md` — abstracted structural view
- `template_bundle_retrofit.overview.md` — overview of the marker retrofit approach
