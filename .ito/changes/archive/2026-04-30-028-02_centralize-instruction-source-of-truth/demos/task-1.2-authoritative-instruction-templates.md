# Task 1.2: Authoritative Instruction Templates

*2026-04-29T15:08:56Z by Showboat 0.6.1*
<!-- showboat-id: 5665640e-e792-4aaa-a520-d51f4ee837ba -->

Expanded the orchestrate instruction artifact with source-of-truth precedence, direct coordinator activation, delegated role agents, gate planning, run state, remediation, and resume behavior. Added provider-operation guidance to memory instruction rendering and shared harness detection between audit context and instruction output.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test agent_instruction_orchestrate -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.52s
     Running tests/agent_instruction_orchestrate.rs (target/debug/deps/agent_instruction_orchestrate-84ba53fcb62ee116)

running 9 tests
test orchestrate_requires_orchestrate_md ... ok
test orchestrate_succeeds_when_orchestrate_md_exists ... ok
test orchestrate_policy_identifies_direct_and_delegated_surfaces ... ok
test orchestrate_json_output_has_correct_artifact_id ... ok
test orchestrate_reports_unknown_harness_without_session_env ... ok
test orchestrate_includes_detected_opencode_harness_context ... ok
test orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter ... ok
test orchestrate_surfaces_recommended_skills_from_preset ... ok
test orchestrate_uses_canonical_harness_detection_order ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.75s

```

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test agent_instruction_memory -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.38s
     Running tests/agent_instruction_memory.rs (target/debug/deps/agent_instruction_memory-347868ea0bb4393d)

running 14 tests
test agent_instruction_help_lists_memory_artifacts ... ok
test memory_query_skill_branch_emits_structured_inputs ... ok
test memory_query_renders_not_configured_when_only_capture_set ... ok
test memory_search_skill_branch_emits_structured_inputs ... ok
test memory_search_not_configured_branch_renders_setup_guidance ... ok
test memory_search_command_branch_overrides_limit_when_supplied ... ok
test memory_capture_not_configured_branch_renders_setup_guidance ... ok
test memory_capture_command_branch_renders_executable_command_line ... ok
test memory_query_command_branch_substitutes_query ... ok
test memory_capture_skill_branch_emits_structured_inputs ... ok
test memory_capture_renders_skill_when_only_capture_configured ... ok
test memory_search_command_branch_substitutes_query_and_default_limit ... ok
test memory_search_requires_query_flag ... ok
test memory_query_not_configured_branch_renders_setup_guidance ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.33s

```

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy --all-targets -- -D warnings
```

```output
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
```
