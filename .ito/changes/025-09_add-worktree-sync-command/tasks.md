<!-- ITO:START -->
# Tasks for: 025-09_add-worktree-sync-command

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 025-09_add-worktree-sync-command
ito tasks next 025-09_add-worktree-sync-command
ito tasks start 025-09_add-worktree-sync-command 1.1
ito tasks complete 025-09_add-worktree-sync-command 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add the `ito sync` CLI surface

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`, `ito-rs/crates/ito-cli/src/commands/sync.rs`
- **Dependencies**: None
- **Action**: Add the top-level `ito sync` command, include `--force`, and wire it to a thin CLI adapter that delegates to core coordination-sync logic.
- **Verify**: `cargo test -p ito-cli sync -- --nocapture`
- **Done When**: `ito sync` is a real CLI entry point with tests covering the command surface.
- **Requirements**: `cli-sync:coordination-worktree-sync`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.2: Implement core coordination sync orchestration

- **Files**: `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-core/src/coordination_worktree.rs`, `ito-rs/crates/ito-core/src/git.rs`, `ito-rs/crates/ito-core/src/repo_paths.rs`
- **Dependencies**: Task 1.1
- **Action**: Add the shared core sync flow that validates exact worktree wiring, detects duplicate local directories, fetches coordination state, auto-commits pending coordination-worktree artifacts, and pushes when safe.
- **Verify**: `cargo test -p ito-core coordination_worktree -- --nocapture`
- **Done When**: The CLI can call a single core sync path that enforces the new validation and push behavior.
- **Requirements**: `cli-sync:coordination-worktree-sync`, `coordination-worktree:exact-sync-wiring`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.3: Add sync metadata, quiet-window suppression, and force-bypass tests

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs`, `ito-rs/crates/ito-core/src/coordination_worktree_tests.rs`, `ito-rs/crates/ito-core/src/coordination_tests.rs`, `ito-rs/crates/ito-cli/src/commands/sync.rs`
- **Dependencies**: Task 1.2
- **Action**: Persist repo-local last-successful-sync metadata for redundant-push suppression, add `--force` bypass behavior, and cover unchanged repeated syncs, changed-state bypass, forced syncs, and divergence/error paths with tests.
- **Verify**: `cargo test -p ito-core coordination -- --nocapture`
- **Done When**: Repeated syncs validate locally but skip redundant remote pushes, `--force` bypasses the quiet window, and tests pin the intended behavior.
- **Requirements**: `cli-sync:quiet-rate-limited-sync`, `coordination-worktree:exact-sync-wiring`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.4: Add configurable sync interval defaults and config support

- **Files**: `ito-rs/crates/ito-core/src/config/defaults.rs`, `ito-rs/crates/ito-config/**`, `ito-rs/crates/ito-cli/src/commands/config.rs`, `schemas/ito-config.schema.json`
- **Dependencies**: Task 1.3
- **Action**: Add `changes.coordination_branch.sync_interval_seconds` with a default of `120`, add `changes.archive.main_integration_mode` with a default of `pull_request`, wire both through config loading and schema generation, and support setting them through `ito config`.
- **Verify**: `cargo test -p ito-cli config -- --nocapture`
- **Done When**: The effective sync interval defaults to 120 seconds, the default archive integration mode is `pull_request`, and both can be configured through the existing config surface.
- **Requirements**: `cli-sync:quiet-rate-limited-sync`, `config-defaults:coordination-sync-interval-default`, `config-defaults:archive-main-integration-mode-default`, `cli-config:coordination-sync-interval`, `cli-config:archive-main-integration-mode`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add coordination-first archive lifecycle behavior

- **Files**: `ito-rs/crates/ito-cli/src/commands/archive.rs`, `ito-rs/crates/ito-core/src/archive.rs`, `ito-rs/crates/ito-core/src/coordination_worktree.rs`, `ito-rs/crates/ito-core/src/git.rs`
- **Dependencies**: None
- **Action**: Update archive orchestration so worktree-mode archive happens on the coordination branch first, blocks `main` integration on failed dissemination, and records the appropriate follow-up state.
- **Verify**: `cargo test -p ito-cli archive -- --nocapture`
- **Done When**: Worktree-mode archive archives on the coordination branch first and only proceeds to configured `main` integration after that succeeds.
- **Requirements**: `cli-archive:coordination-first-archive`, `cli-archive:main-integration-mode`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: Add sync-aware, archive-aware, and finish-aware CLI instruction templates

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-core/src/templates/**/*.rs`, `ito-rs/crates/ito-templates/**/*`
- **Dependencies**: Task 2.1
- **Action**: Update the CLI-generated instruction templates so worktree-backed flows tell agents when to run `ito sync` before mutation-sensitive or handoff-sensitive work, so archive guidance follows the configured `main` integration mode, and so finish guidance asks `Do you want to archive this change now?`.
- **Verify**: `cargo test -p ito-cli instructions -- --nocapture`
- **Done When**: Generated instruction output includes the new sync guidance, mode-specific archive guidance, the finish archive prompt, and template-backed coverage for the relevant worktree-aware flows.
- **Requirements**: `agent-instructions:sync-aware-worktree-guidance`, `agent-instructions:archive-integration-guidance`, `agent-instructions:templated-cli-workflows`, `agent-instructions:finish-archive-prompt`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.3: Update mirrored Ito skills to invoke sync and archive follow-up at the right touchpoints

- **Files**: `.opencode/skills/ito/SKILL.md`, `.opencode/skills/ito-apply/SKILL.md`, `.opencode/skills/ito-finish/SKILL.md`, `.opencode/skills/ito-commit/SKILL.md`, `.opencode/skills/ito-using-git-worktrees/SKILL.md`, `.claude/skills/ito*/SKILL.md`
- **Dependencies**: Task 2.2
- **Action**: Update the relevant OpenCode and Claude skill prompts so they call `ito sync` at mutation and handoff points and so `ito-archive` and `ito-finish` delegate to the CLI-generated archive/finish instructions instead of embedding their own workflow logic.
- **Verify**: `ito validate 025-09_add-worktree-sync-command --strict`
- **Done When**: The mirrored skill families consistently refer to `ito sync` at the intended workflow boundaries and delegate archive/finish behavior to the CLI-generated instructions.
- **Requirements**: `agent-instructions:sync-aware-worktree-guidance`, `agent-instructions:archive-integration-guidance`, `agent-instructions:templated-cli-workflows`, `agent-instructions:finish-archive-prompt`, `cli-sync:quiet-rate-limited-sync`, `cli-archive:main-integration-mode`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Replace scattered best-effort coordination sync hooks with the shared flow

- **Files**: `ito-rs/crates/ito-cli/src/commands/create.rs`, `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: None
- **Action**: Update existing call sites that currently perform ad hoc coordination fetches so they route through the new shared sync behavior where the proposal requires it.
- **Verify**: `cargo test -p ito-cli create tasks instructions -- --nocapture`
- **Done When**: Existing worktree-aware entry points use one consistent sync mechanism instead of scattered fetch-only logic.
- **Requirements**: `cli-sync:coordination-worktree-sync`, `cli-sync:quiet-rate-limited-sync`, `agent-instructions:sync-aware-worktree-guidance`, `cli-config:coordination-sync-interval`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: Wire archive and finish follow-up behavior to the configured integration mode

- **Files**: `ito-rs/crates/ito-cli/src/commands/archive.rs`, `ito-rs/crates/ito-cli/src/app/instructions.rs`, `.opencode/skills/ito-archive/SKILL.md`, `.claude/skills/ito-archive/SKILL.md`
- **Dependencies**: Task 3.1
- **Action**: Ensure archive follow-up behavior, finish prompting behavior, and archive/finish instructions use `changes.archive.main_integration_mode` consistently for direct-merge, PR, PR auto-merge, and coordination-only flows.
- **Verify**: `cargo test -p ito-cli archive instructions -- --nocapture`
- **Done When**: Archive-related command output and finish/agent guidance consistently follow the configured integration mode and the finish archive prompt behaves correctly.
- **Requirements**: `cli-archive:main-integration-mode`, `agent-instructions:archive-integration-guidance`, `agent-instructions:finish-archive-prompt`, `cli-config:archive-main-integration-mode`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.3: Run full validation for the change

- **Files**: `.ito/changes/025-09_add-worktree-sync-command/**`, `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `.opencode/skills/**`, `.claude/skills/**`
- **Dependencies**: Task 3.2
- **Action**: Run the proposal validation and the relevant Rust test/check commands, then fix any spec or implementation drift uncovered by those checks.
- **Verify**: `ito validate 025-09_add-worktree-sync-command --strict`
- **Done When**: The change validates cleanly and the implementation verification commands selected during apply are ready to run.
- **Requirements**: `cli-sync:coordination-worktree-sync`, `cli-sync:quiet-rate-limited-sync`, `coordination-worktree:exact-sync-wiring`, `agent-instructions:sync-aware-worktree-guidance`, `agent-instructions:archive-integration-guidance`, `agent-instructions:templated-cli-workflows`, `agent-instructions:finish-archive-prompt`, `config-defaults:coordination-sync-interval-default`, `config-defaults:archive-main-integration-mode-default`, `cli-config:coordination-sync-interval`, `cli-config:archive-main-integration-mode`, `cli-archive:coordination-first-archive`, `cli-archive:main-integration-mode`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
