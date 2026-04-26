---
children_hash: 5878758f59625e7a3f2ba6fc580befbfcc82331dfc39e36aa10ed1df8a1fc531
compression_ratio: 0.452
condensation_order: 1
covers: [template_bundle_retrofit.md]
covers_token_total: 500
summary_level: d1
token_count: 226
type: summary
---
# Development / ITO Templates

## Template Bundle Retrofit
- Retrofitted plain markdown assets under `ito-rs/crates/ito-templates/assets` with `<!-- ITO:START -->` and `<!-- ITO:END -->` markers.
- Pre-marked files were left unchanged; only unmarked plain `.md` files were eligible for modification.
- Verification across `ito-rs/crates/ito-templates/assets/adapters` found no unmarked adapter sample to update, so no adapter markdown was modified.

### Key relationships and rules
- The retrofit depends on distinguishing plain markdown files from files already carrying ITO markers.
- The update standardized marker presence across the template bundle without rewriting compliant files.
- Flow: scan assets → add markers to plain markdown → preserve pre-marked files → verify adapter status.

### Drill down
- `template_bundle_retrofit.md` for the full retrofit note, file scope, and verification facts.
