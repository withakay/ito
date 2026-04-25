---
children_hash: 28b9ee95aaf0edeebdb7a5fbc5d4d3f014d33afca40de55ee205ee422be11025
compression_ratio: 0.6920821114369502
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 341
summary_level: d3
token_count: 236
type: summary
---
# Development / ITO Templates

This domain is centered on the **Template Bundle Retrofit** work, which standardized the ITO template bundle without altering already compliant files. The key mechanism is a **marker retrofit** for plain markdown assets under `ito-rs/crates/ito-templates/assets`: eligible unmarked `.md` files receive `<!-- ITO:START -->` and `<!-- ITO:END -->`, while pre-marked files remain unchanged.

## Core structure and workflow

- The retrofit distinguishes **plain markdown files** from files already carrying ITO markers.
- Standard flow: **scan assets → add markers to plain markdown → preserve pre-marked files → verify adapter status**.
- Verification confirmed that `ito-rs/crates/ito-templates/assets/adapters` had **no unmarked adapter sample**, so no adapter markdown required modification.

## Drill down

- See **Template Bundle Retrofit** for file-level scope, normalization rules, and verification details.