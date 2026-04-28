## Key points
- Audit mirror synchronization in `mirror.rs` uses **unique temporary names** for both worktrees and orphan branches to avoid collisions during parallel activity.
- Temp names include the **process ID**, a **SystemTime-based timestamp**, and a **process-local atomic counter**.
- JSONL merge logic **deduplicates identical entries**, preserves order, and **aggregates adjacent reconciled events** by increasing a count.
- Mirror retention is bounded by both **age** and **count**: events older than 30 days from the newest event are truncated, and the log is capped at 1000 events.
- The sync flow is Git-based: detect worktree, create temp worktree, fetch/checkout or create orphan branch, merge JSONL, stage/commit, then push or update ref.
- Conflict handling is explicitly retry-oriented: **push conflicts** and **ref update conflicts** get a single retry path, with failure after that.
- The implementation is designed to be **best-effort** and only runs **inside a Git worktree**.

## Structure / sections summary
- **Reason**: Explains that the document records audit mirror worktree naming, branch conflict handling, and JSONL merge behavior from `mirror.rs`.
- **Raw Concept**: Summarizes the task, change list, target file, sync flow, naming patterns, and timestamp.
- **Narrative**
  - **Structure**: Describes module organization: sync, append, merge, truncation, Git helpers, cleanup.
  - **Dependencies**: Notes reliance on Git worktrees/refs and `serde_json` parsing of `AuditEvent` JSONL.
  - **Highlights**: Emphasizes collision avoidance in `.git/worktrees` metadata and bounded mirror growth.
  - **Rules**: States operational constraints and retry behavior on missing branches and conflicts.
  - **Examples**: Provides example error messages for unavailable branches and concurrent-write conflicts.
- **Facts**: Enumerates the main implementation facts, including naming schemes, scope, update strategy, merge behavior, retention policy, and conflict retry policy.

## Notable entities, patterns, or decisions
- **Naming patterns**
  - `ito-audit-mirror-{pid}-{nanos}-{sequence}`
  - `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
- **Atomic counter**: Used to prevent collisions when multiple operations occur in the same timestamp tick.
- **Conflict detection signals**: Git outputs such as **non-fast-forward**, **cannot lock ref**, and **reference already exists**.
- **Design decision**: Prefer **detached worktrees** and **orphan branches** as fallback when the remote branch is missing.
- **Retry policy**: One retry on conflict for both internal branch appends and remote push failures.
- **Error messages / constraints**
  - Internal audit branch unavailable outside a Git worktree
  - Failed to update internal audit branch due to concurrent writes
  - Audit mirror push to branch failed due to a remote conflict