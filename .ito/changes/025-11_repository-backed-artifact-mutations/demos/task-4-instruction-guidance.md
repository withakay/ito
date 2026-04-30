# Task 4: Instruction and harness guidance

*2026-04-30T22:53:20Z by Showboat 0.6.1*
<!-- showboat-id: 981d7dca-842b-4aab-95f4-c85435385e8e -->

Updated artifact instruction templates, default project guidance, source agent templates, and installed harness copies so Ito active-work artifacts route through ito patch / ito write while ordinary repository files remain editable with normal tools.

```bash
cd ito-rs && cargo test -p ito-templates --lib
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/target/debug/deps/ito_templates-b947987ebfdc2418)

running 84 tests
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::render_template_replaces_variant ... ok
test agents::tests::render_template_replaces_model ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::orchestrate_template_renders ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test tests::default_home_files_returns_a_vec ... ok
test tests::agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_project_template_strict_on_undefined ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test tests::default_project_files_contains_expected_files ... ok
test instructions::tests::artifact_template_routes_all_active_artifacts_through_ito_mutation_commands ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
test tests::default_project_includes_orchestrate_user_prompt ... ok
test tests::every_shipped_agent_has_ito_prefix ... ok
test tests::every_shipped_command_has_ito_prefix ... ok
test tests::every_shipped_skill_has_ito_prefix ... ok
test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
test tests::extract_managed_block_rejects_inline_markers ... ok
test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
test tests::extract_managed_block_returns_inner_content ... ok
test tests::fix_and_feature_commands_are_embedded ... ok
test tests::get_preset_file_returns_contents ... ok
test tests::get_schema_file_returns_contents ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
test tests::normalize_ito_dir_prefixes_dot ... ok
test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
test tests::orchestrate_skills_and_command_are_embedded ... ok
test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
test tests::presets_files_contains_orchestrate_builtins ... ok
test tests::proposal_intake_and_routing_skills_are_embedded ... ok
test tests::render_bytes_preserves_non_utf8 ... ok
test tests::render_bytes_returns_borrowed_when_no_rewrite_needed ... ok
test tests::render_bytes_rewrites_dot_ito_paths ... ok
test tests::render_rel_path_rewrites_ito_prefix ... ok
test tests::schema_files_contains_builtins ... ok
test tests::stamp_version_canonical_with_leading_whitespace_is_rewritten ... ok
test tests::stamp_version_handles_crlf_line_endings ... ok
test tests::stamp_version_handles_prerelease_semver ... ok
test tests::stamp_version_idempotent_on_canonical_match ... ok
test tests::stamp_version_idempotent_on_canonical_with_trailing_whitespace ... ok
test tests::stamp_version_inserts_when_missing ... ok
test tests::stamp_version_noop_without_marker ... ok
test tests::stamp_version_preserves_frontmatter ... ok
test tests::stamp_version_preserves_trailing_content ... ok
test tests::stamp_version_rewrites_older_version ... ok
test tests::stamp_version_rewrites_spaced_form_to_canonical ... ok
test tests::stamp_version_round_trip_on_real_skill ... ok
test tests::tmux_skill_and_scripts_are_embedded ... ok

test result: ok. 84 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

```bash
cd ito-rs && cargo test -p ito-cli --test instructions_more
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.31s
     Running tests/instructions_more.rs (/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/target/debug/deps/instructions_more-a9e527c8dacd562c)

running 15 tests
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_review_renders_review_template ... ok
test agent_instruction_artifact_commands_route_specs_and_tasks_through_mutation_cli ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.84s

```

```bash
target/debug/ito agent instruction specs --change 025-11_repository-backed-artifact-mutations
```

````output
- Generating instructions...
<artifact id="specs" change="025-11_repository-backed-artifact-mutations" schema="spec-driven">


<task>
Create the specs artifact for change "025-11_repository-backed-artifact-mutations".
Detailed specifications for the change
</task>

<context>
Read these files for context before creating this artifact:

<dependency id="proposal" status="done">
  <path>/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/.ito/changes/025-11_repository-backed-artifact-mutations/proposal.md</path>
  <description>Initial proposal document outlining the change</description>
</dependency>
</context>


<user_guidance>
` marker.

<!-- ITO:END -->

## Project Guidance

### Rust Code Quality

After modifying Rust code, dispatch these subagents **in parallel**:
- @code-simplifier - Refactors for clarity per `.ito/user-rust-style.md`
- @documentation-police - Ensures public APIs have useful docs
- @rust-code-reviewer - Checks for idiomatic usage, error handling, and best practices

Then run `make check` to verify.

### Running test and checks

Always use the test-with-subagent skill for running builds, tests and checks.

### Commits

Make small, focused commits with clear messages.
Regularly use the `ito-commit` skill for conventional commits aligned with the project's commit message guidelines.
IF you have to do more work to make changes that don't break the build whilst remaining small and focused, so be it.

### Subagent Collaboration

Subagents are first-class tools in this repo. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.

Diversity is good: for non-trivial changes, get at least two independent review passes (for example: `@rust-code-reviewer` + `@codex-review`).

Commonly useful subagents:

- `@explore` - fast codebase navigation/search
- `@test-runner` - runs `make test` / `make check` with curated output
- `@rust-quality-checker` - Rust style/idioms/conventions checks
- `@rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)
- `@rust-test-engineer` - test strategy and coverage design
- `@codex-review` - diff review for correctness and edge cases
- `@documentation-police` - docs coverage/quality
- `@code-simplifier` - refactor for clarity and maintainability
- `@code-quality-squad` - parallel Rust quality workflows
- `@multi-agent` - explore multiple approaches and synthesize

### Showboat Demo Documents

This repo uses [Showboat](https://github.com/simonw/showboat) to have agents produce
executable demo documents that prove their work. Showboat builds markdown files incrementally
via CLI commands (`init`, `note`, `exec`, `image`, `pop`) that capture real command output --
this prevents agents from fabricating results.

- Available via `uvx showboat` (no install required)
- Run `uvx showboat --help` for full CLI reference
- See `.ito/user-prompts/apply.md` for detailed apply-phase usage
- Demo docs go in `.ito/changes/<change-id>/demos/`
- **Never edit showboat markdown directly** -- always use the CLI commands
</user_guidance>


<guidance>
- Add `Tags` only when the requirement needs an explicit facet such as `ui` or `stateful`.
- Add `Contract Refs` when the requirement depends on an external interface instead of copying large contract snippets inline.
- Use `Rules / Invariants` and `State Transitions` only when they materially clarify behavior; leave them out when they would be empty noise.
</guidance>


<output>
Mutate this Ito active-work artifact through the repository-backed CLI, not direct file edits.
Recommended command: `ito write change 025-11_repository-backed-artifact-mutations spec <capability>` for each spec delta, or `ito patch change 025-11_repository-backed-artifact-mutations spec <capability>` for targeted diffs.
Repository projection path: /Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/.ito/changes/025-11_repository-backed-artifact-mutations/specs/**/*.md
Use ordinary file-edit tools only for non-Ito project/code files.
</output>

<instruction>
Create specification files that define WHAT the system should do.

Create one spec file per capability/feature area in specs/<name>/spec.md.

Delta operations (use ## headers):
- **ADDED Requirements**: New capabilities
- **MODIFIED Requirements**: Changed behavior - MUST include full updated content
- **REMOVED Requirements**: Deprecated features - MUST include **Reason** and **Migration**
- **RENAMED Requirements**: Name changes only - use FROM:/TO: format

Format requirements:
- Each requirement: `### Requirement: <name>` followed by description
- Use SHALL/MUST for normative requirements (avoid should/may)
- Each scenario: `#### Scenario: <name>` with WHEN/THEN format
- **CRITICAL**: Scenarios MUST use exactly 4 hashtags (`####`). Using 3 hashtags or bullets will fail silently.
- Every requirement MUST have at least one scenario.

MODIFIED requirements workflow:
1. Locate the existing requirement in ito/specs/<capability>/spec.md
2. Copy the ENTIRE requirement block (from `### Requirement:` through all scenarios)
3. Paste under `## MODIFIED Requirements` and edit to reflect new behavior
4. Ensure header text matches exactly (whitespace-insensitive)

Common pitfall: Using MODIFIED with partial content loses detail at archive time.
If adding new concerns without changing existing behavior, use ADDED instead.

Example:
```
## ADDED Requirements

### Requirement: User can export data
The system SHALL allow users to export their data in CSV format.

#### Scenario: Successful export
- **WHEN** user clicks "Export" button
- **THEN** system downloads a CSV file with all user data

## REMOVED Requirements

### Requirement: Legacy export
**Reason**: Replaced by new export system
**Migration**: Use new export endpoint at /api/v2/export
```

Specs should be testable - each scenario is a potential test case.
</instruction>


<template>
<!-- ITO:START -->
## ADDED Requirements

### Requirement: <!-- requirement name -->

<!-- requirement text -->

- **Requirement ID**: <!-- capability:requirement-name -->

<!-- OPTIONAL: Add tags when validators or readers need extra context such as behavior, ui, or stateful. -->
- **Tags**: <!-- behavior, ui -->

<!-- OPTIONAL: Reference external contracts instead of copying them inline. -->
- **Contract Refs**: <!-- openapi:POST /v1/example, jsonschema:ExampleRequest -->

<!-- OPTIONAL: Capture invariant-style rules for stateful or contract-sensitive behavior. -->
#### Rules / Invariants

<!-- Example:
- Requests without an active session MUST be rejected.
- Duplicate events MUST be ignored.
-->

<!-- OPTIONAL: Prefer a compact table when state changes matter. -->
#### State Transitions

<!-- Example:
| From | Event | To | Notes |
| --- | --- | --- | --- |
| pending | approve | active | Audit entry recorded |
| active | suspend | suspended | Background work stops |
-->

#### Scenario: <!-- scenario name -->

- **WHEN** <!-- condition -->
- **THEN** <!-- expected outcome -->

<!-- Traceability note:
  - Requirement ID format: `<capability>:<requirement-name>` (e.g. `auth:two-factor-auth`)
  - When any requirement in a change includes a Requirement ID, ALL requirements must include one.
  - Requirement IDs enable `ito trace <change-id>` to report task coverage.
  - Omit the Requirement ID line entirely if you do not need traceability for this change.
-->
<!-- ITO:END -->
</template>

<unlocks>
Completing this artifact enables: tasks
</unlocks>


</artifact>
````

```bash
target/debug/ito agent instruction tasks --change 025-11_repository-backed-artifact-mutations
```

````output
- Generating instructions...
<artifact id="tasks" change="025-11_repository-backed-artifact-mutations" schema="spec-driven">


<task>
Create the tasks artifact for change "025-11_repository-backed-artifact-mutations".
Implementation tasks derived from specs and design
</task>

<context>
Read these files for context before creating this artifact:

<dependency id="specs" status="done">
  <path>/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/.ito/changes/025-11_repository-backed-artifact-mutations/specs/**/*.md</path>
  <description>Detailed specifications for the change</description>
</dependency>
<dependency id="design" status="done">
  <path>/Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/.ito/changes/025-11_repository-backed-artifact-mutations/design.md</path>
  <description>Technical design document with implementation details</description>
</dependency>
</context>


<user_guidance>
## Scoped Guidance (tasks)

` marker.

<!-- ITO:END -->

## Your Tasks Guidance

### Task Quality

- Write small, actionable tasks with clear completion criteria.
- Prefer tasks that can be completed and verified independently.
- Include or reference a concrete verification command for each task or section.

### Execution and Status Tracking

- Keep exactly one task in progress at a time.
- Use `ito tasks start <change-id> <task-id>` and `ito tasks complete <change-id> <task-id>` for status updates.
- Avoid manual edits to task state unless unavoidable.

### Wave / Batch Convention

- If tasks include explicit waves, finish and verify one wave before moving to the next.
- If tasks are checkbox-only (no wave sections), treat each major section as a logical batch.

## Shared Guidance

` marker.

<!-- ITO:END -->

## Project Guidance

### Rust Code Quality

After modifying Rust code, dispatch these subagents **in parallel**:
- @code-simplifier - Refactors for clarity per `.ito/user-rust-style.md`
- @documentation-police - Ensures public APIs have useful docs
- @rust-code-reviewer - Checks for idiomatic usage, error handling, and best practices

Then run `make check` to verify.

### Running test and checks

Always use the test-with-subagent skill for running builds, tests and checks.

### Commits

Make small, focused commits with clear messages.
Regularly use the `ito-commit` skill for conventional commits aligned with the project's commit message guidelines.
IF you have to do more work to make changes that don't break the build whilst remaining small and focused, so be it.

### Subagent Collaboration

Subagents are first-class tools in this repo. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.

Diversity is good: for non-trivial changes, get at least two independent review passes (for example: `@rust-code-reviewer` + `@codex-review`).

Commonly useful subagents:

- `@explore` - fast codebase navigation/search
- `@test-runner` - runs `make test` / `make check` with curated output
- `@rust-quality-checker` - Rust style/idioms/conventions checks
- `@rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)
- `@rust-test-engineer` - test strategy and coverage design
- `@codex-review` - diff review for correctness and edge cases
- `@documentation-police` - docs coverage/quality
- `@code-simplifier` - refactor for clarity and maintainability
- `@code-quality-squad` - parallel Rust quality workflows
- `@multi-agent` - explore multiple approaches and synthesize

### Showboat Demo Documents

This repo uses [Showboat](https://github.com/simonw/showboat) to have agents produce
executable demo documents that prove their work. Showboat builds markdown files incrementally
via CLI commands (`init`, `note`, `exec`, `image`, `pop`) that capture real command output --
this prevents agents from fabricating results.

- Available via `uvx showboat` (no install required)
- Run `uvx showboat --help` for full CLI reference
- See `.ito/user-prompts/apply.md` for detailed apply-phase usage
- Demo docs go in `.ito/changes/<change-id>/demos/`
- **Never edit showboat markdown directly** -- always use the CLI commands
</user_guidance>


<guidance>
- Keep `Files`, `Action`, `Verify`, `Done When`, `Requirements`, `Status`, and `Updated At` concrete so task-quality validation can reason about the work.
- Prefer specific verification commands over vague placeholders such as "run tests" or "check it works".
</guidance>


<output>
Mutate this Ito active-work artifact through the repository-backed CLI, not direct file edits.
Recommended command: `ito write change 025-11_repository-backed-artifact-mutations tasks` for full replacement, or `ito patch change 025-11_repository-backed-artifact-mutations tasks` for a targeted diff. Use semantic `ito tasks ...` commands for task lifecycle/status changes.
Repository projection path: /Users/jack/Code/withakay/ito/ito-worktrees/025-11_repository-backed-artifact-mutations/.ito/changes/025-11_repository-backed-artifact-mutations/tasks.md
Use ordinary file-edit tools only for non-Ito project/code files.
</output>

<instruction>
Create the task plan that breaks down the implementation work.

Prefer the enhanced tasks.md format (waves + per-task metadata) so Ito can
track execution progress via the tasks CLI.

Use the tasks CLI while authoring and executing:
- Initialize/upgrade to enhanced tasks: `ito tasks init <change-id>`
- See progress: `ito tasks status <change-id>`
- Pick next ready task: `ito tasks next <change-id>`
- Mark status: `ito tasks start <change-id> <task-id>` / `ito tasks complete <change-id> <task-id>`
- Defer work: `ito tasks shelve|unshelve <change-id> <task-id>`
- Print tasks.md: `ito tasks show <change-id>`

Authoring guidelines:
- Organize work into `## Wave N` sections
- Declare wave-level dependencies with `- **Depends On**: ...`
- Task IDs SHOULD be `wave.task` (e.g., `1.1`, `2.3`)
- Task dependencies MUST be within the same wave
- Each task should include: Files, Dependencies, Action, Verify, Done When, Updated At, Status
- Keep tasks small and verifiable; order by dependency

Reference specs for what needs to be built and design for how to build it.
</instruction>


<template>
<!-- ITO:START -->
# Tasks for: <!-- CHANGE_ID -->

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status <!-- CHANGE_ID -->
ito tasks next <!-- CHANGE_ID -->
ito tasks start <!-- CHANGE_ID --> 1.1
ito tasks complete <!-- CHANGE_ID --> 1.1
```

______________________________________________________________________

## Wave 1

### Task 1.1: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: None
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Requirements**: <!-- capability:requirement-name, capability:other-requirement -->
- **Status**: [ ] pending

### Task 1.2: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: Task 1.1
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Requirements**: <!-- capability:requirement-name -->
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: <!-- Task Name -->

- **Files**: <!-- file paths -->
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: <!-- what to implement -->
- **Verify**: <!-- command to verify -->
- **Done When**: <!-- acceptance criteria -->
- **Requirements**: <!-- capability:requirement-name -->
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
</template>


</artifact>
````
