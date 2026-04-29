---
children_hash: 2b9fe0b2a6a099b721e20c064e673d5b4e2e05bb78fc3bdcdb74085498a586f0
compression_ratio: 0.36775106082036774
condensation_order: 1
covers: [source_guide_workflow.md]
covers_token_total: 707
summary_level: d1
token_count: 260
type: summary
---
# Source Guide Workflow

Ito’s source-guide system is a code map/code atlas workflow used during apply work. The central pattern is: check nearby `source-guide.md` files, refresh missing or stale guides, read them for orientation, verify claims against source, and update affected guides after structural changes.

## Structural model
- Guide coverage spans multiple levels:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md` files
- `source-guide.json` tracks guide freshness.

## Operational rules
- Before implementing an Ito apply change, agents should inspect nearby guides.
- The `source-guide` skill is used to set up or refresh guide files when needed.
- Guides are orientation aids, not the final authority.
- Important implementation claims must be verified against source.
- After structural changes, affected guides must be updated.

## Drill-down
- `source_guide_workflow.md` — full workflow, guide hierarchy, freshness tracking, and verification rules