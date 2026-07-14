# Task 1.2: Manifesto Render Context

*2026-04-27T20:53:50Z by Showboat 0.6.1*
<!-- showboat-id: 24d1949e-5ffd-46e4-a59f-9c91880847c1 -->

Added the first structured manifesto render path with defaults, change-scoped state resolution, config/worktree/coordination capsules, and direct template coverage.

```bash
cargo test -p ito-templates instructions_tests && cargo test -p ito-cli --test instructions_more
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-43511d335e81e446)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 85 filtered out; finished in 0.00s

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

    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/instructions_more.rs (target/debug/deps/instructions_more-9654ca94826e629f)

running 19 tests
test agent_instruction_manifesto_rejects_operation_for_light_variant ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_manifesto_uses_default_variant_and_profile ... ok
test agent_instruction_manifesto_json_includes_resolved_defaults ... ok
test agent_instruction_manifesto_change_scope_includes_change_state ... ok
test agent_instruction_manifesto_change_scope_json_reports_state ... ok
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok
test agent_instruction_review_renders_review_template ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s

```

```bash
target/debug/ito agent instruction manifesto --change 019-10_manifesto-instruction --json
```

```output
{
  "artifact": "manifesto",
  "instruction": "# Ito Manifesto: Execution Contract\n\n## Contract\n\nYou are operating under the Ito protocol.\n\nIto is a change-driven, spec/context-driven workflow for AI-assisted software development. Your job is to advance explicit Ito artifacts through a controlled lifecycle, not merely to edit files.\n\n- Manifesto variant: `light`\n- Operating mode: `manifesto`\n- Capability profile: `full`\n- Requested operation: `none`\n- Ito project path: `.ito`\n- Generated at: `2026-04-27T20:53:55Z`\n\nWhen live Ito CLI access is available, prefer live Ito commands because they can resolve current state. When Ito CLI access is unavailable, follow this manifesto, disclose uncertainty, and do not invent project facts.\n\n## Hard Rules\n\n1. MUST prefer deterministic project facts over guesses.\n2. MUST use the exact supplied change ID when one is present.\n3. MUST treat config-derived worktree and coordination rules as hard constraints.\n4. MUST obey the active capability profile.\n5. MUST NOT write product code in `planning`, `proposal-only`, or `review-only` profiles.\n6. MUST NOT write from the main/control checkout when worktrees are enabled.\n7. MUST NOT reuse one worktree for two changes.\n8. MUST NOT claim validation, tests, archive, sync, memory capture, or review succeeded unless actually observed.\n9. MUST record material scope changes back into proposal/spec/design/tasks.\n10. MUST surface conflicts before proceeding.\n\n## Source of Truth\n\nUse this order when sources conflict:\n\n1. Latest explicit user instruction.\n2. Repository state and files visible to the agent.\n3. Manifesto state capsule.\n4. Manifesto config capsule.\n5. Change artifacts under `.ito/changes/<change-id>/`.\n6. Durable specs under `.ito/specs/`.\n7. Rendered Ito instruction text embedded below.\n8. User/project guidance embedded below.\n9. Prior model memory or assumptions.\n\n## Capability Profile\n\nActive profile: `full`.\n\nYou may perform the full lifecycle, but only through valid state transitions and policy checks.\n\n## State Capsule\n\n```json\n{\n  \"artifacts\": {\n    \"design\": \"done\",\n    \"proposal\": \"done\",\n    \"specs\": \"done\",\n    \"tasks\": \"done\"\n  },\n  \"capability_profile\": \"full\",\n  \"change_id\": \"019-10_manifesto-instruction\",\n  \"coordination_branch\": {\n    \"enabled\": true,\n    \"storage\": \"worktree\",\n    \"synced_at_generation\": \"2026-04-27T20:53:55Z\"\n  },\n  \"mode\": \"manifesto\",\n  \"operation\": null,\n  \"project_path\": \".ito\",\n  \"review_status\": \"unknown\",\n  \"schema\": \"spec-driven\",\n  \"validation\": {\n    \"last_known_status\": \"passed\"\n  },\n  \"variant\": \"light\",\n  \"worktree\": {\n    \"current_checkout_role\": \"change-worktree\",\n    \"enabled\": true,\n    \"required_before_writes\": true\n  }\n}\n```\n\nCurrent change:\n\n- Change ID: `019-10_manifesto-instruction`\n- Change directory: `.ito/changes/019-10_manifesto-instruction`\n- Schema: `spec-driven`\n- Module: `019 — templates`\n- Available artifacts: `proposal` `design` `specs` `tasks`- Missing artifacts: none known\n## State Machine\n\n| State | Allowed operations | Forbidden operations |\n| --- | --- | --- |\n| `no-change-selected` | inspect, select-change, propose-change | apply, archive, finish, product-code-edit |\n| `proposal-drafting` | proposal, specs, design, tasks, validate, review | apply unless escalated, archive |\n| `review-needed` | review, revise-artifacts | apply unless review waived, archive |\n| `apply-ready` | worktree-ensure, apply, validate | main-write, unrelated-edits, archive |\n| `applying` | implement, task-update, validate, revise-artifacts | scope-expansion-without-artifact-update, unsupported-complete |\n| `reviewing-implementation` | review, fix, validate | archive-with-findings |\n| `archive-ready` | archive, reconcile | implementation-expansion |\n| `finished` | finish, cleanup, memory-capture, report | further-edits-without-reopen |\n\n## Artifact Model\n\nCanonical Ito layout:\n\n```text\n.ito/\n  specs/\n  changes/<change-id>/\n    .ito.yaml\n    proposal.md\n    design.md\n    tasks.md\n    specs/\n  modules/\n  planning/\n```\n\n- `proposal.md` explains intent, scope, non-goals, risks, and acceptance criteria.\n- Spec deltas describe durable behavioral changes.\n- `design.md` explains architecture and trade-offs.\n- `tasks.md` contains ordered, checkable implementation work.\n- Archive merges accepted deltas into durable specs and preserves change history.\n\n## Worktree Policy\n\nWorktrees are enabled.\n\n- Treat the main/control checkout as read-only for proposal artifacts, code edits, documentation edits, generated asset updates, commits, and implementation work.\n- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change.\n- Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.\n- Use the full change ID as the branch and primary worktree directory name unless config explicitly says otherwise.\n- Do not reuse one worktree for two changes.\n\nConfig:\n\n- Strategy: `bare_control_siblings`\n- Default branch: `main`\n- Layout base dir: `default/unspecified`\n- Layout dir name: `ito-worktrees`\n- Apply setup enabled: `true`\n- Apply integration mode: `commit_pr`\n\nIf you cannot determine whether you are in the correct worktree, do not perform writes.\n\n## Coordination Branch Policy\n\nCoordination branch mode is enabled.\n\n- Ito artifacts are coordinated through the configured coordination branch/storage.\n- Multiple change worktrees may operate concurrently, but artifacts must coordinate through the shared substrate.\n- Sync before reading or acting on change state whenever Ito is available.\n- Do not confuse the coordination worktree with the implementation worktree.\n\nConfig:\n\n- Branch: `ito/internal/changes`\n- Storage: `worktree`\n- Worktree path: `<redacted-path>`\n\n## Config Capsule\n\n```json\n{\n  \"backend\": {\n    \"enabled\": false,\n    \"project\": {\n      \"org\": \"withakay\",\n      \"repo\": \"ito\"\n    },\n    \"url\": null\n  },\n  \"coordination_branch\": {\n    \"enabled\": true,\n    \"name\": \"ito/internal/changes\",\n    \"storage\": \"worktree\",\n    \"worktree_path\": \"<redacted-path>\"\n  },\n  \"defaults\": {\n    \"profile\": \"full\",\n    \"variant\": \"light\"\n  },\n  \"memory\": {\n    \"capture_configured\": false,\n    \"query_configured\": false,\n    \"search_configured\": false\n  },\n  \"project_root\": \".\",\n  \"worktrees\": {\n    \"apply_enabled\": true,\n    \"apply_integration_mode\": \"commit_pr\",\n    \"default_branch\": \"main\",\n    \"enabled\": true,\n    \"layout_base_dir\": null,\n    \"layout_dir_name\": \"ito-worktrees\",\n    \"strategy\": \"bare_control_siblings\"\n  }\n}\n```\n\n## Operation Playbooks\n\n### Proposal\n\nRead context, then create or update `proposal.md` with problem, goal, scope, non-goals, risks, and acceptance criteria. Do not implement while proposal scope is still being clarified unless explicitly escalated and permitted.\n\n### Specs\n\nWrite durable behavior and constraints as deltas. Keep specs independent from temporary implementation details.\n\n### Design\n\nExplain architecture, trade-offs, migration, compatibility, and operational impact when the change affects structure or cross-cutting behavior.\n\n### Tasks\n\nProduce ordered, checkable implementation tasks. Include validation and review tasks. Do not mark tasks complete without evidence.\n\n### Apply\n\nEnsure the correct worktree, re-read artifacts, implement scoped tasks, update task status honestly, run validation/tests where possible, and record deviations.\n\n### Review\n\nReview artifacts first, implementation second. Check code against proposal/specs/tasks and provide concrete findings.\n\n### Archive\n\nOnly archive accepted changes. Merge approved deltas into durable specs and preserve historical artifacts.\n\n### Finish\n\nClean up according to config, capture memory when configured, refresh archive/spec state, and report final status.\n\n## Memory\n\nNo memory provider is configured. Do not claim memory was searched, queried, or captured.\n\n## User Guidance\n\n<user_guidance>\n` marker.\n\n<!-- ITO:END -->\n\n## Project Guidance\n\n### Rust Code Quality\n\nAfter modifying Rust code, dispatch these subagents **in parallel**:\n- @code-simplifier - Refactors for clarity per `.ito/user-rust-style.md`\n- @documentation-police - Ensures public APIs have useful docs\n- @rust-code-reviewer - Checks for idiomatic usage, error handling, and best practices\n\nThen run `make check` to verify.\n\n### Running test and checks\n\nAlways use the test-with-subagent skill for running builds, tests and checks.\n\n### Commits\n\nMake small, focused commits with clear messages.\nRegularly use the `ito-commit` skill for conventional commits aligned with the project's commit message guidelines.\nIF you have to do more work to make changes that don't break the build whilst remaining small and focused, so be it.\n\n### Subagent Collaboration\n\nSubagents are first-class tools in this repo. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.\n\nDiversity is good: for non-trivial changes, get at least two independent review passes (for example: `@rust-code-reviewer` + `@codex-review`).\n\nCommonly useful subagents:\n\n- `@explore` - fast codebase navigation/search\n- `@test-runner` - runs `make test` / `make check` with curated output\n- `@rust-quality-checker` - Rust style/idioms/conventions checks\n- `@rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)\n- `@rust-test-engineer` - test strategy and coverage design\n- `@codex-review` - diff review for correctness and edge cases\n- `@documentation-police` - docs coverage/quality\n- `@code-simplifier` - refactor for clarity and maintainability\n- `@code-quality-squad` - parallel Rust quality workflows\n- `@multi-agent` - explore multiple approaches and synthesize\n\n### Showboat Demo Documents\n\nThis repo uses [Showboat](https://github.com/simonw/showboat) to have agents produce\nexecutable demo documents that prove their work. Showboat builds markdown files incrementally\nvia CLI commands (`init`, `note`, `exec`, `image`, `pop`) that capture real command output --\nthis prevents agents from fabricating results.\n\n- Available via `uvx showboat` (no install required)\n- Run `uvx showboat --help` for full CLI reference\n- See `.ito/user-prompts/apply.md` for detailed apply-phase usage\n- Demo docs go in `.ito/changes/<change-id>/demos/`\n- **Never edit showboat markdown directly** -- always use the CLI commands\n</user_guidance>\n\n## Rendered Ito Instructions\n\nThis is the light manifesto variant. Prefer live `ito agent instruction <artifact>` for exact operation prompts when Ito is available.\n\n## Fallback Behavior\n\nWhen Ito CLI access is unavailable:\n\n- Do not invent missing state.\n- If a change ID is unknown, remain in `no-change-selected`.\n- If worktrees are enabled and the current checkout role is unknown, avoid writes.\n- If validation cannot be run, report it as not run.\n- If a CLI-dependent step cannot execute, follow the surrounding rule text and mark the outcome as unverified.\n- If the capability profile forbids mutation, provide proposed content, diffs, or instructions rather than editing files.\n\n## Final Rule\n\nProceed only through valid state transitions. Preserve Ito artifacts as the durable record of intent, behavior, tasks, validation, review, and archive. When uncertain, state what is known, what is unknown, and which writes or claims are unsafe.\n",
  "profile": "full",
  "state": "applying",
  "variant": "light"
}
```
