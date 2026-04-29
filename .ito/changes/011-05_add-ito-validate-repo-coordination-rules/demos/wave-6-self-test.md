# Wave 6: ito validate repo self-test on this repo

*2026-04-29T13:23:22Z by Showboat 0.6.1*
<!-- showboat-id: 5aada8d1-4018-4b52-a17a-0a8b314c1d69 -->

Wave 6 (task 6.1) verifies the wired pre-commit hook works on this repo. The hook script delegates to ito validate repo --staged --strict; the engine runs all six built-in rules and reports them as active in the current configuration.

```bash
ito-rs/tools/hooks/pre-commit ; echo exit=$?
```

```output
Repository validation passed.
exit=0
```

```bash
target/debug/ito validate repo --staged --strict ; echo exit=$?
```

```output
Repository validation passed.
exit=0
```

```bash
target/debug/ito validate repo --list-rules
```

```output
Built-in repository validation rules:

  [x] coordination/branch-name-set                     WARNING  (always active)
  [x] coordination/gitignore-entries                   WARNING  changes.coordination_branch.storage == worktree
  [x] coordination/staged-symlinked-paths              ERROR    changes.coordination_branch.storage == worktree && staged context present
  [x] coordination/symlinks-wired                      ERROR    changes.coordination_branch.storage == worktree
  [x] worktrees/layout-consistent                      WARNING  worktrees.enabled == true
  [x] worktrees/no-write-on-control                    ERROR    worktrees.enabled == true
```
