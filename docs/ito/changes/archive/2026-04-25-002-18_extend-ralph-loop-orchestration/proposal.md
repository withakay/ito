<!-- ITO:START -->
## Why

Ito's `ralph` loop already has a solid core engine, but it still lacks many of the execution-context, queueing, restart, and operator-visibility behaviors that make the upstream `ralphy.sh` workflow effective for autonomous work. We need to absorb the useful Ralphy behaviors into Ito's change-centric workflow now that the Rust Ralph runtime, worktree awareness, and harness support are established.

## What Changes

- Add a first-class change execution context for Ralph so change-scoped runs include proposal, task progress, next actionable work, and Ito-native execution guidance instead of relying on ad hoc prompts.
- Add explicit queue-execution behavior for `--continue-ready` and `--continue-module`, including per-change result tracking and fail-soft continuation across eligible changes.
- Add richer Ralph run reporting and status surfaces so restart context, iteration history, failure reasons, and effective working directory are visible and reusable.
- Add task-source modes so Ralph can operate from Ito change context, markdown task files, YAML task files, and GitHub issue queues.
- Add git automation options for branch-per-task and optional PR creation during orchestrated Ralph runs.
- Add full parallel orchestration with isolated worktree execution for independent tasks or grouped task batches.
- Add optional browser automation and operator notification capabilities for parity with the upstream reference workflow.
- Align the installed `/ito-loop` wrapper contract with the actual shipped command path, safe defaults, and restart-context behavior.
- Capture the upstream `ralphy.reference.sh` feature matrix in the design, and explicitly distinguish Ito-native parity goals from out-of-scope standalone workflow features.

## Capabilities

### New Capabilities

- `ralph-execution-context`: Define the Ito-native context Ralph must inject for change-scoped autonomous execution.
- `ralph-queue-execution`: Define how Ralph processes multiple eligible changes in continue-ready and continue-module flows.
- `ralph-run-reporting`: Define persisted run history, status output, and restart-summary behavior for Ralph loops.
- `ralph-task-sources`: Define markdown, YAML, GitHub issue, and change-scoped task sourcing for Ralph orchestration.
- `ralph-git-automation`: Define branch-per-task and PR automation behavior for Ralph orchestration.
- `ralph-parallel-execution`: Define parallel Ralph orchestration with isolated worktrees and grouped task execution.
- `ralph-runtime-capabilities`: Define optional browser automation and operator notification integrations.

### Modified Capabilities

- `opencode-loop-command`: Update the installed loop command contract to match the shipped `/ito-loop` behavior and bounded restart-context orchestration.

## Impact

- **Affected code**: `ito-rs/crates/ito-core/src/ralph/{prompt,runner,state,validation}.rs`, `ito-rs/crates/ito-cli/src/{cli/ralph.rs,commands/ralph.rs}`, plus related harness/runtime utilities for task-source parsing, git automation, and optional integrations
- **Affected template assets**: `ito-rs/crates/ito-templates/assets/commands/ito-loop.md`, `ito-rs/crates/ito-templates/assets/skills/ito-loop/SKILL.md`, and corresponding harness-installed variants
- **Affected QA**: `qa/ralph/test-ralph-loop.sh`, Ralph CLI/core tests, and any status/help snapshots covering Ralph behavior
- **Behavioral impact**: Ralph becomes a more self-contained autonomous change executor with clearer restart semantics, richer status visibility, and safer multi-change continuation behavior
<!-- ITO:END -->
