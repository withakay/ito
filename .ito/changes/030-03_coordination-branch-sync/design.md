# Design: Coordination Branch Sync

## Overview

Use a deterministic sync engine for Ito's internal coordination branch. The engine should be callable directly and used automatically by coordination writes.

## Prior Art

The audit mirror already uses temp worktrees, unique temp names, JSONL merging, and retry-on-conflict behavior. Coordination sync should reuse those ideas where appropriate instead of leaving non-fast-forward recovery to agents.

## Commands

```bash
ito coordination sync --json
ito coordination doctor --json
ito coordination lock <change-id> --json
```

## Write Flow

1. Read configured coordination branch name.
2. Fetch remote coordination ref when a remote exists.
3. Materialize current remote metadata in a temp worktree or isolated index.
4. Merge local metadata changes deterministically.
5. Commit metadata changes if needed.
6. Push.
7. On non-fast-forward, retry from step 2 with a bounded retry count.
8. On semantic conflict, return structured JSON with conflict paths and suggested resolution.

## Lock Model

Locks should be advisory metadata with owner, host, process or session identifier, timestamp, and expiration. Expired locks should be ignored or cleaned by sync.

## Risks

Internal branch handling must not mutate the user's active feature branch. Use isolated worktrees or low-level git commands against explicit refs.
