# Installer line-limit cleanup

*2026-03-22T11:19:53Z by Showboat 0.6.1*
<!-- showboat-id: 6f9c4059-b404-4502-a4b8-a26e68c8ee49 -->

Trimmed documentation-only content in ito-core installers to get the module back under the 1200-line repo limit without changing behavior.

```bash
wc -l ito-rs/crates/ito-core/src/installers/mod.rs
```

```output
    1156 ito-rs/crates/ito-core/src/installers/mod.rs
```

```bash
cargo test -p ito-core --test distribution
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.06s
     Running tests/distribution.rs (target/debug/deps/distribution-8c94ad7e63563442)

running 11 tests
test codex_manifests_includes_bootstrap_and_skills ... ok
test claude_manifests_includes_hooks_and_skills ... ok
test github_manifests_includes_skills_and_commands ... ok
test opencode_manifests_includes_plugin_and_skills ... ok
test install_manifests_make_tmux_skill_scripts_executable ... ok
test install_manifests_renders_worktree_skill_with_context ... ok
test install_manifests_renders_worktree_skill_enabled ... ok
test install_manifests_creates_parent_directories ... ok
test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
test install_manifests_writes_files_to_disk ... ok
test all_manifests_use_embedded_assets ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

```

```bash
cargo test -p ito-templates tmux_skill_and_scripts_are_embedded
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-cb3052cb4f694ff3)

running 1 test
test tests::tmux_skill_and_scripts_are_embedded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-6a50b3a39df3821a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-bdfe2fbaa8fbf4b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-647c504bf051f08d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```
