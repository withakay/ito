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
target/debug/ito agent instruction manifesto --change 019-10_manifesto-instruction --variant full
```

````output
# Ito Manifesto: Execution Contract

## Contract

You are operating under the Ito protocol.

Ito is a change-driven, spec/context-driven workflow for AI-assisted software development. Your job is to advance explicit Ito artifacts through a controlled lifecycle, not merely to edit files.

- Manifesto variant: `full`
- Operating mode: `manifesto`
- Capability profile: `full`
- Requested operation: `none`
- Ito project path: `.ito`
- Generated at: `2026-04-27T21:10:15Z`

When live Ito CLI access is available, prefer live Ito commands because they can resolve current state. When Ito CLI access is unavailable, follow this manifesto, disclose uncertainty, and do not invent project facts.

## Hard Rules

1. MUST prefer deterministic project facts over guesses.
2. MUST use the exact supplied change ID when one is present.
3. MUST treat config-derived worktree and coordination rules as hard constraints.
4. MUST obey the active capability profile.
5. MUST NOT write product code in `planning`, `proposal-only`, or `review-only` profiles.
6. MUST NOT write from the main/control checkout when worktrees are enabled.
7. MUST NOT reuse one worktree for two changes.
8. MUST NOT claim validation, tests, archive, sync, memory capture, or review succeeded unless actually observed.
9. MUST record material scope changes back into proposal/spec/design/tasks.
10. MUST surface conflicts before proceeding.

## Source of Truth

Use this order when sources conflict:

1. Latest explicit user instruction.
2. Repository state and files visible to the agent.
3. Manifesto state capsule.
4. Manifesto config capsule.
5. Change artifacts under `.ito/changes/<change-id>/`.
6. Durable specs under `.ito/specs/`.
7. Rendered Ito instruction text embedded below.
8. User/project guidance embedded below.
9. Prior model memory or assumptions.

## Capability Profile

Active profile: `full`.

You may perform the full lifecycle, but only through valid state transitions and policy checks.

## State Capsule

```json
{
  "artifacts": {
    "design": "done",
    "proposal": "done",
    "specs": "done",
    "tasks": "done"
  },
  "capability_profile": "full",
  "change_id": "019-10_manifesto-instruction",
  "coordination_branch": {
    "enabled": true,
    "storage": "worktree",
    "synced_at_generation": "2026-04-27T21:10:15Z"
  },
  "mode": "manifesto",
  "operation": null,
  "project_path": ".ito",
  "review_status": "unknown",
  "schema": "spec-driven",
  "validation": {
    "last_known_status": "passed"
  },
  "variant": "full",
  "worktree": {
    "current_checkout_role": "change-worktree",
    "enabled": true,
    "required_before_writes": true
  }
}
```

Current change:

- Change ID: `019-10_manifesto-instruction`
- Change directory: `.ito/changes/019-10_manifesto-instruction`
- Schema: `spec-driven`
- Module: `019 — templates`
- Available artifacts: `proposal` `design` `specs` `tasks`- Missing artifacts: none known
## State Machine

| State | Allowed operations | Forbidden operations |
| --- | --- | --- |
| `no-change-selected` | inspect, select-change, propose-change | apply, archive, finish, product-code-edit |
| `proposal-drafting` | proposal, specs, design, tasks, validate, review | apply unless escalated, archive |
| `review-needed` | review, revise-artifacts | apply unless review waived, archive |
| `apply-ready` | worktree-ensure, apply, validate | main-write, unrelated-edits, archive |
| `applying` | implement, task-update, validate, revise-artifacts | scope-expansion-without-artifact-update, unsupported-complete |
| `reviewing-implementation` | review, fix, validate | archive-with-findings |
| `archive-ready` | archive, reconcile | implementation-expansion |
| `finished` | finish, cleanup, memory-capture, report | further-edits-without-reopen |

## Artifact Model

Canonical Ito layout:

```text
.ito/
  specs/
  changes/<change-id>/
    .ito.yaml
    proposal.md
    design.md
    tasks.md
    specs/
  modules/
  planning/
```

- `proposal.md` explains intent, scope, non-goals, risks, and acceptance criteria.
- Spec deltas describe durable behavioral changes.
- `design.md` explains architecture and trade-offs.
- `tasks.md` contains ordered, checkable implementation work.
- Archive merges accepted deltas into durable specs and preserves change history.

## Worktree Policy

Worktrees are enabled.

- Treat the main/control checkout as read-only for proposal artifacts, code edits, documentation edits, generated asset updates, commits, and implementation work.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change.
- Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- Use the full change ID as the branch and primary worktree directory name unless config explicitly says otherwise.
- Do not reuse one worktree for two changes.

Config:

- Strategy: `bare_control_siblings`
- Default branch: `main`
- Layout base dir: `default/unspecified`
- Layout dir name: `ito-worktrees`
- Apply setup enabled: `true`
- Apply integration mode: `commit_pr`

If you cannot determine whether you are in the correct worktree, do not perform writes.

## Coordination Branch Policy

Coordination branch mode is enabled.

- Ito artifacts are coordinated through the configured coordination branch/storage.
- Multiple change worktrees may operate concurrently, but artifacts must coordinate through the shared substrate.
- Sync before reading or acting on change state whenever Ito is available.
- Do not confuse the coordination worktree with the implementation worktree.

Config:

- Branch: `ito/internal/changes`
- Storage: `worktree`
- Worktree path: `<redacted-path>`

## Config Capsule

```json
{
  "backend": {
    "enabled": false,
    "project": {
      "org": "withakay",
      "repo": "ito"
    },
    "url": null
  },
  "coordination_branch": {
    "enabled": true,
    "name": "ito/internal/changes",
    "storage": "worktree",
    "worktree_path": "<redacted-path>"
  },
  "defaults": {
    "profile": "full",
    "variant": "light"
  },
  "memory": {
    "capture_configured": false,
    "query_configured": false,
    "search_configured": false
  },
  "project_root": ".",
  "worktrees": {
    "apply_enabled": true,
    "apply_integration_mode": "commit_pr",
    "default_branch": "main",
    "enabled": true,
    "layout_base_dir": null,
    "layout_dir_name": "ito-worktrees",
    "strategy": "bare_control_siblings"
  }
}
```

## Operation Playbooks

### Proposal

Read context, then create or update `proposal.md` with problem, goal, scope, non-goals, risks, and acceptance criteria. Do not implement while proposal scope is still being clarified unless explicitly escalated and permitted.

### Specs

Write durable behavior and constraints as deltas. Keep specs independent from temporary implementation details.

### Design

Explain architecture, trade-offs, migration, compatibility, and operational impact when the change affects structure or cross-cutting behavior.

### Tasks

Produce ordered, checkable implementation tasks. Include validation and review tasks. Do not mark tasks complete without evidence.

### Apply

Ensure the correct worktree, re-read artifacts, implement scoped tasks, update task status honestly, run validation/tests where possible, and record deviations.

### Review

Review artifacts first, implementation second. Check code against proposal/specs/tasks and provide concrete findings.

### Archive

Only archive accepted changes. Merge approved deltas into durable specs and preserve historical artifacts.

### Finish

Clean up according to config, capture memory when configured, refresh archive/spec state, and report final status.

## Memory

No memory provider is configured. Do not claim memory was searched, queried, or captured.

## User Guidance

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

## Rendered Ito Instructions

### `apply`

## Apply: 019-10_manifesto-instruction
Schema: spec-driven

### Context Files
- design: /Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/design.md
- proposal: /Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/proposal.md
- specs: /Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/specs/**/*.md
- tasks: /Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/tasks.md

### Coordination Sync

`ito agent instruction apply` refreshes coordination state before rendering. If you are using these instructions later, or after switching worktrees, refresh again before editing. `ito sync` is a no-op unless coordination-worktree storage is active, and it rate-limits pushes to avoid excessive remote updates.

```bash
ito sync
```

### Worktree Setup

Strategy: `bare_control_siblings` | Default branch: `main` | Dir: `ito-worktrees`

Worktree rules for this change:

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- Before any write operation, create the dedicated worktree for this change or move into it.
- Use the full change ID as the branch and primary worktree directory name: `019-10_manifesto-instruction`.
- Do not reuse one worktree for two changes.
- Additional worktrees for this same change must start with `019-10_manifesto-instruction` and add a suffix, for example `019-10_manifesto-instruction-review`.

**Recommended**: Use `ito worktree ensure` to create and initialize the worktree in one step:

```bash
CHANGE_DIR=$(ito worktree ensure --change "019-10_manifesto-instruction") || {
  echo "Error: failed to ensure worktree for 019-10_manifesto-instruction" >&2
  exit 1
}
```

This creates the dedicated change worktree (if absent), copies include files, and runs setup commands.
All subsequent file operations should use `$CHANGE_DIR` as the working directory.

After `ito worktree ensure`, run the sync from inside the change worktree if you did not just run it there:

```bash
cd "$CHANGE_DIR"
ito sync
```

<details>
<summary>Manual setup (alternative)</summary>

For full layout diagrams and setup commands, run:

```bash
ito agent instruction worktrees
```

**Quick start for this change:**

```bash
CHANGE_NAME='019-10_manifesto-instruction'
CHANGE_DIR="$(ito path worktree --change "$CHANGE_NAME")"

if [ ! -d "$CHANGE_DIR" ]; then
  PROJECT_ROOT="$(ito path project-root)"
  mkdir -p "$(ito path worktrees-root)"
  git -C "$PROJECT_ROOT" worktree add "$CHANGE_DIR" -b "$CHANGE_NAME" "main"
fi

echo "Working directory: $CHANGE_DIR"
```
Synchronize coordination state from inside the change worktree before editing:

```bash
cd "$CHANGE_DIR"
ito sync
```

Copy local setup files into the change worktree (missing files are skipped):

```bash
SOURCE_ROOT="$(ito path worktree --main)"
for match in "$SOURCE_ROOT"/.env; do
  [ -e "$match" ] && cp "$match" "$CHANGE_DIR/" 2>/dev/null || true
done
for match in "$SOURCE_ROOT"/.envrc; do
  [ -e "$match" ] && cp "$match" "$CHANGE_DIR/" 2>/dev/null || true
done
for match in "$SOURCE_ROOT"/.mise.local.toml; do
  [ -e "$match" ] && cp "$match" "$CHANGE_DIR/" 2>/dev/null || true
done
```

</details>

### Task Tracking
- file: tasks.md
- format: enhanced
- path: /Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/tasks.md

### Testing Policy
- TDD workflow: red-green-refactor (RED -> GREEN -> REFACTOR)
- TDD loop: write a failing test (RED), implement the minimum to pass (GREEN), then refactor (REFACTOR)
- Coverage target: 100% (guidance; override per project)
- Override keys: defaults.testing.tdd.workflow, defaults.testing.coverage.target_percent
- Override files (low -> high): ito.json, .ito.json, .ito/config.json, $PROJECT_DIR/config.json

### Progress
3/6 complete

### Tasks
- [x] Add manifesto CLI surface
- [x] Define manifesto rendering context
- [x] Wire light and full manifesto rendering
- [] Compose embedded instructions for full mode
- [] Add profile and change-state regression coverage
- [] Add redaction and discoverability coverage

### Audit

Use `ito tasks` CLI commands for all task state changes (emits audit events automatically).
Avoid editing `tasks.md` directly, but if you do, run `ito audit reconcile --change 019-10_manifesto-instruction --fix` immediately after.
Run `ito audit reconcile --change 019-10_manifesto-instruction` before archiving.

### Instruction
Read context files and work through pending tasks.

Testing Policy:
- Follow a disciplined RED/GREEN/REFACTOR loop while implementing each task.
- Aim to meet or exceed the configured coverage target.
- See the Testing Policy section printed by `ito agent instruction apply`.

Prefer driving execution via the tasks CLI:
- `ito tasks next <change-id>`
- `ito tasks start <change-id> <task-id>`
- `ito tasks complete <change-id> <task-id>`
- `ito tasks status <change-id>`

Pause if you hit blockers or need clarification.

### Integration

Mode: **Commit & PR** — commit on the change branch, push, open a PR into `main`.

```bash
CHANGE_NAME='019-10_manifesto-instruction'
CHANGE_DIR="$(ito path worktree --change "$CHANGE_NAME")"
CHANGE_ID="${CHANGE_NAME%%_*}"
cd "$CHANGE_DIR"
git add -A && git commit -m "feat: $CHANGE_NAME"
git push -u origin "$CHANGE_NAME"
gh pr create --title "feat($CHANGE_ID): <short summary>" --body "Implements change $CHANGE_NAME"
```

### Cleanup

After the change branch is merged:

```bash
CHANGE_NAME='019-10_manifesto-instruction'
ito agent instruction finish --change "$CHANGE_NAME"
```

### User Guidance

## Scoped Guidance (apply)

` marker.

<!-- ITO:END -->

## Your Apply Guidance

After completing any logical batch of tasks (including checkbox-only task lists),
you MUST run @code-quality-squad review before reporting completion.
Treat the entire change as one wave if no wave sections exist.

When writing Rust tests that assert on structured data (structs/enums, nested graphs, vec/map payloads),
prefer `assert-struct` (https://crates.io/crates/assert-struct) over verbose field-by-field asserts.

Quick start:

```toml
[dev-dependencies]
assert-struct = "0.2"
```

```rust
use assert_struct::assert_struct;

assert_struct!(value, Type {
    important: "expected",
    nested.field: > 0,
    ..
});
```

## Showboat: Demo Documents for Completed Tasks

After completing each task (or logical batch of tasks), you MUST create a Showboat demo document
that demonstrates the work you just did. This is a key part of proving code actually works beyond
just automated tests.

### Setup

Showboat is available via `uvx showboat` (no install needed). Run `uvx showboat --help` to see
full CLI usage if needed.

### When to Create a Showboat Document

- After completing each task in `tasks.md`
- After completing a logical batch of related tasks
- Before reporting task completion

### Where to Put Demos

Place demo documents in the change directory:

```
.ito/changes/<change-id>/demos/
```

Name files descriptively: `task-1.1-schema-migration.md`, `api-endpoints.md`, etc.

### How to Build a Demo

Use the Showboat CLI commands to build the document incrementally. **Do NOT edit the markdown
file directly** -- always use the CLI so that command outputs are captured authentically.

```bash
# 1. Initialize the demo document
uvx showboat init demos/task-1.1-schema.md "Task 1.1: Database Schema Migration"

# 2. Add context notes explaining what was built
uvx showboat note demos/task-1.1-schema.md "Created the new user_sessions table with TTL support."

# 3. Run commands that prove the code works and capture their output
uvx showboat exec demos/task-1.1-schema.md bash "cargo test -p ito-core --lib schema -- --nocapture 2>&1 | tail -20"

# 4. Show the actual artifacts/output
uvx showboat exec demos/task-1.1-schema.md bash "cat ito-rs/crates/ito-core/src/schema.rs | head -30"

# 5. If something fails, remove the last entry and retry
uvx showboat pop demos/task-1.1-schema.md
```

### What to Demonstrate

- **Tests passing**: Run the relevant tests and capture output
- **CLI behavior**: Run the CLI commands that exercise the new feature
- **Code snippets**: Show key implementation details with `cat` / `head`
- **Before/after**: Show the state change your implementation caused
- **Error handling**: Demonstrate that error cases are handled correctly

### Important Rules

1. **Always use `uvx showboat exec`** to capture command output -- never fake output by editing the markdown
2. Use `uvx showboat pop` to remove failed entries rather than editing
3. Use `--workdir` if commands need to run from a specific directory
4. Keep demos focused -- one per task or logical batch, not one giant document
5. The demo is for your supervisor (the human) to quickly see what you built and verify it works

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


### `review`

# Peer Review Instructions

<review change="019-10_manifesto-instruction" generated-at="2026-04-27T21:10:15Z" />

You are reviewing an Ito change proposal package before implementation. Focus on proposal/spec/design/tasks quality, internal consistency, and risk.

## Change Context

- Change: `019-10_manifesto-instruction`
- Change directory: `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction`
- Schema: `spec-driven`
- Module: `019` (templates)
## Artifact Inventory

| Artifact | Present | Path |
| --- | --- | --- |
| `proposal` | yes | `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/proposal.md` |
| `specs` | yes | `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/specs/**/*.md` |
| `design` | yes | `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/design.md` |
| `tasks` | yes | `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/tasks.md` |

## Structural Validation

- Validation passed: yes
- Issues:
  - `[INFO]` `schema` - Resolved schema 'spec-driven' from embedded
  - `[INFO]` `schema.validation` - Using schema validation.yaml

## Context Files

Use these files as review inputs:
- `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/proposal.md`
- `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/specs/**/*.md`
- `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/design.md`
- `/Users/jack/Code/withakay/ito/ito-worktrees/019-10_manifesto-instruction/.ito/changes/019-10_manifesto-instruction/tasks.md`

## Proposal Review

Check and report:
- Problem framing is specific and evidence-based.
- Scope is bounded and exclusions are explicit.
- Claimed impact aligns with listed changes.
- Risks and trade-offs are concrete and testable.
- Success criteria are measurable and falsifiable.

## Spec Review

Check and report:
- Requirement statements use normative language (`SHALL`, `MUST`) consistently.
- Every requirement has at least one scenario with clear WHEN/THEN behavior.
- Edge cases and failure behavior are covered, not implied.
- Added/modified/removed deltas are coherent with existing specs.
- Requirement wording is implementation-agnostic where appropriate.
- No contradictory behavior appears across spec files.

## Design Review

Check and report:
- Architecture choices directly satisfy spec requirements.
- Alternatives and trade-offs are documented.
- Operational concerns (observability, rollback, migration) are addressed.
- Security/performance implications are explicit and actionable.

## Task Review

- Task summary: total=6, complete=3, in-progress=1, pending=2, shelved=0, waves=3

Check and report:
- Tasks map directly to proposal/spec/design deliverables.
- Dependencies and wave sequencing are realistic.
- Verification commands validate intended behavior.
- Task granularity is actionable and reviewable.

## Requirement Traceability

Status: **Traced** (5 requirements declared)



All declared requirements are covered by active tasks.


Check and report:
- Are all spec requirements linked to implementing tasks?
- Are there tasks that reference requirements that don't exist?
- Should the task plan be revised to cover uncovered requirements?

## Cross-Cutting Concerns

- Identify likely conflicts with other active work in this module (`019`).
- Flag any system-wide impacts that are not captured in requirements.
- Evaluate whether testing strategy is sufficient for risk level.
- Confirm decomposition supports incremental, reversible delivery.
- Affected main specs (`MODIFIED` deltas): none.

## Testing Policy

- TDD workflow: `red-green-refactor`
- Coverage target: `100%`

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

## Output Format

Return findings using this exact tag format at the start of each item:

- `[blocking]` for issues that must be fixed before implementation proceeds.
- `[suggestion]` for improvements that are non-blocking but valuable.
- `[note]` for informational context.

End with a single verdict line:

- `Verdict: approve`
- `Verdict: request-changes`
- `Verdict: needs-discussion`


## Fallback Behavior

When Ito CLI access is unavailable:

- Do not invent missing state.
- If a change ID is unknown, remain in `no-change-selected`.
- If worktrees are enabled and the current checkout role is unknown, avoid writes.
- If validation cannot be run, report it as not run.
- If a CLI-dependent step cannot execute, follow the surrounding rule text and mark the outcome as unverified.
- If the capability profile forbids mutation, provide proposed content, diffs, or instructions rather than editing files.

## Final Rule

Proceed only through valid state transitions. Preserve Ito artifacts as the durable record of intent, behavior, tasks, validation, review, and archive. When uncertain, state what is known, what is unknown, and which writes or claims are unsafe.
````
