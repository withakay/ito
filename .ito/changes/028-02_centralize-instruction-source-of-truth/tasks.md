<!-- ITO:START -->
# Tasks for: 028-02_centralize-instruction-source-of-truth

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 028-02_centralize-instruction-source-of-truth
ito tasks next 028-02_centralize-instruction-source-of-truth
ito tasks start 028-02_centralize-instruction-source-of-truth 1.1
ito tasks complete 028-02_centralize-instruction-source-of-truth 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define generated surface inventory

- **Files**: `ito-rs/crates/ito-templates/assets/agents/**`, `ito-rs/crates/ito-templates/assets/skills/ito-*/SKILL.md`, `ito-rs/crates/ito-templates/src/lib.rs`, related template tests
- **Dependencies**: None
- **Action**: Inventory Ito-managed agents, skills, commands, and prompts involved in orchestration, multi-agent execution, memory, and instruction-backed workflows; encode the canonical direct/delegated/adapter/project-guidance/deprecated classification in tests or template metadata.
- **Verify**: `cargo test -p ito-templates agent_templates_are_embedded_for_all_harnesses`
- **Done When**: The generated surface inventory identifies `ito-general` and `ito-orchestrator` as direct entrypoints, planner/researcher/worker/reviewer/test-runner as delegated roles, and every orchestration-adjacent skill/prompt has a non-overlapping classification.
- **Requirements**: instruction-source-of-truth:canonical-surface-inventory, agent-surface-taxonomy:activation-mode, agent-surface-taxonomy:direct-general-orchestrator, agent-surface-taxonomy:delegated-role-agents
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 1.2: Expand authoritative instruction templates

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/orchestrate.md.j2`, memory instruction templates under `ito-rs/crates/ito-templates/assets/instructions/agent/`, related instruction tests
- **Dependencies**: Task 1.1
- **Action**: Move canonical orchestrate and memory workflow detail into baked-in instruction templates and ensure rendered output includes source-of-truth precedence, direct orchestrator activation, delegated roles, gates, run state, remediation, resume behavior, and provider-operation guidance.
- **Verify**: `cargo test -p ito-templates instructions_tests` and `cargo test -p ito-cli --test agent_instruction_orchestrate`
- **Done When**: Rendered instructions contain the authoritative workflow detail currently spread across skills/agents and tests cover direct entrypoints, delegated roles, and expected sections.
- **Requirements**: instruction-source-of-truth:authoritative-artifacts, orchestrate-instruction:artifact-type, agent-memory-abstraction:installed-ito-memory-skill, agent-surface-taxonomy:direct-general-orchestrator, agent-surface-taxonomy:delegated-role-agents
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 1.3: Reclassify generated agent templates

- **Files**: `ito-rs/crates/ito-templates/assets/agents/**/ito-general*`, `ito-rs/crates/ito-templates/assets/agents/**/ito-orchestrator*`, delegated role agent templates, installer tests for OpenCode/Claude Code/GitHub Copilot/Codex/Pi
- **Dependencies**: Task 1.1
- **Action**: Install/render `ito-general` and `ito-orchestrator` as direct entrypoint agents where harnesses support direct activation, keep delegated role agents in sub-agent locations or with delegated metadata, and add harness-specific tests for the mapping.
- **Verify**: `cargo test -p ito-templates orchestrator_agent_templates_are_embedded_for_all_harnesses` and focused installer tests for generated agent destinations
- **Done When**: Supported harness templates expose `ito-general` and `ito-orchestrator` for direct activation and delegated role agents remain narrowly scoped sub-agents.
- **Requirements**: agent-surface-taxonomy:activation-mode, agent-surface-taxonomy:direct-general-orchestrator, agent-surface-taxonomy:delegated-role-agents, instruction-source-of-truth:harness-template-boundary
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 1.4: Thin and consolidate orchestration skills and prompts

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-orchestrate/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-orchestrator-workflow/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-subagent-driven-development/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-test-with-subagent/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-memory/SKILL.md`, related command/prompt templates and tests
- **Dependencies**: Task 1.2
- **Action**: Replace duplicated workflow policy in skills, commands, and role prompts with instruction invocation and role-local guidance; remove or deprecate obsolete Ito-managed orchestration/multi-agent surfaces with safe installer cleanup where appropriate.
- **Verify**: `cargo test -p ito-templates orchestrate_skills_and_command_are_embedded orchestrator_agent_templates_are_embedded_for_all_harnesses`
- **Done When**: Skills and prompts consistently defer to rendered instruction artifacts, duplicate orchestration/multi-agent policy is removed, and any retained specialized skill has a distinct documented trigger.
- **Requirements**: instruction-source-of-truth:thin-adapters, instruction-source-of-truth:harness-template-boundary, instruction-source-of-truth:canonical-surface-inventory, agent-surface-taxonomy:orchestration-consolidation
- **Updated At**: 2026-04-29
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Repair coordination symlinks during worktree sync/init

- **Files**: coordination worktree sync/init code in `ito-rs/crates/ito-core/src/` and CLI command paths that call it, related `ito-core` and `ito-cli` tests
- **Dependencies**: None
- **Action**: Add safe repair for missing coordination symlinks and empty generated directories; preserve hard failures for wrong symlinks and non-empty duplicate directories with actionable remediation output.
- **Verify**: `cargo test -p ito-core coordination_worktree` and focused CLI/worktree tests covering init/sync repair
- **Done When**: New worktrees can be repaired automatically when safe, and unsafe cases produce exact path/target guidance.
- **Requirements**: coordination-worktree:exact-sync-wiring
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 2.2: Verify generated installs and full quality gate

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`, `ito-rs/crates/ito-cli/tests/init_more.rs`, generated template assertions as needed
- **Dependencies**: Task 2.1
- **Action**: Add or update tests that install/update managed files and confirm orchestrate/memory skills, direct entrypoint agents, delegated role agents, and obsolete asset cleanup preserve the instruction-source boundary. Run the project quality gate.
- **Verify**: `make check`
- **Done When**: Focused tests and `make check` pass, and installed template content matches the source-of-truth and agent-surface taxonomy design.
- **Requirements**: instruction-source-of-truth:harness-template-boundary, instruction-source-of-truth:canonical-surface-inventory, agent-surface-taxonomy:activation-mode, agent-surface-taxonomy:orchestration-consolidation, coordination-worktree:exact-sync-wiring
- **Updated At**: 2026-04-29
- **Status**: [>] in-progress

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave.
- Wave 2 depends on Wave 1 completing so implementation and tests can rely on the new source-of-truth and agent-surface boundaries.
- Use `ito tasks` for status changes during implementation.
<!-- ITO:END -->
