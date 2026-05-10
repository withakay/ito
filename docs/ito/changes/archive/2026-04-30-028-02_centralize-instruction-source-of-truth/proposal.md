<!-- ITO:START -->
## Why

Ito workflow behavior has drifted into overlapping skills, agent prompts, and harness-specific prompt surfaces. This now creates two concrete problems: `ito-orchestrator` and `ito-general` are being generated as delegated sub-agents when they should be directly activatable entrypoints, and orchestration/multi-agent guidance is duplicated across too many skills and prompts.

## What Changes

- Define a canonical Ito agent surface taxonomy that separates directly activatable entrypoint agents from delegated role sub-agents.
- Make `ito-general` and `ito-orchestrator` direct entrypoint agents for supported harnesses, while keeping planner/researcher/worker/reviewer/test-runner style agents as delegated sub-agents.
- Consolidate overlapping orchestration and multi-agent skills/prompts into a smaller instruction-backed surface, with `ito agent instruction orchestrate` as the authoritative workflow source.
- Thin the orchestrate/orchestrator skills and agent prompts so they defer to rendered instruction artifacts instead of duplicating canonical workflow policy.
- Apply the same source-of-truth pattern to Ito memory: memory skills SHALL call memory instruction artifacts rather than embedding provider workflow detail directly.
- Add generated-template verification so future Ito-managed skills, commands, and agents cannot silently reintroduce duplicate orchestration policy or direct/delegated placement drift.
- Strengthen coordination-worktree repair behavior so worktree sync or initialization can create/repair the required `.ito` coordination symlinks instead of only warning that regular directories are invalid.

## Change Shape

- **Type**: refactor
- **Risk**: medium
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: This crosses instruction templates, shared skills, agent prompts, generated harness assets, and worktree coordination state; a design doc is needed to keep source-of-truth and activation boundaries explicit.

## Capabilities

### New Capabilities

- `instruction-source-of-truth`: Defines the repository-wide pattern that authoritative workflow content belongs in baked-in `ito agent instruction ...` artifacts, with skills and agents acting as thin loaders/adapters.
- `agent-surface-taxonomy`: Defines the canonical generated-agent inventory, including which Ito agents are direct entrypoints, which are delegated sub-agents, and how overlapping orchestration/multi-agent surfaces are consolidated.

### Modified Capabilities

- `orchestrate-instruction`: `ito agent instruction orchestrate` becomes the complete authoritative orchestrator instruction and names the direct orchestrator plus delegated role-agent model.
- `agent-memory-abstraction`: Ito memory skills defer to memory instruction artifacts and do not duplicate provider-specific operational workflow.
- `coordination-worktree`: Sync/initialization can repair expected coordination symlinks and emits actionable guidance when repair is unsafe.

## Impact

- Affected templates: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, especially `orchestrate.md.j2` and memory instruction templates.
- Affected skills: `ito-orchestrate`, `ito-orchestrator-workflow`, `ito-orchestrate-setup`, `ito-subagent-driven-development`, `ito-test-with-subagent`, `ito-memory`, and any overlapping Ito-managed orchestration/multi-agent skills.
- Affected agents: `ito-general`, `ito-orchestrator`, and delegated role agents across OpenCode, Claude Code, GitHub Copilot, Codex, and Pi harness templates.
- Affected installer behavior: generated harness assets must install direct entrypoints where the harness supports direct activation and delegated role prompts where the harness supports sub-agents.
- Affected CLI/worktree behavior: coordination sync and/or worktree initialization paths that validate `.ito/{changes,specs,modules,workflows,audit}` wiring.
<!-- ITO:END -->
