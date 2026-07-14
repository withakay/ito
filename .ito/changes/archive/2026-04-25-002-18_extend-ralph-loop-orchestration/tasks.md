<!-- ITO:START -->
# Tasks for: 002-18_extend-ralph-loop-orchestration

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 002-18_extend-ralph-loop-orchestration
ito tasks next 002-18_extend-ralph-loop-orchestration
ito tasks start 002-18_extend-ralph-loop-orchestration 1.1
ito tasks complete 002-18_extend-ralph-loop-orchestration 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement change execution context assembly

- **Files**: `ito-rs/crates/ito-core/src/ralph/prompt.rs`, `ito-rs/crates/ito-core/src/ralph/validation.rs`, related Ralph prompt tests
- **Dependencies**: None
- **Action**: Expand Ralph prompt construction so change-scoped runs include proposal context, task progress, next actionable work, and Ito-native execution guidance derived from the targeted change.
- **Verify**: `cargo test -p ito-core ralph`
- **Done When**: Change-scoped Ralph prompts no longer rely on ad hoc user prompts to expose task progress and execution guidance.
- **Requirements**: ralph-execution-context:change-scoped-context, ralph-execution-context:ito-execution-guidance, ralph-execution-context:preserve-loop-context
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 1.2: Implement queue execution behavior and aggregate summaries

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`, `ito-rs/crates/ito-core/tests/ralph.rs`, related CLI smoke tests
- **Dependencies**: Task 1.1
- **Action**: Update continue-ready and continue-module execution so Ralph records per-change outcomes, continues through eligible work after per-change failures, and returns aggregate results at the end.
- **Verify**: `cargo test -p ito-core ralph && cargo test -p ito-cli ralph_smoke`
- **Done When**: Queue-style Ralph runs are fail-soft, ordered, and report aggregate success/failure clearly.
- **Requirements**: ralph-queue-execution:continue-ready-sweep, ralph-queue-execution:continue-module-sweep, ralph-queue-execution:per-change-outcomes
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 1.3: Expand persisted state and status output

- **Files**: `ito-rs/crates/ito-core/src/ralph/state.rs`, `ito-rs/crates/ito-core/src/ralph/runner.rs`, `ito-rs/crates/ito-core/tests/ralph.rs`
- **Dependencies**: Task 1.1
- **Action**: Extend Ralph history and status reporting to capture actionable run outcomes, restart-relevant details, and current task summary for targeted changes.
- **Verify**: `cargo test -p ito-core ralph`
- **Done When**: `ito ralph --status` and persisted state expose enough information to derive restart summaries and inspect the last run outcome.
- **Requirements**: ralph-run-reporting:actionable-run-state, ralph-run-reporting:status-supports-resume, ralph-run-reporting:restart-summary-from-state
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Align `/ito-loop` wrapper behavior with the shipped contract

- **Files**: `ito-rs/crates/ito-templates/assets/commands/ito-loop.md`, `ito-rs/crates/ito-templates/assets/skills/ito-loop/SKILL.md`, harness-installed variants, related template tests
- **Dependencies**: None
- **Action**: Update the installed `ito-loop` command and skill so the path, invocation examples, safe defaults, and restart-context behavior match the intended OpenCode wrapper contract.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed loop wrapper and its spec agree on the command path, supported target modes, and bounded restart behavior.
- **Requirements**: opencode-loop-command:ito-loop-command, opencode-loop-command:restart-context, opencode-loop-command:model-override
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 2.2: Refresh QA coverage around proposal/apply style loop runs

- **Files**: `qa/ralph/test-ralph-loop.sh`, `ito-rs/crates/ito-cli/tests/ralph_smoke.rs`, relevant snapshots/tests
- **Dependencies**: Task 2.1
- **Action**: Update Ralph QA and smoke coverage so proposal/apply-style runs succeed based on built-in execution context rather than fragile hand-authored prompt wording.
- **Verify**: `cargo test -p ito-cli ralph_smoke`
- **Done When**: Ralph integration tests assert the new execution-context and wrapper behavior explicitly.
- **Requirements**: ralph-execution-context:change-scoped-context, ralph-run-reporting:status-supports-resume, opencode-loop-command:restart-context
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add task-source orchestration modes

- **Files**: `ito-rs/crates/ito-cli/src/{cli,commands}/ralph.rs`, `ito-rs/crates/ito-core/src/ralph/`, related task-source parsing/helpers
- **Dependencies**: None
- **Action**: Add markdown, YAML, and GitHub task-source support to Ralph orchestration, including source parsing and completion-state sync hooks.
- **Verify**: `cargo test -p ito-cli ralph_smoke`
- **Done When**: Ralph can orchestrate work from external task sources in addition to change-scoped Ito runs.
- **Requirements**: ralph-task-sources:multiple-task-sources, ralph-task-sources:sync-external-state
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 3.2: Add git branch and PR automation modes

- **Files**: `ito-rs/crates/ito-core/src/ralph/`, `ito-rs/crates/ito-cli/src/{cli,commands}/ralph.rs`, related git/process helpers
- **Dependencies**: Task 3.1
- **Action**: Add branch-per-task and optional PR automation behavior for Ralph orchestration modes that opt into git automation.
- **Verify**: `cargo test -p ito-core ralph && cargo test -p ito-cli ralph_smoke`
- **Done When**: Ralph can create task branches and optionally open PRs when configured.
- **Requirements**: ralph-git-automation:branch-per-task, ralph-git-automation:create-pr
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 3.3: Add parallel orchestration support

- **Files**: `ito-rs/crates/ito-core/src/ralph/`, `ito-rs/crates/ito-cli/src/{cli,commands}/ralph.rs`, worktree/process helpers
- **Dependencies**: Task 3.1
- **Action**: Add parallel worker orchestration with isolated worktrees and grouped task-batch behavior.
- **Verify**: `cargo test -p ito-core ralph`
- **Done When**: Ralph can run eligible grouped work concurrently within configured limits.
- **Requirements**: ralph-parallel-execution:isolated-worktrees, ralph-parallel-execution:grouped-batches
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Add browser automation and notification capabilities

- **Files**: `ito-rs/crates/ito-core/src/ralph/`, `ito-rs/crates/ito-cli/src/{cli,commands}/ralph.rs`, template/wrapper assets as needed
- **Dependencies**: None
- **Action**: Add optional browser automation prompt guidance and operator notification behavior for orchestrated Ralph runs.
- **Verify**: `cargo test -p ito-core ralph && cargo test -p ito-templates`
- **Done When**: Ralph can enable browser and notification capabilities when configured and supported by the environment.
- **Requirements**: ralph-runtime-capabilities:browser-automation, ralph-runtime-capabilities:operator-notifications
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Validate the full Ralph proposal implementation surface

- **Files**: `ito-rs/crates/ito-core/src/ralph/`, `ito-rs/crates/ito-cli/src/{cli,commands}/ralph.rs`, template assets and tests
- **Dependencies**: None
- **Action**: Run the relevant Ralph-focused core, CLI, and template checks and resolve any regressions introduced by the orchestration changes.
- **Verify**: `make check`
- **Done When**: Core, CLI, and template verification all pass for the Ralph orchestration change.
- **Requirements**: ralph-execution-context:change-scoped-context, ralph-queue-execution:continue-ready-sweep, ralph-run-reporting:actionable-run-state, opencode-loop-command:restart-context
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 5.2: Validate the proposal package strictly

- **Files**: `.ito/changes/002-18_extend-ralph-loop-orchestration/`
- **Dependencies**: None
- **Action**: Run strict Ito validation and repair any traceability or artifact-format problems in the proposal package.
- **Verify**: `ito validate 002-18_extend-ralph-loop-orchestration --strict`
- **Done When**: The proposal package validates with no strict-mode errors.
- **Requirements**: ralph-execution-context:change-scoped-context, ralph-queue-execution:per-change-outcomes, ralph-run-reporting:status-supports-resume, opencode-loop-command:ito-loop-command
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
