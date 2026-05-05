# Apply Instruction Sync Is Opt-In

*2026-05-01T14:20:00Z by Showboat 0.6.1*
<!-- showboat-id: 016-13-apply-sync-opt-in -->

This demo verifies that `ito agent instruction apply` now renders from local state by default and only fetches coordination state when `--sync` is passed.

## Default apply instructions are offline-safe

```bash
cd <repo-root>/ito-worktrees/016-13_optimize-agent-instructions
cargo test -p ito-cli --test agent_instruction_apply_sync apply_instruction_does_not_fetch_by_default_in_worktree_mode
```

```output
running 1 test
test apply_instruction_does_not_fetch_by_default_in_worktree_mode ... ok

test result: ok. 1 passed; 0 failed
```

## `--sync` opts into coordination fetch

```bash
cd <repo-root>/ito-worktrees/016-13_optimize-agent-instructions
cargo test -p ito-cli --test agent_instruction_apply_sync apply_instruction_sync_flag_fetches_coordination_branch_in_worktree_mode
```

```output
running 1 test
test apply_instruction_sync_flag_fetches_coordination_branch_in_worktree_mode ... ok

test result: ok. 1 passed; 0 failed
```

## Strict change validation

```bash
cd <repo-root>/ito-worktrees/016-13_optimize-agent-instructions
ito validate 016-13_optimize-agent-instructions --strict
```

```output
Change '016-13_optimize-agent-instructions' is valid
```
