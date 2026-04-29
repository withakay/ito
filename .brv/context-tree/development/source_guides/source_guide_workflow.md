---
title: Source Guide Workflow
summary: Ito uses source-guide.md files as a code map/code atlas; agents refresh nearby guides before apply work, treat guides as orientation, verify claims against source, and update guides after structural changes.
tags: []
related: []
keywords: []
createdAt: '2026-04-28T20:08:29.775Z'
updatedAt: '2026-04-28T20:08:29.775Z'
---
## Reason
Document the source-guide skill and atlas workflow for Ito apply changes

## Raw Concept
**Task:**
Document the source-guide skill and code atlas workflow used by Ito apply changes

**Changes:**
- Adopted source-guide skill as the code map/code atlas workflow
- Established pre-apply checks for nearby source-guide.md files
- Defined guide refresh, verification, and post-structure-update behavior

**Files:**
- source-guide.md
- ito-rs/source-guide.md
- ito-rs/crates/source-guide.md
- source-guide.json

**Flow:**
check nearby source-guide.md files -> refresh missing/stale guides -> read guides for orientation -> verify claims against source -> update affected guides after structural changes

**Timestamp:** 2026-04-28

**Author:** ByteRover

## Narrative
### Structure
The workflow spans a root source guide, an ito-rs-level guide, a crates-level guide, and per-crate source-guide.md files, with freshness tracked in source-guide.json.

### Dependencies
Relies on the source-guide skill before Ito apply work and on source verification for important implementation claims.

### Highlights
Guides are explicitly treated as orientation aids rather than the final authority, and they must be kept current after structural changes.

### Rules
Before implementing an Ito apply change, agents should check for nearby source-guide.md files, use the source-guide skill to set up or refresh missing/stale guides, read guides as orientation rather than authority, verify important claims against source, and update affected guides after structural changes.

## Facts
- **workflow**: Ito now uses the source-guide skill as its code map/code atlas workflow. [project]
- **pre_change_check**: Before implementing an Ito apply change, agents should check for nearby source-guide.md files. [convention]
- **guide_refresh**: Agents should use the source-guide skill to set up or refresh missing or stale guides. [convention]
- **guide_role**: Guides should be read as orientation rather than authority. [convention]
- **verification_rule**: Important claims must be verified against source. [convention]
- **post_change_update**: Affected guides should be updated after structural changes. [convention]
- **guide_coverage**: The repository includes a root source-guide.md, ito-rs/source-guide.md, ito-rs/crates/source-guide.md, and one source-guide.md per Rust crate. [project]
- **freshness_tracking**: source-guide.json tracks source-guide freshness. [project]
