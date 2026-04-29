# Task 2.1 Demo: Coordination Symlink Repair

## Scenario

Coordination worktree mode now repairs safe `.ito/` wiring drift during init/sync while preserving hard failures for ambiguous duplicate state.

## Covered Behavior

- Missing coordination links are recreated by `wire_coordination_symlinks`.
- Correct symlinks whose target directories were removed recreate the missing target directory.
- Empty generated directories under `.ito/` are removed and replaced with links to the coordination worktree.
- Wrong symlink targets fail with actual and expected target paths.
- Non-empty local duplicate directories fail without moving content implicitly.
- `sync_coordination_worktree_with_runner` repairs safe wiring before running git synchronization.
- Missing remote configuration remains non-fatal after safe local repair, preserving backend/local archive flows without `origin`.

## Verification

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-core coordination
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test archive_remote_mode remote_archive_succeeds_without_local_active_change_markdown -- --nocapture
DEVELOPER_DIR=/Library/Developer/CommandLineTools make check
```

All verification commands passed on 2026-04-29.
