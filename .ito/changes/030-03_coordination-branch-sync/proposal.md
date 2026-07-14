# Change: Coordination Branch Sync

## Why

While creating this module, Ito reproduced a session-mined failure: each `ito create change` succeeded locally but warned that coordination sync failed because `ito/internal/changes` was remote-ahead and the push was rejected as non-fast-forward.

That is exactly the kind of deterministic git state handling Ito should own. LLMs should not manually recover internal coordination branches.

## What

Add deterministic coordination branch synchronization and diagnostics:

```bash
ito coordination sync --json
ito coordination doctor --json
ito coordination lock <change-id> --json
```

All coordination writes should fetch, merge or rebase metadata, retry bounded conflicts, and return structured failure details only when deterministic recovery is not safe.

## Impact

Creating, updating, and archiving changes becomes resilient to concurrent agents and remote-ahead internal branches. Agents receive actionable JSON when human or higher-level orchestration is required.

## Out Of Scope

This change does not replace normal feature branch workflows or PR integration. It only addresses Ito's internal coordination metadata branch.

## Success Criteria

- Coordination writes retry non-fast-forward push failures after fetching and merging current remote metadata.
- `ito coordination sync --json` can be run explicitly and is safe to repeat.
- `ito coordination doctor --json` explains local, remote, and conflict state.
- Tests cover remote-ahead, concurrent writer, missing branch, and conflict cases.
