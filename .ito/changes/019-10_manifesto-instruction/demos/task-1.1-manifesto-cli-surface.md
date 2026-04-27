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

```bash
cargo run -p ito-cli -- agent instruction manifesto --json
```

```output
   Compiling cfg-if v1.0.4
   Compiling itoa v1.0.18
   Compiling memchr v2.8.0
   Compiling once_cell v1.21.4
   Compiling bitflags v2.11.1
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling bytes v1.11.1
   Compiling libc v0.2.186
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
   Compiling zerocopy v0.8.48
   Compiling futures-core v0.3.32
   Compiling generic-array v0.14.7
   Compiling strsim v0.11.1
   Compiling typenum v1.20.0
   Compiling futures-sink v0.3.32
   Compiling rustix v1.1.4
   Compiling getrandom v0.3.4
   Compiling httparse v1.10.1
   Compiling futures-channel v0.3.32
   Compiling core-foundation-sys v0.8.7
   Compiling subtle v2.6.1
   Compiling getrandom v0.4.2
   Compiling zmij v1.0.21
   Compiling tracing-core v0.1.36
   Compiling aho-corasick v1.1.4
   Compiling powerfmt v0.2.0
   Compiling http v1.4.0
   Compiling regex-syntax v0.8.10
   Compiling time-core v0.1.8
   Compiling num-conv v0.2.1
   Compiling object v0.37.3
   Compiling darling_core v0.20.11
   Compiling serde_json v1.0.149
   Compiling deranged v0.5.8
   Compiling futures-io v0.3.32
   Compiling adler2 v2.0.1
   Compiling slab v0.4.12
   Compiling unicode-width v0.2.2
   Compiling gimli v0.32.3
   Compiling errno v0.3.14
   Compiling signal-hook-registry v1.4.8
   Compiling socket2 v0.6.3
   Compiling mio v1.2.0
   Compiling regex-automata v0.4.14
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling cpufeatures v0.2.17
   Compiling digest v0.10.7
   Compiling smallvec v1.15.1
   Compiling tokio v1.52.1
   Compiling futures-task v0.3.32
   Compiling percent-encoding v2.3.2
   Compiling thiserror v2.0.18
   Compiling rand_core v0.9.5
   Compiling futures-util v0.3.32
   Compiling darling_macro v0.20.11
   Compiling miniz_oxide v0.8.9
   Compiling tracing v0.1.44
   Compiling http-body v1.0.1
   Compiling rustc-demangle v0.1.27
   Compiling darling v0.20.11
   Compiling owo-colors v4.3.0
   Compiling addr2line v0.25.1
   Compiling anyhow v1.0.102
   Compiling ahash v0.8.12
   Compiling is_ci v1.2.0
   Compiling tower-service v0.3.3
   Compiling unicode-linebreak v0.1.5
   Compiling supports-color v3.0.2
   Compiling terminal_size v0.4.4
   Compiling textwrap v0.16.2
   Compiling num-traits v0.2.19
   Compiling equivalent v1.0.2
   Compiling zeroize v1.8.2
   Compiling fastrand v2.4.1
   Compiling mime v0.3.17
   Compiling base64 v0.22.1
   Compiling httpdate v1.0.3
   Compiling ppv-lite86 v0.2.21
   Compiling unicode-width v0.1.14
   Compiling ryu v1.0.23
   Compiling supports-hyperlinks v3.2.0
   Compiling rand_chacha v0.9.0
   Compiling tower-layer v0.3.3
   Compiling hashbrown v0.17.0
   Compiling rand v0.9.4
   Compiling supports-unicode v3.0.0
   Compiling tempfile v3.27.0
   Compiling libsqlite3-sys v0.28.0
   Compiling http-body-util v0.1.3
   Compiling sha1 v0.10.6
   Compiling core-foundation v0.10.1
   Compiling indexmap v2.14.0
   Compiling security-framework-sys v2.17.0
   Compiling iana-time-zone v0.1.65
   Compiling base64ct v1.8.3
   Compiling unicase v2.9.0
   Compiling atomic-waker v1.1.2
   Compiling data-encoding v2.11.0
   Compiling sync_wrapper v1.0.2
   Compiling utf8parse v0.2.2
   Compiling derive_builder_core v0.20.2
   Compiling schemars v0.8.22
   Compiling pem-rfc7468 v1.0.0
   Compiling hyper v1.9.0
   Compiling anstyle-parse v1.0.0
   Compiling tungstenite v0.29.0
   Compiling mime_guess v2.0.5
   Compiling security-framework v3.7.0
   Compiling native-tls v0.2.18
   Compiling bstr v1.12.1
   Compiling uuid v1.23.1
   Compiling backtrace v0.3.76
   Compiling derive_builder_macro v0.20.2
   Compiling hashbrown v0.14.5
   Compiling form_urlencoded v1.2.2
   Compiling time-macros v0.2.27
   Compiling backtrace-ext v0.2.1
   Compiling miette v7.6.0
   Compiling grep-matcher v0.1.8
   Compiling encoding_rs v0.8.35
   Compiling derive_builder v0.20.2
   Compiling hashlink v0.9.1
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle-query v1.1.5
   Compiling fallible-streaming-iterator v0.1.9
   Compiling dyn-clone v1.0.20
   Compiling anstyle v1.0.14
   Compiling chrono v0.4.44
   Compiling fallible-iterator v0.3.0
   Compiling colorchoice v1.0.5
   Compiling anstream v1.0.0
   Compiling thiserror v1.0.69
   Compiling cookie v0.18.1
   Compiling ito-common v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-common)
   Compiling time v0.3.47
   Compiling encoding_rs_io v0.1.7
   Compiling vergen-lib v9.1.0
   Compiling include_dir v0.7.4
   Compiling serde_urlencoded v0.7.1
   Compiling tokio-tungstenite v0.29.0
   Compiling hyper-util v0.1.20
   Compiling der v0.8.0
   Compiling tower v0.5.3
   Compiling axum-core v0.5.6
   Compiling ureq-proto v0.6.0
   Compiling regex v1.12.3
   Compiling rustls-pki-types v1.14.1
   Compiling sha2 v0.10.9
   Compiling minijinja v1.0.22
   Compiling serde_path_to_error v0.1.20
   Compiling memmap2 v0.9.10
   Compiling clap_lex v1.1.0
   Compiling matchit v0.8.4
   Compiling unsafe-libyaml v0.2.11
   Compiling hex v0.4.3
   Compiling same-file v1.0.6
   Compiling utf8-zero v0.8.1
   Compiling walkdir v2.5.0
   Compiling clap_builder v4.6.0
   Compiling ureq v3.3.0
   Compiling rustix v0.38.44
   Compiling grep-searcher v0.1.16
   Compiling vergen v9.1.0
   Compiling ito-config v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-config)
   Compiling nix v0.28.0
   Compiling serde_yaml v0.9.34+deprecated
   Compiling axum v0.8.9
   Compiling rusqlite v0.31.0
   Compiling grep-regex v0.1.14
   Compiling tokio-util v0.7.18
   Compiling http-range-header v0.4.2
   Compiling glob v0.3.3
   Compiling shell-words v1.1.1
   Compiling vergen-gitcl v9.1.0
   Compiling ito-templates v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-templates)
   Compiling tower-http v0.6.8
   Compiling filedescriptor v0.8.3
   Compiling futures-executor v0.3.32
   Compiling ito-domain v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-domain)
   Compiling serde_spanned v0.6.9
   Compiling toml_datetime v0.6.11
   Compiling clap v4.6.1
   Compiling serial2 v0.2.36
   Compiling toml_write v0.1.2
   Compiling downcast-rs v1.2.1
   Compiling lazy_static v1.5.0
   Compiling winnow v0.7.15
   Compiling portable-pty v0.9.0
   Compiling sharded-slab v0.1.7
   Compiling futures v0.3.32
   Compiling ito-cli v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-cli)
   Compiling gethostname v0.5.0
   Compiling matchers v0.2.0
   Compiling hmac v0.12.1
   Compiling console v0.16.3
   Compiling tracing-log v0.2.0
   Compiling thread_local v1.1.9
   Compiling nu-ansi-term v0.50.3
   Compiling dialoguer v0.12.0
   Compiling clap_complete v4.6.2
   Compiling ito-logging v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-logging)
   Compiling serde_ignored v0.1.14
   Compiling tracing-subscriber v0.3.23
   Compiling toml_edit v0.22.27
   Compiling axum-extra v0.10.3
   Compiling ito-core v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-core)
   Compiling toml v0.8.23
   Compiling ito-backend v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-backend)
   Compiling ito-web v0.1.29 (/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/ito-rs/crates/ito-web)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 21.06s
     Running `target/debug/ito agent instruction manifesto --json`
{
  "artifact": "manifesto",
  "instruction": "# Ito Manifesto: Execution Contract\n\n## Contract\n\nYou are operating under the Ito protocol.\n\nIto is a change-driven, spec/context-driven workflow for AI-assisted software development. Your job is to advance explicit Ito artifacts through a controlled lifecycle, not merely to edit files.\n\n- Manifesto variant: `light`\n- Operating mode: `manifesto`\n- Capability profile: `full`\n- Requested operation: `none`\n- Ito project path: `.ito`\n- Generated at: `2026-04-27T10:20:29Z`\n\nWhen live Ito CLI access is available, prefer live Ito commands because they can resolve current state. When Ito CLI access is unavailable, follow this manifesto, disclose uncertainty, and do not invent project facts.\n\n## Hard Rules\n\n1. MUST prefer deterministic project facts over guesses.\n2. MUST use the exact supplied change ID when one is present.\n3. MUST treat config-derived worktree and coordination rules as hard constraints.\n4. MUST obey the active capability profile.\n5. MUST NOT write product code in `planning`, `proposal-only`, or `review-only` profiles.\n6. MUST NOT write from the main/control checkout when worktrees are enabled.\n7. MUST NOT reuse one worktree for two changes.\n8. MUST NOT claim validation, tests, archive, sync, memory capture, or review succeeded unless actually observed.\n9. MUST record material scope changes back into proposal/spec/design/tasks.\n10. MUST surface conflicts before proceeding.\n\n## Source of Truth\n\nUse this order when sources conflict:\n\n1. Latest explicit user instruction.\n2. Repository state and files visible to the agent.\n3. Manifesto state capsule.\n4. Manifesto config capsule.\n5. Change artifacts under `.ito/changes/<change-id>/`.\n6. Durable specs under `.ito/specs/`.\n7. Rendered Ito instruction text embedded below.\n8. User/project guidance embedded below.\n9. Prior model memory or assumptions.\n\n## Capability Profile\n\nActive profile: `full`.\n\nYou may perform the full lifecycle, but only through valid state transitions and policy checks.\n\n## State Capsule\n\n```json\n{\n  \"artifacts\": {\n    \"design\": \"missing\",\n    \"proposal\": \"missing\",\n    \"specs\": \"missing\",\n    \"tasks\": \"missing\"\n  },\n  \"capability_profile\": \"full\",\n  \"change_id\": null,\n  \"coordination_branch\": {\n    \"enabled\": false,\n    \"storage\": \"worktree\",\n    \"synced_at_generation\": null\n  },\n  \"mode\": \"manifesto\",\n  \"operation\": null,\n  \"project_path\": \".ito\",\n  \"review_status\": \"unknown\",\n  \"schema\": null,\n  \"validation\": {\n    \"last_known_status\": \"unknown\"\n  },\n  \"variant\": \"light\",\n  \"worktree\": {\n    \"current_checkout_role\": \"change-worktree\",\n    \"enabled\": true,\n    \"required_before_writes\": true\n  }\n}\n```\n\nNo change is selected. Remain in `no-change-selected` until a valid change ID is supplied or discovered.\n\n## State Machine\n\n| State | Allowed operations | Forbidden operations |\n| --- | --- | --- |\n| `no-change-selected` | inspect, select-change, propose-change | apply, archive, finish, product-code-edit |\n| `proposal-drafting` | proposal, specs, design, tasks, validate, review | apply unless escalated, archive |\n| `review-needed` | review, revise-artifacts | apply unless review waived, archive |\n| `apply-ready` | worktree-ensure, apply, validate | main-write, unrelated-edits, archive |\n| `applying` | implement, task-update, validate, revise-artifacts | scope-expansion-without-artifact-update, unsupported-complete |\n| `reviewing-implementation` | review, fix, validate | archive-with-findings |\n| `archive-ready` | archive, reconcile | implementation-expansion |\n| `finished` | finish, cleanup, memory-capture, report | further-edits-without-reopen |\n\n## Artifact Model\n\nCanonical Ito layout:\n\n```text\n.ito/\n  specs/\n  changes/<change-id>/\n    .ito.yaml\n    proposal.md\n    design.md\n    tasks.md\n    specs/\n  modules/\n  planning/\n```\n\n- `proposal.md` explains intent, scope, non-goals, risks, and acceptance criteria.\n- Spec deltas describe durable behavioral changes.\n- `design.md` explains architecture and trade-offs.\n- `tasks.md` contains ordered, checkable implementation work.\n- Archive merges accepted deltas into durable specs and preserves change history.\n\n## Worktree Policy\n\nWorktrees are enabled.\n\n- Treat the main/control checkout as read-only for proposal artifacts, code edits, documentation edits, generated asset updates, commits, and implementation work.\n- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change.\n- Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.\n- Use the full change ID as the branch and primary worktree directory name unless config explicitly says otherwise.\n- Do not reuse one worktree for two changes.\n\nConfig:\n\n- Strategy: `bare_control_siblings`\n- Default branch: `main`\n- Layout base dir: `default/unspecified`\n- Layout dir name: `ito-worktrees`\n- Apply setup enabled: `true`\n- Apply integration mode: `commit_pr`\n\nIf you cannot determine whether you are in the correct worktree, do not perform writes.\n\n## Coordination Branch Policy\n\nCoordination branch mode is disabled.\n\n## Config Capsule\n\n```json\n{\n  \"backend\": {\n    \"enabled\": false,\n    \"project\": {\n      \"org\": \"withakay\",\n      \"repo\": \"ito\"\n    },\n    \"url\": null\n  },\n  \"coordination_branch\": {\n    \"enabled\": false,\n    \"name\": \"ito/internal/changes\",\n    \"storage\": \"worktree\",\n    \"worktree_path\": null\n  },\n  \"defaults\": {\n    \"profile\": \"full\",\n    \"variant\": \"light\"\n  },\n  \"memory\": {\n    \"capture_configured\": false,\n    \"query_configured\": false,\n    \"search_configured\": false\n  },\n  \"project_root\": \".\",\n  \"worktrees\": {\n    \"apply_enabled\": true,\n    \"apply_integration_mode\": \"commit_pr\",\n    \"default_branch\": \"main\",\n    \"enabled\": true,\n    \"layout_base_dir\": null,\n    \"layout_dir_name\": \"ito-worktrees\",\n    \"strategy\": \"bare_control_siblings\"\n  }\n}\n```\n\n## Operation Playbooks\n\n### Proposal\n\nRead context, then create or update `proposal.md` with problem, goal, scope, non-goals, risks, and acceptance criteria. Do not implement while proposal scope is still being clarified unless explicitly escalated and permitted.\n\n### Specs\n\nWrite durable behavior and constraints as deltas. Keep specs independent from temporary implementation details.\n\n### Design\n\nExplain architecture, trade-offs, migration, compatibility, and operational impact when the change affects structure or cross-cutting behavior.\n\n### Tasks\n\nProduce ordered, checkable implementation tasks. Include validation and review tasks. Do not mark tasks complete without evidence.\n\n### Apply\n\nEnsure the correct worktree, re-read artifacts, implement scoped tasks, update task status honestly, run validation/tests where possible, and record deviations.\n\n### Review\n\nReview artifacts first, implementation second. Check code against proposal/specs/tasks and provide concrete findings.\n\n### Archive\n\nOnly archive accepted changes. Merge approved deltas into durable specs and preserve historical artifacts.\n\n### Finish\n\nClean up according to config, capture memory when configured, refresh archive/spec state, and report final status.\n\n## Memory\n\nNo memory provider is configured. Do not claim memory was searched, queried, or captured.\n\n## User Guidance\n\n<user_guidance>\n` marker.\n\n<!-- ITO:END -->\n\n## Project Guidance\n\n### Rust Code Quality\n\nAfter modifying Rust code, dispatch these subagents **in parallel**:\n- @code-simplifier - Refactors for clarity per `.ito/user-rust-style.md`\n- @documentation-police - Ensures public APIs have useful docs\n- @rust-code-reviewer - Checks for idiomatic usage, error handling, and best practices\n\nThen run `make check` to verify.\n\n### Running test and checks\n\nAlways use the test-with-subagent skill for running builds, tests and checks.\n\n### Commits\n\nMake small, focused commits with clear messages.\nRegularly use the `ito-commit` skill for conventional commits aligned with the project's commit message guidelines.\nIF you have to do more work to make changes that don't break the build whilst remaining small and focused, so be it.\n\n### Subagent Collaboration\n\nSubagents are first-class tools in this repo. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.\n\nDiversity is good: for non-trivial changes, get at least two independent review passes (for example: `@rust-code-reviewer` + `@codex-review`).\n\nCommonly useful subagents:\n\n- `@explore` - fast codebase navigation/search\n- `@test-runner` - runs `make test` / `make check` with curated output\n- `@rust-quality-checker` - Rust style/idioms/conventions checks\n- `@rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)\n- `@rust-test-engineer` - test strategy and coverage design\n- `@codex-review` - diff review for correctness and edge cases\n- `@documentation-police` - docs coverage/quality\n- `@code-simplifier` - refactor for clarity and maintainability\n- `@code-quality-squad` - parallel Rust quality workflows\n- `@multi-agent` - explore multiple approaches and synthesize\n\n### Showboat Demo Documents\n\nThis repo uses [Showboat](https://github.com/simonw/showboat) to have agents produce\nexecutable demo documents that prove their work. Showboat builds markdown files incrementally\nvia CLI commands (`init`, `note`, `exec`, `image`, `pop`) that capture real command output --\nthis prevents agents from fabricating results.\n\n- Available via `uvx showboat` (no install required)\n- Run `uvx showboat --help` for full CLI reference\n- See `.ito/user-prompts/apply.md` for detailed apply-phase usage\n- Demo docs go in `.ito/changes/<change-id>/demos/`\n- **Never edit showboat markdown directly** -- always use the CLI commands\n</user_guidance>\n\n## Rendered Ito Instructions\n\nThis is the light manifesto variant. Prefer live `ito agent instruction <artifact>` for exact operation prompts when Ito is available.\n\n## Fallback Behavior\n\nWhen Ito CLI access is unavailable:\n\n- Do not invent missing state.\n- If a change ID is unknown, remain in `no-change-selected`.\n- If worktrees are enabled and the current checkout role is unknown, avoid writes.\n- If validation cannot be run, report it as not run.\n- If a CLI-dependent step cannot execute, follow the surrounding rule text and mark the outcome as unverified.\n- If the capability profile forbids mutation, provide proposed content, diffs, or instructions rather than editing files.\n\n## Final Rule\n\nProceed only through valid state transitions. Preserve Ito artifacts as the durable record of intent, behavior, tasks, validation, review, and archive. When uncertain, state what is known, what is unknown, and which writes or claims are unsafe.\n",
  "profile": "full",
  "state": "no-change-selected",
  "variant": "light"
}
```
