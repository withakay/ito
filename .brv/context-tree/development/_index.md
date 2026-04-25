---
children_hash: a37ce70d5e76244c57e4096b22b756c72a227be4f4897781ca92352c53496ffc
compression_ratio: 0.9652777777777778
condensation_order: 2
covers: [ito_templates/_index.md]
covers_token_total: 288
summary_level: d2
token_count: 278
type: summary
---
# Development / ITO Templates

- This domain currently centers on the **Template Bundle Retrofit** entry, which documents how the ITO template bundle was standardized without disturbing already compliant files.
- The main pattern is a **marker retrofit** for plain markdown assets under `ito-rs/crates/ito-templates/assets`: add `<!-- ITO:START -->` and `<!-- ITO:END -->` to eligible unmarked `.md` files, while leaving pre-marked files unchanged.
- Verification showed that `ito-rs/crates/ito-templates/assets/adapters` contained **no unmarked adapter sample**, so no adapter markdown required modification.

## Structural relationships and workflow

- The retrofit process depends on distinguishing **plain markdown files** from files already carrying ITO markers.
- The standardized flow is: **scan assets → add markers to plain markdown → preserve pre-marked files → verify adapter status**.
- The update is a normalization pass over the template bundle, not a rewrite of compliant content.

## Drill down

- See **Template Bundle Retrofit** for the full scope, file-level details, and verification notes.