## Key points
- Ito adopts `source-guide.md` files as a **code map / code atlas** workflow for apply changes.
- Before making an Ito apply change, agents should **check for nearby `source-guide.md` files** and refresh any that are missing or stale.
- Guides are intended as **orientation aids**, not the final source of truth; important claims must be **verified against source**.
- After structural changes, agents should **update affected guides** to keep the atlas current.
- The workflow spans multiple levels of guidance: a root guide, `ito-rs`-level guides, crates-level guides, and per-crate `source-guide.md` files.
- `source-guide.json` is used to track **freshness** of the guides.

## Structure / sections summary
- **Reason**: States the purpose of documenting the source-guide skill and atlas workflow for Ito apply changes.
- **Raw Concept**: Describes the task, changes adopted, affected files, the workflow sequence, timestamp, and author.
- **Narrative**:
  - **Structure**: Explains the layered guide hierarchy and freshness tracking.
  - **Dependencies**: Notes reliance on the source-guide skill and source verification.
  - **Highlights**: Emphasizes that guides are orientation only and must stay current after structural changes.
  - **Rules**: Lists the operational procedure for pre-change checks, guide refresh, verification, and post-change updates.
- **Facts**: Captures the workflow as discrete conventions and project facts.

## Notable entities, patterns, or decisions
- **Entities**: `source-guide.md`, `ito-rs/source-guide.md`, `ito-rs/crates/source-guide.md`, `source-guide.json`.
- **Pattern**: A layered documentation atlas, with coverage from repository root down to per-crate guides.
- **Decision**: Treat guides as **non-authoritative orientation** rather than implementation truth.
- **Decision**: Require **source verification** for important implementation claims.
- **Decision**: Enforce **post-structural-change guide updates** to maintain accuracy.