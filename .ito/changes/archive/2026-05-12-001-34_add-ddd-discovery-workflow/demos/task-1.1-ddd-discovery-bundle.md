# Task 1.1: DDD Discovery Bundle

*2026-05-11T19:09:32Z by Showboat 0.6.1*
<!-- showboat-id: f0c3f8d9-a5c5-4af3-ac67-6c2235aa7000 -->

```bash
cargo test -p ito-templates 2>&1 | tail -40
```

```output
test template_markdown_is_well_formed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-36770e8c31892375)

running 2 tests
test user_guidance_template_exists_and_has_markers ... ok
test user_prompt_stub_templates_exist ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-825fa89e3cbc9b79)

running 8 tests
test agents_md_disabled ... ok
test skill_checkout_subdir ... ok
test skill_bare_control_siblings ... ok
test skill_checkout_siblings ... ok
test agents_md_checkout_siblings ... ok
test skill_disabled ... ok
test agents_md_bare_control_siblings ... ok
test agents_md_checkout_subdir ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.47s; merged doctests compilation took 0.16s
```

```bash
rg -n 'Domain Discovery Summary|Discovery Depth Gate|Domain Discovery Gate|Domain-Grill Mode|Boundary-smell' ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md ito-rs/crates/ito-templates/assets/skills/ito-proposal-intake/SKILL.md ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2
```

```output
ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2:16:## Before Schema Selection: Domain Discovery Gate
ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2:53:## Domain Discovery Summary
ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md:24:## Discovery Depth Gate
ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md:72:## Domain-Grill Mode
ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md:88:## Domain Discovery Summary
ito-rs/crates/ito-templates/assets/skills/ito-proposal-intake/SKILL.md:36:## Domain Discovery Gate
ito-rs/crates/ito-templates/assets/skills/ito-proposal-intake/SKILL.md:106:## Domain Discovery Summary
```

After review, aligned the handoff label to Translation boundaries and tightened ADR/documentation-capture criteria.

```bash
cargo test -p ito-templates 2>&1 | tail -25
```

```output
test skill_disabled ... ok
test agents_md_disabled ... ok
test skill_checkout_siblings ... ok
test skill_bare_control_siblings ... ok
test agents_md_bare_control_siblings ... ok
test agents_md_checkout_subdir ... ok
test agents_md_checkout_siblings ... ok
test skill_checkout_subdir ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.42s; merged doctests compilation took 0.16s
```
