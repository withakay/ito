# Task 1.3: Agent Template Activation Contract

*2026-04-29T18:20:24Z by Showboat 0.6.1*
<!-- showboat-id: bfd591c5-607b-4db8-836a-87282f46f5ee -->

Added explicit activation metadata to generated Ito agent templates so direct entrypoints declare activation: direct while delegated role agents declare activation: delegated.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-templates agent_templates_declare_activation_contract -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-7eaa5889a2394c40)

running 1 test
test agent_surface_tests::agent_templates_declare_activation_contract ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 0.00s

     Running tests/instructions_apply_memory.rs (target/debug/deps/instructions_apply_memory-d1e1807ce87f211a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-7a5705c6aff70672)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-b0565b06adac7694)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-95514587e0df9f18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-bc4ea5e74b0d0fe5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-36770e8c31892375)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-825fa89e3cbc9b79)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test init_agent_activation -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.30s
     Running tests/init_agent_activation.rs (target/debug/deps/init_agent_activation-e20dd9c42c9d9078)

running 1 test
test init_update_with_tools_all_preserves_agent_activation_contract ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s

```

```bash
make check-max-lines
```

```output
python3 "ito-rs/tools/check_max_lines.py" --max-lines "1000" --root "ito-rs" --baseline "ito-rs/tools/max_lines_baseline.txt"
Warning: 8 Rust files exceed limits but remain within baseline:
  - ito-rs/crates/ito-core/src/ralph/runner.rs: 1426 (baseline 1426)
  - ito-rs/crates/ito-cli/tests/ralph_smoke.rs: 1408 (baseline 1408)
  - ito-rs/crates/ito-core/src/installers/mod.rs: 1376 (baseline 1380)
  - ito-rs/crates/ito-config/src/config/types.rs: 1371 (baseline 1371)
  - ito-rs/crates/ito-cli/tests/init_more.rs: 1311 (baseline 1336)
  - ito-rs/crates/ito-core/src/coordination_worktree.rs: 1283 (baseline 1283)
  - ito-rs/crates/ito-core/tests/ralph.rs: 1279 (baseline 1279)
  - ito-rs/crates/ito-templates/src/instructions_tests.rs: 1235 (baseline 1414)
Warning: 14 Rust files over soft limit (1000 lines):
  - ito-rs/crates/ito-cli/src/app/instructions.rs: 1199 (consider splitting)
  - ito-rs/crates/ito-cli/src/cli.rs: 1199 (consider splitting)
  - ito-rs/crates/ito-templates/src/lib.rs: 1170 (consider splitting)
  - ito-rs/crates/ito-core/src/create/mod.rs: 1131 (consider splitting)
  - ito-rs/crates/ito-core/src/validate/mod.rs: 1129 (consider splitting)
  - ito-rs/crates/ito-domain/src/tasks/parse.rs: 1097 (consider splitting)
  - ito-rs/crates/ito-core/src/config.rs: 1077 (consider splitting)
  - ito-rs/crates/ito-core/src/tasks.rs: 1075 (consider splitting)
  - ito-rs/crates/ito-cli/src/commands/tasks.rs: 1061 (consider splitting)
  - ito-rs/crates/ito-core/src/coordination_worktree_tests.rs: 1039 (consider splitting)
  - ito-rs/crates/ito-core/src/backend_http.rs: 1025 (consider splitting)
  - ito-rs/crates/ito-core/src/templates/mod.rs: 1015 (consider splitting)
  - ito-rs/crates/ito-core/tests/validate.rs: 1010 (consider splitting)
  - ito-rs/crates/ito-core/src/audit/mirror.rs: 1003 (consider splitting)
```
