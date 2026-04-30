---
children_hash: 973e78f9e3195411dc6c679d8855f9fd76921af86b6cdbe95f60ea78fd57cc1f
compression_ratio: 0.47426981919332406
condensation_order: 1
covers: [source_guide_workflow.md]
covers_token_total: 719
summary_level: d1
token_count: 341
type: summary
---
# Source Guide Workflow

## Overview
Ito apply work uses `source-guide.md` files as a code map / code atlas. The workflow is centered on keeping nearby guides current before changes, using them for orientation, and verifying important claims against source rather than treating the guides as authoritative. See **source_guide_workflow.md** for the full workflow.

## Structural pattern
- Guide hierarchy spans:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md` files
- Freshness is tracked in `source-guide.json`.
- The guide layer is meant to support navigation and context during apply work, not replace source truth.

## Core workflow
1. Check for nearby `source-guide.md` files before Ito apply work.
2. Refresh or create missing/stale guides using the source-guide skill.
3. Read guides for orientation.
4. Verify important implementation claims against source.
5. Update affected guides after structural changes.

## Key rules and decisions
- Guides are orientation aids, not final authority.
- Source verification is required for important claims.
- Guide updates are part of structural change follow-up.
- The workflow is explicitly tied to Ito apply changes.

## Related entry
- **source_guide_workflow.md** — canonical description of the source-guide atlas workflow and its rules.