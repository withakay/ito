# Task 1.1: Add tools.tmux config defaults

*2026-03-22T10:31:52Z by Showboat 0.6.1*
<!-- showboat-id: f4a9df16-b06e-40e5-8686-756cf1e2fd3f -->

Added a top-level tools namespace to Ito config with tools.tmux.enabled defaulting to true, and covered it with config + schema tests.

```bash
cd ito-rs && cargo test -p ito-config 2>&1
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/ito_config-fc6a4d513e34c55e)

running 29 tests
test config::tests::global_config_path_prefers_xdg ... ok
test config::tests::ito_config_dir_prefers_xdg ... ok
test config::schema::tests::schema_contains_expected_sections ... ok
test config::tests::audit_mirror_defaults_exist_in_cascading_config ... ok
test config::tests::tools_tmux_enabled_defaults_to_true_in_cascading_config ... ok
test context::tests::resolve_with_ctx_sets_none_when_ito_dir_is_missing ... ok
test config::tests::coordination_branch_defaults_exist_in_cascading_config ... ok
test context::tests::resolve_with_ctx_sets_ito_path_when_directory_exists ... ok
test config::tests::load_global_ito_config_returns_defaults_when_no_file ... ok
test config::tests::worktrees_config_has_defaults_in_cascading_config ... ok
test ito_dir::tests::sanitize_rejects_path_separators_and_overlong_values ... ok
test output::tests::no_color_env_set_matches_ts_values ... ok
test output::tests::resolve_interactive_respects_cli_and_env ... ok
test output::tests::resolve_ui_options_combines_sources ... ok
test ito_dir::tests::get_ito_dir_name_defaults_to_dot_ito ... ok
test config::tests::coordination_branch_defaults_can_be_overridden ... ok
test context::tests::resolve_with_ctx_uses_explicit_config_context_paths ... ok
test config::tests::legacy_worktree_default_branch_key_migrates ... ok
test config::tests::cascading_project_config_ignores_schema_ref_key ... ok
test config::tests::audit_mirror_defaults_can_be_overridden ... ok
test config::tests::legacy_worktree_local_files_key_migrates ... ok
test config::tests::new_worktree_keys_take_precedence_over_legacy ... ok
test ito_dir::tests::invalid_repo_project_path_falls_back_to_default ... ok
test config::tests::cascading_project_config_ignores_invalid_json_sources ... ok
test ito_dir::tests::dot_repo_config_overrides_repo_config ... ok
test ito_dir::tests::get_ito_path_normalizes_dotdot_segments ... ok
test config::tests::load_global_ito_config_reads_backend_server_auth ... ok
test ito_dir::tests::repo_config_overrides_global_config ... ok
test config::tests::cascading_project_config_merges_sources_in_order_with_scalar_override ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests ito_config

running 4 tests
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::absolutize_and_normalize_lossy (line 112) ... ignored
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::get_ito_path_fs (line 59) - compile ... ok
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::lexical_normalize (line 129) ... ok
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::absolutize_and_normalize (line 89) ... ok

test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.44s; merged doctests compilation took 0.17s
```
