# Wave 2: Manifesto Rendering and Composition

*2026-04-27T21:05:31Z by Showboat 0.6.1*
<!-- showboat-id: b2634656-c2fc-4b8e-916a-9a11e081e48c -->

Implemented light/full manifesto rendering, redacted config/worktree capsules, and full-mode embedding of existing instruction bodies with operation gating.

```bash
cargo test -p ito-templates instructions_tests && cargo test -p ito-cli --test instructions_more
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-43511d335e81e446)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 86 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-4be66a48dfefacf5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-89f6f29b2c677eb1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-c542d94a0d9bbd52)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-354bb8adddb77ade)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-d45bf1384b899f95)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-ea6b170a0185265d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/instructions_more.rs (target/debug/deps/instructions_more-9654ca94826e629f)

running 24 tests
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_manifesto_redacts_explicit_coordination_path ... ok
test agent_instruction_manifesto_full_variant_rejects_incompatible_operation ... ok
test agent_instruction_manifesto_json_includes_resolved_defaults ... ok
test agent_instruction_manifesto_full_variant_renders_full_section ... ok
test agent_instruction_manifesto_full_variant_embeds_requested_proposal_instruction ... ok
test agent_instruction_manifesto_change_scope_json_reports_state ... ok
test agent_instruction_manifesto_change_scope_includes_change_state ... ok
test agent_instruction_manifesto_full_variant_embeds_allowed_default_set ... ok
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_manifesto_rejects_operation_for_light_variant ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok
test agent_instruction_review_renders_review_template ... ok
test agent_instruction_manifesto_uses_default_variant_and_profile ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.97s

```

```bash
target/debug/ito agent instruction manifesto --change 000-01_test-change --variant full --operation proposal
```

```output

✖ Error: Requested operation 'proposal' is not allowed for state 'finished' and profile 'full'.
```
