<!-- ITO:START -->
## Why

Ito workflow behavior has drifted into skills and agent prompts, making skills and agents partial sources of truth instead of lightweight entrypoints. Agents need a consistent pattern where baked-in `ito agent instruction ...` artifacts and their templates carry authoritative workflow detail, while skills and agents primarily discover, render, and follow those instructions.

## What Changes

- Expand `ito agent instruction orchestrate` into the authoritative orchestrator workflow instruction, including planning, roles, gate order, run state, remediation, and project prompt handling.
- Thin the orchestrate/orchestrator skills and agent prompts so they defer to the rendered orchestrate instruction instead of duplicating canonical workflow policy.
- Apply the same source-of-truth pattern to Ito memory: memory skills SHALL call memory instruction artifacts rather than embedding provider workflow detail directly.
- Add a general convention for future Ito skills and agents: if a baked-in instruction artifact exists for a workflow, the skill/agent is an adapter that invokes and follows it.
- Strengthen coordination-worktree repair behavior so worktree sync or initialization can create/repair the required `.ito` coordination symlinks instead of only warning that regular directories are invalid.

## Change Shape

- **Type**: refactor
- **Risk**: medium
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: This crosses instruction templates, shared skills, agent prompts, and worktree coordination state; a design doc is needed to keep source-of-truth boundaries explicit.

## Capabilities

### New Capabilities

- `instruction-source-of-truth`: Defines the repository-wide pattern that authoritative workflow content belongs in baked-in `ito agent instruction ...` artifacts, with skills and agents acting as thin loaders/adapters.

### Modified Capabilities

- `orchestrate-instruction`: `ito agent instruction orchestrate` becomes the complete authoritative orchestrator instruction instead of a light wrapper around skills.
- `agent-memory-abstraction`: Ito memory skills defer to memory instruction artifacts and do not duplicate provider-specific operational workflow.
- `coordination-worktree`: Sync/initialization can repair expected coordination symlinks and emits actionable guidance when repair is unsafe.

## Impact

- Affected templates: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, especially `orchestrate.md.j2` and memory instruction templates.
- Affected skills: `ito-orchestrate`, `ito-orchestrator-workflow`, `ito-memory`, and any orchestrator role skills under template assets.
- Affected agents: orchestrator coordinator/planner/researcher/worker/reviewer prompts across OpenCode, Claude Code, GitHub Copilot, and Pi harness templates.
- Affected CLI/worktree behavior: coordination sync and/or worktree initialization paths that validate `.ito/{changes,specs,modules,workflows,audit}` wiring.
<!-- ITO:END -->
