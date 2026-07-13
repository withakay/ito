# Wave 1: Legacy coordination detection and command policy

*2026-07-13T12:20:57Z by Showboat 0.6.1*
<!-- showboat-id: d816ce71-228d-4b9a-9fb1-e36c67df4cff -->

The detector gathers configuration, managed-path, link-target, and gitignore evidence without mutation. The CLI policy then classifies every compiled top-level command and fails closed for unknown or mutating operations.

```bash
CARGO_TARGET_DIR=/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/target-showboat cargo test --manifest-path /Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/Cargo.toml -p ito-core legacy_coordination
```

```output
    Blocking waiting for file lock on build directory
    Finished `test` profile [optimized + debuginfo] target(s) in 8.41s
     Running unittests src/lib.rs (target-showboat/debug/deps/ito_core-f94ddd81afa69f58)

running 13 tests
test legacy_coordination::legacy_coordination_tests::classification_serialization_has_one_stable_tagged_shape ... ok
test legacy_coordination::legacy_coordination_tests::configured_worktree_storage_is_legacy_even_without_links ... ok
test legacy_coordination::legacy_coordination_tests::broken_expected_link_is_still_legacy_evidence ... ok
test legacy_coordination::legacy_coordination_tests::disabled_storage_with_no_managed_paths_is_absent ... ok
test legacy_coordination::legacy_coordination_tests::partial_real_directory_materialization_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::wrong_link_target_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::mixed_link_and_non_empty_real_directory_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::worktree_config_with_materialized_directories_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::partial_coordination_gitignore_entries_are_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::real_directories_with_embedded_storage_are_main_compatible ... ok
test legacy_coordination::legacy_coordination_tests::residual_managed_gitignore_marker_is_ambiguous_after_materialization ... ok
test legacy_coordination::legacy_coordination_tests::inspection_does_not_change_files_or_links ... ok
test legacy_coordination::legacy_coordination_tests::expected_coordination_links_are_legacy ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 730 filtered out; finished in 0.05s

     Running tests/archive.rs (target-showboat/debug/deps/archive-dfc04dd76b886ddf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target-showboat/debug/deps/audit_mirror-6dd1472ff1603cbe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target-showboat/debug/deps/audit_storage-f211900d01a08e99)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target-showboat/debug/deps/backend_archive-d28103fb364261b0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target-showboat/debug/deps/backend_auth-0410f61f1d641fc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target-showboat/debug/deps/backend_auth_service-88a8a2ae9d6c3712)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target-showboat/debug/deps/backend_client_mode-a40d955a657778da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target-showboat/debug/deps/backend_module_repository-68427bdb5aea49fa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target-showboat/debug/deps/backend_sub_module_support-a647e5af5d4c8ec1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target-showboat/debug/deps/change_repository_lifecycle-289502ae25eeb59d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target-showboat/debug/deps/change_repository_orchestrate_metadata-d3c893fb55908635)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target-showboat/debug/deps/change_repository_parity-fb49c1433ce7555f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target-showboat/debug/deps/change_target_resolution_parity-502b6d08def9b4a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target-showboat/debug/deps/coordination_worktree-e5d8b80bc4da60d0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target-showboat/debug/deps/create-85b618baa4d358ce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/distribution.rs (target-showboat/debug/deps/distribution-db95f824ddc73a72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target-showboat/debug/deps/event_forwarding-ab84399b75203c00)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target-showboat/debug/deps/grep_scopes-ab419613daaeca7f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target-showboat/debug/deps/harness_context-df829d645d274599)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target-showboat/debug/deps/harness_opencode-c638bb84de72f309)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target-showboat/debug/deps/harness_streaming-5ebf02004f3f9719)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target-showboat/debug/deps/harness_stub-cda686dbe23d6c45)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target-showboat/debug/deps/import-9f1c49227f5ac8f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target-showboat/debug/deps/io-7e76b6d1a1b3744d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target-showboat/debug/deps/orchestrate_run_state-edd6d0d8fa43b767)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target-showboat/debug/deps/planning_init-230f5cd162c9ab18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/ralph.rs (target-showboat/debug/deps/ralph-0de62592c0fe0294)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target-showboat/debug/deps/repo_index-a8c4ebc8a98df7a9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target-showboat/debug/deps/repo_integrity-37487272a4dfa68c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target-showboat/debug/deps/repo_paths-e4e7a2ee5edf9935)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target-showboat/debug/deps/repository_runtime-af15d63c3d0be33f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target-showboat/debug/deps/repository_runtime_config_validation-a4ba721776f063c6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target-showboat/debug/deps/show-e765750892666d52)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target-showboat/debug/deps/spec_repository_backends-cd8ec10693f8d69c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target-showboat/debug/deps/spec_show_repository-1b7eb8cac80086bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target-showboat/debug/deps/sqlite_archive_mirror-0fc4721021f45a26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target-showboat/debug/deps/sqlite_task_mutations-9ab4b380c3c8f60d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target-showboat/debug/deps/stats-18afc9eaba4a11c2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target-showboat/debug/deps/task_repository_summary-82e9c7c22e7e0146)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target-showboat/debug/deps/tasks_api-dce70601399fd872)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target-showboat/debug/deps/tasks_checkbox_format-02dcf9c5192e8bca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target-showboat/debug/deps/tasks_orchestration-4dc6458c17828eb8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target-showboat/debug/deps/templates_apply_instructions-36fc2c2f66b60a0b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target-showboat/debug/deps/templates_change_status-60b55524446a585f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target-showboat/debug/deps/templates_review_context-1cea0c44d22ab372)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target-showboat/debug/deps/templates_schema_resolution-c3c66a79b2847c37)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target-showboat/debug/deps/templates_schemas_listing-668137b6e72d35ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target-showboat/debug/deps/templates_user_guidance-be58f8ababda5411)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target-showboat/debug/deps/traceability_e2e-75c23d62c01e723f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target-showboat/debug/deps/validate-e28a396cec365936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target-showboat/debug/deps/validate_delta_rules-ed0fa165c695be27)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_boundary_regressions.rs (target-showboat/debug/deps/validate_domain_discovery_boundary_regressions-636b192300a11ad2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_doc_consistency.rs (target-showboat/debug/deps/validate_domain_discovery_doc_consistency-248cb2e39c56b681)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_rules.rs (target-showboat/debug/deps/validate_domain_discovery_rules-debcbd7f59bd4c1c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target-showboat/debug/deps/validate_rules_extension-0d209ce4ff43407a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target-showboat/debug/deps/validate_tracking_rules-1ff53312e07c4e16)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/wiki_install.rs (target-showboat/debug/deps/wiki_install-adb45ab3585eaca7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target-showboat/debug/deps/worktree_ensure_e2e-ef2730433b855d53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```

```bash
CARGO_TARGET_DIR=/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/target-showboat cargo test --manifest-path /Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/Cargo.toml -p ito-core legacy_coordination
```

```output
   Compiling libc v0.2.186
   Compiling quote v1.0.45
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.24
   Compiling cfg-if v1.0.4
   Compiling memchr v2.8.1
   Compiling version_check v0.9.5
   Compiling bitflags v2.11.1
   Compiling serde_core v1.0.228
   Compiling once_cell v1.21.4
   Compiling rustix v1.1.4
   Compiling zerocopy v0.8.48
   Compiling object v0.37.3
   Compiling serde v1.0.228
   Compiling core-foundation-sys v0.8.7
   Compiling itoa v1.0.18
   Compiling regex-syntax v0.8.10
   Compiling adler2 v2.0.1
   Compiling getrandom v0.4.2
   Compiling gimli v0.32.3
   Compiling miniz_oxide v0.8.9
   Compiling ahash v0.8.12
   Compiling generic-array v0.14.7
   Compiling find-msvc-tools v0.1.9
   Compiling aho-corasick v1.1.4
   Compiling owo-colors v4.3.0
   Compiling shlex v1.3.0
   Compiling zmij v1.0.21
   Compiling rustc-demangle v0.1.27
   Compiling cc v1.2.62
   Compiling regex-automata v0.4.14
   Compiling typenum v1.20.0
   Compiling syn v2.0.117
   Compiling pkg-config v0.3.33
   Compiling addr2line v0.25.1
   Compiling autocfg v1.5.1
   Compiling getrandom v0.3.4
   Compiling errno v0.3.14
   Compiling unicode-width v0.2.2
   Compiling vcpkg v0.2.15
   Compiling unicode-linebreak v0.1.5
   Compiling is_ci v1.2.0
   Compiling serde_json v1.0.150
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.2
   Compiling libsqlite3-sys v0.28.0
   Compiling num-traits v0.2.19
   Compiling foldhash v0.2.0
   Compiling supports-unicode v3.0.0
   Compiling unicode-width v0.1.14
   Compiling httparse v1.10.1
   Compiling supports-hyperlinks v3.2.0
   Compiling log v0.4.30
   Compiling hashbrown v0.17.1
   Compiling backtrace v0.3.76
   Compiling terminal_size v0.4.4
   Compiling backtrace-ext v0.2.1
   Compiling security-framework-sys v2.17.0
   Compiling core-foundation v0.10.1
   Compiling schemars v0.8.22
   Compiling zeroize v1.8.2
   Compiling native-tls v0.2.18
   Compiling fastrand v2.4.1
   Compiling bytes v1.11.1
   Compiling thiserror v2.0.18
   Compiling base64ct v1.8.3
   Compiling bstr v1.12.1
   Compiling pem-rfc7468 v1.0.0
   Compiling serde_derive_internals v0.29.1
   Compiling regex v1.12.3
   Compiling http v1.4.1
   Compiling tempfile v3.27.0
   Compiling security-framework v3.7.0
   Compiling serde_derive v1.0.228
   Compiling miette-derive v7.6.0
   Compiling schemars_derive v0.8.22
   Compiling thiserror-impl v2.0.18
   Compiling hashbrown v0.14.5
   Compiling ppv-lite86 v0.2.21
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling rand_core v0.9.5
   Compiling include_dir_macros v0.7.4
   Compiling miette v7.6.0
   Compiling grep-matcher v0.1.8
   Compiling iana-time-zone v0.1.65
   Compiling encoding_rs v0.8.35
   Compiling equivalent v1.0.2
   Compiling fallible-streaming-iterator v0.1.9
   Compiling dyn-clone v1.0.20
   Compiling base64 v0.22.1
   Compiling fallible-iterator v0.3.0
   Compiling smallvec v1.15.1
   Compiling hashlink v0.9.1
   Compiling indexmap v2.14.0
   Compiling ureq-proto v0.6.0
   Compiling chrono v0.4.44
   Compiling include_dir v0.7.4
   Compiling rand_chacha v0.9.0
   Compiling ito-common v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-common)
   Compiling digest v0.10.7
   Compiling tracing-attributes v0.1.31
   Compiling der v0.8.0
   Compiling rustls-pki-types v1.14.1
   Compiling uuid v1.23.1
   Compiling encoding_rs_io v0.1.7
   Compiling cpufeatures v0.2.17
   Compiling memmap2 v0.9.10
   Compiling tracing-core v0.1.36
   Compiling ryu v1.0.23
   Compiling unsafe-libyaml v0.2.11
   Compiling utf8-zero v0.8.1
   Compiling percent-encoding v2.3.2
   Compiling same-file v1.0.6
   Compiling pin-project-lite v0.2.17
   Compiling grep-searcher v0.1.16
   Compiling minijinja v1.0.22
   Compiling ureq v3.3.0
   Compiling walkdir v2.5.0
   Compiling serde_yaml v0.9.34+deprecated
   Compiling assert-struct-macros v0.2.0
   Compiling tracing v0.1.44
   Compiling sha2 v0.10.9
   Compiling rand v0.9.4
   Compiling grep-regex v0.1.14
   Compiling ito-config v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-config)
   Compiling diffy v0.5.0
   Compiling glob v0.3.3
   Compiling hex v0.4.3
   Compiling filetime v0.2.29
   Compiling ito-templates v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-templates)
   Compiling assert-struct v0.2.0
   Compiling rusqlite v0.31.0
   Compiling ito-domain v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-domain)
   Compiling ito-core v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-core)
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 06s
     Running unittests src/lib.rs (target-showboat/debug/deps/ito_core-f94ddd81afa69f58)

running 13 tests
test legacy_coordination::legacy_coordination_tests::classification_serialization_has_one_stable_tagged_shape ... ok
test legacy_coordination::legacy_coordination_tests::broken_expected_link_is_still_legacy_evidence ... ok
test legacy_coordination::legacy_coordination_tests::wrong_link_target_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::configured_worktree_storage_is_legacy_even_without_links ... ok
test legacy_coordination::legacy_coordination_tests::worktree_config_with_materialized_directories_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::expected_coordination_links_are_legacy ... ok
test legacy_coordination::legacy_coordination_tests::disabled_storage_with_no_managed_paths_is_absent ... ok
test legacy_coordination::legacy_coordination_tests::partial_real_directory_materialization_is_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::inspection_does_not_change_files_or_links ... ok
test legacy_coordination::legacy_coordination_tests::residual_managed_gitignore_marker_is_ambiguous_after_materialization ... ok
test legacy_coordination::legacy_coordination_tests::real_directories_with_embedded_storage_are_main_compatible ... ok
test legacy_coordination::legacy_coordination_tests::partial_coordination_gitignore_entries_are_ambiguous ... ok
test legacy_coordination::legacy_coordination_tests::mixed_link_and_non_empty_real_directory_is_ambiguous ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 730 filtered out; finished in 0.05s

     Running tests/archive.rs (target-showboat/debug/deps/archive-dfc04dd76b886ddf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target-showboat/debug/deps/audit_mirror-6dd1472ff1603cbe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target-showboat/debug/deps/audit_storage-f211900d01a08e99)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target-showboat/debug/deps/backend_archive-d28103fb364261b0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target-showboat/debug/deps/backend_auth-0410f61f1d641fc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target-showboat/debug/deps/backend_auth_service-88a8a2ae9d6c3712)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target-showboat/debug/deps/backend_client_mode-a40d955a657778da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target-showboat/debug/deps/backend_module_repository-68427bdb5aea49fa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target-showboat/debug/deps/backend_sub_module_support-a647e5af5d4c8ec1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target-showboat/debug/deps/change_repository_lifecycle-289502ae25eeb59d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target-showboat/debug/deps/change_repository_orchestrate_metadata-d3c893fb55908635)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target-showboat/debug/deps/change_repository_parity-fb49c1433ce7555f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target-showboat/debug/deps/change_target_resolution_parity-502b6d08def9b4a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target-showboat/debug/deps/coordination_worktree-e5d8b80bc4da60d0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target-showboat/debug/deps/create-85b618baa4d358ce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/distribution.rs (target-showboat/debug/deps/distribution-db95f824ddc73a72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target-showboat/debug/deps/event_forwarding-ab84399b75203c00)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target-showboat/debug/deps/grep_scopes-ab419613daaeca7f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target-showboat/debug/deps/harness_context-df829d645d274599)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target-showboat/debug/deps/harness_opencode-c638bb84de72f309)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target-showboat/debug/deps/harness_streaming-5ebf02004f3f9719)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target-showboat/debug/deps/harness_stub-cda686dbe23d6c45)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target-showboat/debug/deps/import-9f1c49227f5ac8f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target-showboat/debug/deps/io-7e76b6d1a1b3744d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target-showboat/debug/deps/orchestrate_run_state-edd6d0d8fa43b767)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target-showboat/debug/deps/planning_init-230f5cd162c9ab18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/ralph.rs (target-showboat/debug/deps/ralph-0de62592c0fe0294)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target-showboat/debug/deps/repo_index-a8c4ebc8a98df7a9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target-showboat/debug/deps/repo_integrity-37487272a4dfa68c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target-showboat/debug/deps/repo_paths-e4e7a2ee5edf9935)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target-showboat/debug/deps/repository_runtime-af15d63c3d0be33f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target-showboat/debug/deps/repository_runtime_config_validation-a4ba721776f063c6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target-showboat/debug/deps/show-e765750892666d52)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target-showboat/debug/deps/spec_repository_backends-cd8ec10693f8d69c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target-showboat/debug/deps/spec_show_repository-1b7eb8cac80086bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target-showboat/debug/deps/sqlite_archive_mirror-0fc4721021f45a26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target-showboat/debug/deps/sqlite_task_mutations-9ab4b380c3c8f60d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target-showboat/debug/deps/stats-18afc9eaba4a11c2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target-showboat/debug/deps/task_repository_summary-82e9c7c22e7e0146)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target-showboat/debug/deps/tasks_api-dce70601399fd872)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target-showboat/debug/deps/tasks_checkbox_format-02dcf9c5192e8bca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target-showboat/debug/deps/tasks_orchestration-4dc6458c17828eb8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target-showboat/debug/deps/templates_apply_instructions-36fc2c2f66b60a0b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target-showboat/debug/deps/templates_change_status-60b55524446a585f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target-showboat/debug/deps/templates_review_context-1cea0c44d22ab372)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target-showboat/debug/deps/templates_schema_resolution-c3c66a79b2847c37)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target-showboat/debug/deps/templates_schemas_listing-668137b6e72d35ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target-showboat/debug/deps/templates_user_guidance-be58f8ababda5411)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target-showboat/debug/deps/traceability_e2e-75c23d62c01e723f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target-showboat/debug/deps/validate-e28a396cec365936)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target-showboat/debug/deps/validate_delta_rules-ed0fa165c695be27)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_boundary_regressions.rs (target-showboat/debug/deps/validate_domain_discovery_boundary_regressions-636b192300a11ad2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_doc_consistency.rs (target-showboat/debug/deps/validate_domain_discovery_doc_consistency-248cb2e39c56b681)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_domain_discovery_rules.rs (target-showboat/debug/deps/validate_domain_discovery_rules-debcbd7f59bd4c1c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target-showboat/debug/deps/validate_rules_extension-0d209ce4ff43407a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target-showboat/debug/deps/validate_tracking_rules-1ff53312e07c4e16)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/wiki_install.rs (target-showboat/debug/deps/wiki_install-adb45ab3585eaca7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target-showboat/debug/deps/worktree_ensure_e2e-ef2730433b855d53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```

```bash
CARGO_TARGET_DIR=/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/target-showboat cargo test --manifest-path /Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/Cargo.toml -p ito-cli command_intent
```

```output
   Compiling libc v0.2.186
   Compiling serde_core v1.0.228
   Compiling log v0.4.30
   Compiling aho-corasick v1.1.4
   Compiling futures-core v0.3.32
   Compiling rustversion v1.0.22
   Compiling http v1.4.1
   Compiling futures-sink v0.3.32
   Compiling httparse v1.10.1
   Compiling serde_json v1.0.150
   Compiling subtle v2.6.1
   Compiling anyhow v1.0.102
   Compiling tokio-macros v2.7.0
   Compiling fnv v1.0.7
   Compiling ident_case v1.0.1
   Compiling strsim v0.11.1
   Compiling futures-channel v0.3.32
   Compiling smallvec v1.15.1
   Compiling time-core v0.1.8
   Compiling digest v0.10.7
   Compiling num-conv v0.2.2
   Compiling futures-macro v0.3.32
   Compiling tracing-core v0.1.36
   Compiling futures-io v0.3.32
   Compiling futures-task v0.3.32
   Compiling darling_core v0.20.11
   Compiling slab v0.4.12
   Compiling tower-service v0.3.3
   Compiling httpdate v1.0.3
   Compiling anstyle v1.0.14
   Compiling tower-layer v0.3.3
   Compiling mime v0.3.17
   Compiling data-encoding v2.11.0
   Compiling tracing v0.1.44
   Compiling utf8parse v0.2.2
   Compiling thiserror v1.0.69
   Compiling powerfmt v0.2.0
   Compiling futures-util v0.3.32
   Compiling http-body v1.0.1
   Compiling sync_wrapper v1.0.2
   Compiling unicase v2.9.0
   Compiling regex-automata v0.4.14
   Compiling http-body-util v0.1.3
   Compiling cfg_aliases v0.1.1
   Compiling atomic-waker v1.1.2
   Compiling nix v0.28.0
   Compiling deranged v0.5.8
   Compiling mime_guess v2.0.5
   Compiling rusqlite v0.31.0
   Compiling vergen-lib v9.1.0
   Compiling vergen v9.1.0
   Compiling anstyle-parse v1.0.0
   Compiling num_threads v0.1.7
   Compiling form_urlencoded v1.2.2
   Compiling time-macros v0.2.27
   Compiling thiserror-impl v1.0.69
   Compiling cookie v0.18.1
   Compiling errno v0.3.14
   Compiling getrandom v0.4.2
   Compiling socket2 v0.6.3
   Compiling rustix v1.1.4
   Compiling signal-hook-registry v1.4.8
   Compiling mio v1.2.0
   Compiling getrandom v0.3.4
   Compiling cpufeatures v0.2.17
   Compiling rand_core v0.9.5
   Compiling darling_macro v0.20.11
   Compiling backtrace v0.3.76
   Compiling rand_chacha v0.9.0
   Compiling sha1 v0.10.6
   Compiling darling v0.20.11
   Compiling uuid v1.23.1
   Compiling tokio v1.52.3
   Compiling rand v0.9.4
   Compiling derive_builder_core v0.20.2
   Compiling security-framework-sys v2.17.0
   Compiling backtrace-ext v0.2.1
   Compiling core-foundation v0.10.1
   Compiling serde v1.0.228
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle-query v1.1.5
   Compiling security-framework v3.7.0
   Compiling colorchoice v1.0.5
   Compiling tungstenite v0.29.0
   Compiling itoa v1.0.18
   Compiling time v0.3.47
   Compiling derive_builder_macro v0.20.2
   Compiling anstream v1.0.0
   Compiling derive_builder v0.20.2
   Compiling terminal_size v0.4.4
   Compiling tempfile v3.27.0
   Compiling miette v7.6.0
   Compiling serde_urlencoded v0.7.1
   Compiling minijinja v1.0.22
   Compiling bstr v1.12.1
   Compiling regex v1.12.3
   Compiling native-tls v0.2.18
   Compiling ito-common v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-common)
   Compiling schemars v0.8.22
   Compiling serde_path_to_error v0.1.20
   Compiling ito-domain v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-domain)
   Compiling sha2 v0.10.9
   Compiling memmap2 v0.9.10
   Compiling ureq-proto v0.6.0
   Compiling vergen-gitcl v9.1.0
   Compiling axum-core v0.5.6
   Compiling rustix v0.38.44
   Compiling clap_lex v1.1.0
   Compiling matchit v0.8.4
   Compiling shell-words v1.1.1
   Compiling heck v0.5.0
   Compiling ureq v3.3.0
   Compiling clap_derive v4.6.1
   Compiling clap_builder v4.6.0
   Compiling ito-config v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-config)
   Compiling ito-templates v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-templates)
   Compiling serde_yaml v0.9.34+deprecated
   Compiling hyper v1.10.0
   Compiling tokio-tungstenite v0.29.0
   Compiling tower v0.5.3
   Compiling tokio-util v0.7.18
   Compiling grep-searcher v0.1.16
   Compiling filedescriptor v0.8.3
   Compiling grep-regex v0.1.14
   Compiling serial2 v0.2.37
   Compiling http-range-header v0.4.2
   Compiling downcast-rs v1.2.1
   Compiling hyper-util v0.1.20
   Compiling portable-pty v0.9.0
   Compiling clap v4.6.1
   Compiling ito-core v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-core)
   Compiling axum v0.8.9
   Compiling tower-http v0.6.11
   Compiling futures-executor v0.3.32
   Compiling toml_datetime v0.6.11
   Compiling serde_spanned v0.6.9
   Compiling console v0.16.3
   Compiling predicates-core v1.0.10
   Compiling toml_write v0.1.2
   Compiling winnow v0.7.15
   Compiling lazy_static v1.5.0
   Compiling sharded-slab v0.1.7
   Compiling gethostname v0.5.0
   Compiling futures v0.3.32
   Compiling ito-cli v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-cli)
   Compiling matchers v0.2.0
   Compiling tracing-log v0.2.0
   Compiling hmac v0.12.1
   Compiling float-cmp v0.10.0
   Compiling vte v0.14.1
   Compiling thread_local v1.1.9
   Compiling assert_cmd v2.2.2
   Compiling normalize-line-endings v0.3.0
   Compiling difflib v0.4.0
   Compiling termtree v0.5.1
   Compiling nu-ansi-term v0.50.3
   Compiling predicates-tree v1.0.13
   Compiling strip-ansi-escapes v0.2.1
   Compiling tracing-subscriber v0.3.23
   Compiling predicates v3.1.4
   Compiling dialoguer v0.12.0
   Compiling clap_complete v4.6.5
   Compiling ito-logging v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-logging)
   Compiling globset v0.4.18
   Compiling toml_edit v0.22.27
   Compiling serde_ignored v0.1.14
   Compiling wait-timeout v0.2.1
   Compiling similar v2.7.0
   Compiling ito-test-support v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-test-support)
   Compiling filetime v0.2.29
   Compiling insta v1.47.2
   Compiling axum-extra v0.10.3
   Compiling toml v0.8.23
   Compiling ito-backend v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-backend)
   Compiling ito-web v0.1.32 (/Users/jack/Code/withakay/ito/ito-worktrees/031-01_migrate-coordination-state-to-main/ito-rs/crates/ito-web)
warning: enum `CommandIntent` is never used
 --> ito-rs/crates/ito-cli/src/app/legacy_coordination.rs:8:17
  |
8 | pub(crate) enum CommandIntent {
  |                 ^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `command_intent` is never used
  --> ito-rs/crates/ito-cli/src/app/legacy_coordination.rs:19:15
   |
19 | pub(crate) fn command_intent(command: &Commands) -> CommandIntent {
   |               ^^^^^^^^^^^^^^

warning: `ito-cli` (bin "ito") generated 2 warnings
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 15s
     Running unittests src/main.rs (target-showboat/debug/deps/ito-9a1d425ad6848fda)

running 4 tests
test app::legacy_coordination::legacy_coordination_tests::command_intent_fails_closed_for_unknown_external_operations ... ok
test app::legacy_coordination::legacy_coordination_tests::command_intent_marks_migrate_to_main_instruction_as_recovery ... ok
test app::legacy_coordination::legacy_coordination_tests::command_intent_marks_nested_writes_and_instruction_sync_as_mutating ... ok
test app::legacy_coordination::legacy_coordination_tests::command_intent_keeps_diagnostic_and_artifact_reads_read_only ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 97 filtered out; finished in 0.01s

     Running tests/agent_instruction_apply_sync.rs (target-showboat/debug/deps/agent_instruction_apply_sync-b3b7b0bfeb75e060)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_bootstrap.rs (target-showboat/debug/deps/agent_instruction_bootstrap-c776c0314128e691)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_cleanup.rs (target-showboat/debug/deps/agent_instruction_cleanup-0c8b076c7849e1ab)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target-showboat/debug/deps/agent_instruction_context-618c068e090581b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_memory.rs (target-showboat/debug/deps/agent_instruction_memory-0855da4282af2125)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/agent_instruction_orchestrate.rs (target-showboat/debug/deps/agent_instruction_orchestrate-8d4aa1ff437bebc0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_repo_sweep.rs (target-showboat/debug/deps/agent_instruction_repo_sweep-81f9a535784f4025)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target-showboat/debug/deps/agent_instruction_worktrees-1893629b098b04a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target-showboat/debug/deps/aliases-5aebda41f37b7125)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target-showboat/debug/deps/archive_completed-781071a048df46c2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/archive_remote_mode.rs (target-showboat/debug/deps/archive_remote_mode-e497f4f58aedddf9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/archive_smoke.rs (target-showboat/debug/deps/archive_smoke-d49354b5c0510d5c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/artifact_mutations.rs (target-showboat/debug/deps/artifact_mutations-d346993a8c679ab0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_more.rs (target-showboat/debug/deps/audit_more-9b4bb3320771348b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target-showboat/debug/deps/audit_remote_mode-de7225e5009273c4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target-showboat/debug/deps/backend_import-4bdede5e77c25b19)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/backend_qa_walkthrough.rs (target-showboat/debug/deps/backend_qa_walkthrough-f180e9dd29a34dcf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target-showboat/debug/deps/backend_serve-215408a96d19c7eb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_status_more.rs (target-showboat/debug/deps/backend_status_more-94d6db80983fea8d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target-showboat/debug/deps/cli_smoke-8f0eb741674210f7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/cli_snapshots.rs (target-showboat/debug/deps/cli_snapshots-5fb614238a7aadc9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target-showboat/debug/deps/config_more-9bcc9b6696d9a815)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/coverage_smoke.rs (target-showboat/debug/deps/coverage_smoke-d3983f8650b975f7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target-showboat/debug/deps/create_more-c740581fbab3b69f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target-showboat/debug/deps/grep_more-3a57ddbe18f487d9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target-showboat/debug/deps/help-c4ce5a3367ab8d9b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_agent_activation.rs (target-showboat/debug/deps/init_agent_activation-e722eb480d4fee24)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/init_cleanup.rs (target-showboat/debug/deps/init_cleanup-ab89e472368f7551)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/init_coordination.rs (target-showboat/debug/deps/init_coordination-19e563aa180aa930)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target-showboat/debug/deps/init_gitignore_session_json-9899e5e6fdea8941)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target-showboat/debug/deps/init_more-9e69a17239809717)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s

     Running tests/init_obsolete_cleanup.rs (target-showboat/debug/deps/init_obsolete_cleanup-2e09b2ab208602a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/init_rendering.rs (target-showboat/debug/deps/init_rendering-5c3df4147640e8e0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/init_tmux.rs (target-showboat/debug/deps/init_tmux-c332e5166c5e9b29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_upgrade_more.rs (target-showboat/debug/deps/init_upgrade_more-7c5bea377812b153)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_worktree_flags.rs (target-showboat/debug/deps/init_worktree_flags-957e085f330f32fe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target-showboat/debug/deps/instructions_more-1d0ba671e4cd9ca1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

     Running tests/list_archive.rs (target-showboat/debug/deps/list_archive-aa0376ed9cc7588b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target-showboat/debug/deps/list_regression-284ebf66c2c8aa30)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target-showboat/debug/deps/misc_more-0f669363034a5217)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s

     Running tests/new_more.rs (target-showboat/debug/deps/new_more-9240046c0ecce799)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target-showboat/debug/deps/parity_help_version-4e4712850b36fb38)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target-showboat/debug/deps/parity_tasks-f6de83a61f2459cc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target-showboat/debug/deps/path_more-9aa4c7f3b352f295)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/plan_state_more.rs (target-showboat/debug/deps/plan_state_more-ee4729dbc9b871b3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/ralph_smoke.rs (target-showboat/debug/deps/ralph_smoke-fdbbb0712e145cee)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target-showboat/debug/deps/serve_more-99e7906ed102de10)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target-showboat/debug/deps/show_specs_bundle-e1d73e75dbcf9916)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target-showboat/debug/deps/show_specs_remote_mode-1a764b000cd2613e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target-showboat/debug/deps/source_file_size-dc0b7ba055e25a1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target-showboat/debug/deps/stats-91229370b60b9e39)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target-showboat/debug/deps/tasks_more-fd5f68541f8ec04f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target-showboat/debug/deps/tasks_remote_mode-05cd043c3d6f43f6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target-showboat/debug/deps/templates_schemas_export-270a93808d1ed7c6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/trace_more.rs (target-showboat/debug/deps/trace_more-d0055086bcbe54ea)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/update_marker_scoped.rs (target-showboat/debug/deps/update_marker_scoped-02d6ebdd36a3d1af)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target-showboat/debug/deps/update_smoke-57f275627d326750)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/user_guidance_injection.rs (target-showboat/debug/deps/user_guidance_injection-40a2d37551371572)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target-showboat/debug/deps/validate_more-c14e851e95d136e9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/validate_repo_cli.rs (target-showboat/debug/deps/validate_repo_cli-abbc2035f3ec4552)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target-showboat/debug/deps/view_proposal-db3bc3fa65e0c9c8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/worktree_validate.rs (target-showboat/debug/deps/worktree_validate-aaf31f22f1a0bb4c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```
