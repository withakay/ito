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

### Task 1.1: Expand authoritative instruction templates

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/orchestrate.md.j2`, memory instruction templates under `ito-rs/crates/ito-templates/assets/instructions/agent/`, related instruction tests
- **Dependencies**: None
- **Action**: Move canonical orchestrate and memory workflow detail into baked-in instruction templates and ensure rendered output includes source-of-truth precedence, role expectations, gates, run state, remediation, and provider-operation guidance.
- **Verify**: `cargo test -p ito-templates instructions_tests` and `cargo test -p ito-cli --test agent_instruction_orchestrate`
- **Done When**: Rendered instructions contain the authoritative workflow detail currently spread across skills/agents and tests cover the expected sections.
- **Requirements**: instruction-source-of-truth:authoritative-artifacts, orchestrate-instruction:artifact-type, agent-memory-abstraction:installed-ito-memory-skill
- **Updated At**: 2026-04-28
- **Status**: [ ] pending

### Task 1.2: Thin skills and agent prompts

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-orchestrate/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-memory/SKILL.md`, `ito-rs/crates/ito-templates/assets/agents/**/ito-orchestrator*.md`, related template tests
- **Dependencies**: Task 1.1
- **Action**: Replace duplicated workflow policy in skills and orchestrator agents with instruction invocation and role-local guidance. Keep skills as discovery/setup adapters and agents as role prompts.
- **Verify**: `cargo test -p ito-templates orchestrate_skills_and_command_are_embedded orchestrator_agent_templates_are_embedded_for_all_harnesses`
- **Done When**: Skills and agents consistently defer to the rendered instruction artifact and no longer contain conflicting canonical workflow policy.
- **Requirements**: instruction-source-of-truth:thin-adapters, instruction-source-of-truth:harness-template-boundary
- **Updated At**: 2026-04-28
- **Status**: [ ] pending

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
- **Updated At**: 2026-04-28
- **Status**: [ ] pending

### Task 2.2: Verify generated installs and full quality gate

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`, `ito-rs/crates/ito-cli/tests/init_more.rs`, generated template assertions as needed
- **Dependencies**: Task 2.1
- **Action**: Add or update tests that install/update managed files and confirm orchestrate/memory skills plus orchestrator agents preserve the instruction-source boundary. Run the project quality gate.
- **Verify**: `make check`
- **Done When**: Focused tests and `make check` pass, and installed template content matches the source-of-truth design.
- **Requirements**: instruction-source-of-truth:harness-template-boundary, coordination-worktree:exact-sync-wiring
- **Updated At**: 2026-04-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave.
- Wave 2 depends on Wave 1 completing so implementation and tests can rely on the new source-of-truth boundaries.
- Use `ito tasks` for status changes during implementation.
<!-- ITO:END -->
