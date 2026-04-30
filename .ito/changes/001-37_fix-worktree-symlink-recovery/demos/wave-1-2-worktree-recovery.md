# Wave 1-2: Worktree Recovery

*2026-04-30T10:46:31Z by Showboat 0.6.1*
<!-- showboat-id: 435372cd-045c-4a46-a1b1-581b76fdf788 -->

Implemented automatic coordination symlink wiring in worktree ensure, explicit repair support for init update on existing worktrees, and create-change recovery/error handling for missing worktree wiring.

```bash
cd /Users/jack/Code/withakay/ito/ito-worktrees/001-37_fix-worktree-symlink-recovery && cargo test -p ito-core --test worktree_ensure_e2e
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.33s
     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-8e817f71bae0c07d)

running 4 tests
test ensure_worktree_disabled_returns_cwd ... ok
test ensure_worktree_repairs_missing_coordination_links_in_existing_worktree ... ok
test ensure_worktree_creates_and_initializes_with_include_files ... ok
test ensure_worktree_with_setup_script ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

```

```bash
cd /Users/jack/Code/withakay/ito/ito-worktrees/001-37_fix-worktree-symlink-recovery && cargo test -p ito-core --test create
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/create.rs (target/debug/deps/create-c5e723a8111a8f02)

running 17 tests
test create_change_rejects_uppercase_names ... ok
test create_change_in_sub_module_rejects_missing_parent_module ... ok
test create_change_reports_actionable_error_when_coordination_worktree_missing ... ok
test create_module_creates_directory_and_module_md ... ok
test create_module_returns_existing_module_when_name_matches ... ok
test create_change_in_sub_module_rejects_missing_sub_module_dir ... ok
test create_module_writes_description_to_purpose_section ... ok
test create_change_rewrites_module_changes_in_ascending_change_id_order ... ok
test create_change_allocates_next_number_from_existing_change_dirs ... ok
test create_change_creates_change_dir_and_updates_module_md ... ok
test create_change_in_sub_module_uses_composite_id_format ... ok
test create_change_in_sub_module_checklist_is_sorted_ascending ... ok
test create_change_in_sub_module_writes_checklist_to_sub_module_md ... ok
test create_change_repairs_missing_coordination_links_before_module_lookup ... ok
test allocation_state_sub_module_keys_sort_after_parent ... ok
test create_change_in_sub_module_allocates_independent_sequence ... ok
test create_change_writes_allocation_modules_in_ascending_id_order ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

```bash
cd /Users/jack/Code/withakay/ito/ito-worktrees/001-37_fix-worktree-symlink-recovery && cargo test -p ito-cli --test init_coordination
```

```output
warning: call to `.clone()` on a reference in this situation does nothing
    --> ito-rs/crates/ito-cli/tests/init_coordination.rs:263:18
     |
 263 |         rust_path.clone(),
     |                  ^^^^^^^^
     |
     = note: the type `Path` does not implement `Clone`, so calling `clone` on `&Path` copies the reference, which does not do anything and can be removed
     = note: `#[warn(noop_method_call)]` on by default
help: remove this redundant call
     |
 263 -         rust_path.clone(),
 263 +         rust_path,
     |
help: if you meant to clone `Path`, implement `Clone` for it
    -->  /opt/homebrew/Cellar/rust/1.95.0/lib/rustlib/src/rust/library/std/src/path.rs:2327:1
     |
2327 + #[derive(Clone)]
2328 | pub struct Path {
     |

warning: `ito-cli` (test "init_coordination") generated 1 warning (run `cargo fix --test "init_coordination" -p ito-cli` to apply 1 suggestion)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.30s
     Running tests/init_coordination.rs (target/debug/deps/init_coordination-5fe7bf043430e821)

running 5 tests
test init_no_coordination_worktree_writes_embedded_storage ... ok
test init_without_git_remote_falls_back_gracefully ... ok
test init_upgrade_does_not_touch_coordination_storage ... ok
test init_with_git_remote_creates_coordination_worktree ... ok
test init_update_repairs_missing_coordination_symlink ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s

```
