# Wave 2: Coordination/Worktrees Rules + Pre-commit Detection

*2026-04-29T12:55:13Z by Showboat 0.6.1*
<!-- showboat-id: 63c85c11-cfac-4f53-bbee-e4765f1aa3e4 -->

Wave 2 (tasks 2.1-2.4) ships six built-in rules and the pre-commit framework detector. Task 2.1 extracted gitignore_entries() from coordination.rs (lockstep with COORDINATION_DIRS, regression-tested). Task 2.2 added 4 coordination/* rules (symlinks-wired, gitignore-entries, staged-symlinked-paths, branch-name-set). Task 2.3 added 2 worktrees/* rules (no-write-on-control, layout-consistent). Task 2.4 added detect_pre_commit_system covering Prek/PreCommit/Husky/Lefthook/None with deterministic ordering. RuleRegistry::built_in() now wires all six rules in deterministic order.

```bash
cargo test -p ito-core --lib validate_repo 2>&1 | tail -3
```

```output

test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 602 filtered out; finished in 0.02s

```
