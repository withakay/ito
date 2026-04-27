# Task 1.1: Manifesto CLI Surface

*2026-04-27T10:20:00Z by Showboat 0.6.1*
<!-- showboat-id: bbbb98a1-418a-4ed8-bf1a-fc640499081e -->

Added the manifesto instruction artifact to the CLI/help surface, added manifesto-specific selectors, and verified default/text/json behavior plus the light-variant operation guard.

```bash
cargo test -p ito-cli --test help --test instructions_more
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.19s
     Running tests/help.rs (target/debug/deps/help-aa42fd8589824328)

running 7 tests
test help_prints_usage ... ok
test agent_instruction_help_shows_instruction_details ... ok
test help_shows_navigation_footer ... ok
test help_all_global_flag_works ... ok
test dash_h_help_matches_dash_dash_help ... ok
test help_all_shows_complete_reference ... ok
test help_all_json_outputs_valid_json ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-9654ca94826e629f)

running 17 tests
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_review_renders_review_template ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_manifesto_rejects_operation_for_light_variant ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_manifesto_uses_default_variant_and_profile ... ok
test agent_instruction_manifesto_json_includes_resolved_defaults ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.36s

```
