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

```bash
make check-max-lines 2>&1 | tail -3 ; echo exit=$?
```

```output
  - ito-rs/crates/ito-core/src/templates/mod.rs: 1015 (consider splitting)
  - ito-rs/crates/ito-core/tests/validate.rs: 1010 (consider splitting)
  - ito-rs/crates/ito-core/src/audit/mirror.rs: 1003 (consider splitting)
exit=0
```

Final gate (6.3): cargo build/clippy/doc/test all clean, make arch-guardrails pass, make check-max-lines pass, cargo deny pass. 18/18 tasks complete. Pre-existing init_more failure on main is unrelated to this change.
