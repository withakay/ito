# Wave 3: Docs and Validation

*2026-04-30T10:57:24Z by Showboat 0.6.1*
<!-- showboat-id: 147a3839-c7f0-4a6f-a9c8-c7004927bf7d -->

Updated worktree instructions to document automatic Ito wiring via worktree ensure and the init update repair fallback, then re-ran docs and strict change validation.

```bash
cd <repo-root>/ito-worktrees/001-37_fix-worktree-symlink-recovery && cargo test -p ito-cli --test init_coordination
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.36s
     Running tests/init_coordination.rs (target/debug/deps/init_coordination-5fe7bf043430e821)

running 5 tests
test init_no_coordination_worktree_writes_embedded_storage ... ok
test init_without_git_remote_falls_back_gracefully ... ok
test init_upgrade_does_not_touch_coordination_storage ... ok
test init_with_git_remote_creates_coordination_worktree ... ok
test init_update_repairs_missing_coordination_symlink ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.03s

```

```bash
cd <repo-root>/ito-worktrees/001-37_fix-worktree-symlink-recovery && make docs
```

```output
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
   Generated <repo-root>/ito-worktrees/001-37_fix-worktree-symlink-recovery/target/doc/ito_backend/index.html and 9 other files
rm -rf docs/rustdoc
cp -R target/doc docs/rustdoc
```

```bash
cd <repo-root>/ito-worktrees/001-37_fix-worktree-symlink-recovery && ito validate 001-37_fix-worktree-symlink-recovery --strict
```

```output
Change '001-37_fix-worktree-symlink-recovery' is valid
```
