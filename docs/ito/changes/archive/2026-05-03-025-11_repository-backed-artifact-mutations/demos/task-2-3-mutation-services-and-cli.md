# Tasks 2-3: Mutation services and CLI behavior

*2026-04-30T22:39:17Z by Showboat 0.6.1*
<!-- showboat-id: 422a8ef5-2ef4-4a0d-a097-2f60998488fd -->

Added validation coverage for filesystem, SQLite, bundle-backed, remote client, and CLI success/failure behavior for repository-backed artifact mutations.

```bash
cd ito-rs && cargo test -p ito-core --lib -- artifact_mutations
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/target/debug/deps/ito_core-9b65f71c4ed51f7f)

running 7 tests
test artifact_mutations::tests::bundle_service_patches_design_and_returns_revision ... ok
test artifact_mutations::tests::bundle_service_write_creates_new_spec_artifact ... ok
test artifact_mutations::tests::validate_path_component_rejects_unsafe_capabilities ... ok
test artifact_mutations::tests::remote_bundle_client_writes_and_loads_artifacts ... ok
test artifact_mutations::tests::fs_service_patch_returns_not_found_for_missing_artifact ... ok
test artifact_mutations::tests::fs_service_writes_and_patches_proposal ... ok
test artifact_mutations::tests::fs_service_creates_spec_delta_directory_on_write ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 596 filtered out; finished in 0.00s

```

```bash
cd ito-rs && cargo test -p ito-core --test repository_runtime
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/repository_runtime.rs (/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/target/debug/deps/repository_runtime-6fe327e51667114f)

running 8 tests
test remote_runtime_uses_remote_factory ... ok
test sqlite_mode_requires_db_path ... ok
test filesystem_runtime_exposes_working_artifact_mutations ... ok
test filesystem_runtime_builds_repository_set ... ok
test sqlite_runtime_exposes_working_artifact_mutations ... ok
test sqlite_runtime_builds_repository_set ... ok
test repository_modes_return_consistent_change_names ... ok
test resolve_target_parity_between_filesystem_and_sqlite ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

```bash
cd ito-rs && cargo test -p ito-cli --test artifact_mutations
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.29s
     Running tests/artifact_mutations.rs (/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/target/debug/deps/artifact_mutations-b27e652ab63b81fc)

running 7 tests
test write_with_empty_stdin_fails ... ok
test patch_with_invalid_diff_fails ... ok
test patch_change_proposal_applies_unified_diff ... ok
test patch_nonexistent_change_fails ... ok
test write_change_proposal_replaces_contents ... ok
test write_change_spec_delta_creates_missing_capability_file ... ok
test write_spec_with_traversal_capability_fails ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.56s

```
