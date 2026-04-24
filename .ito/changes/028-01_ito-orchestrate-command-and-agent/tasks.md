<!-- ITO:START -->
# Tasks for: 028-01_ito-orchestrate-command-and-agent

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 028-01_ito-orchestrate-command-and-agent
ito tasks next 028-01_ito-orchestrate-command-and-agent
ito tasks start 028-01_ito-orchestrate-command-and-agent 1.1
ito tasks complete 028-01_ito-orchestrate-command-and-agent 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add orchestrate instruction artifact surface

- **Files**: `ito-rs/crates/ito-cli/src/cli/agent.rs`, `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/orchestrate.md.j2`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add `ito agent instruction orchestrate`, wire template rendering and JSON output, and return setup guidance when `orchestrate.md` is missing.
- **Verify**: `cargo test -p ito-cli instruction -- --nocapture`
- **Done When**: The CLI recognizes `orchestrate` as a valid instruction artifact, renders the new template, and has coverage for help/output behavior.
- **Requirements**: orchestrate-instruction:artifact-type, agent-instructions:orchestrate-artifact
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.2: Add structured orchestrate user-prompt parsing and preset loading

- **Files**: `ito-rs/crates/ito-core/src/orchestrate/**`, `ito-rs/crates/ito-core/src/templates/**`, `ito-rs/crates/ito-templates/assets/presets/orchestrate/*.yaml`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: Task 1.1
- **Action**: Add core parsing for `orchestrate.md` front matter and markdown sections, load built-in preset YAML files, and expose resolved orchestrate instruction context to the CLI/template layer.
- **Verify**: `cargo test -p ito-core orchestrate -- --nocapture`
- **Done When**: Core can parse `orchestrate.md`, resolve a preset, and provide deterministic render context for the orchestrate instruction.
- **Requirements**: orchestrate-user-prompt:schema, orchestrate-user-prompt:gate-overrides, orchestrate-presets:library, orchestrate-presets:agent-roles, orchestrate-workflow-skill:convention-load, orchestrate-workflow-skill:living-doc
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.3: Expose orchestrate metadata through change repository models

- **Files**: `ito-rs/crates/ito-domain/src/changes/mod.rs`, `ito-rs/crates/ito-core/src/change_repository.rs`, `ito-rs/crates/ito-core/src/templates/mod.rs`, `ito-rs/crates/ito-core/src/task_repository.rs`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: Task 1.2
- **Action**: Replace ad hoc `.ito.yaml` schema scanning with shared structured metadata parsing and expose `orchestrate.depends_on` plus `orchestrate.preferred_gates` on change models and summaries.
- **Verify**: `cargo test -p ito-core change_repository -- --nocapture`
- **Done When**: Filesystem-backed change loads expose the new orchestrate metadata fields and existing schema lookups still work.
- **Requirements**: change-repository:lifecycle-aware-canonical-access, orchestrate-parallelism:dependency-graph
- **Updated At**: 2026-04-24
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement orchestrate planning, run state, and gate model in core

- **Files**: `ito-rs/crates/ito-core/src/orchestrate/**`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: None
- **Action**: Implement run id generation, dependency-aware planning, run-state read/write helpers, append-only event logging, default gate sequencing, and remediation packet construction.
- **Verify**: `cargo test -p ito-core orchestrate -- --nocapture`
- **Done When**: Core can build a plan from change metadata, persist run state under `.ito/.state/orchestrate/runs/<run-id>/`, and resume from prior state.
- **Requirements**: orchestrate-run-state:layout, orchestrate-run-state:event-log, orchestrate-run-state:resumability, orchestrate-gates:pipeline, orchestrate-gates:semantics, orchestrate-gates:remediation, orchestrate-parallelism:max-parallel-flag, orchestrate-parallelism:dependency-graph
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: Add setup guidance and setup skill/template assets

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-orchestrate/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-orchestrate-setup/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-orchestrator-workflow/SKILL.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/orchestrate.md`, `ito-rs/crates/ito-templates/assets/commands/ito-orchestrate.md`, `ito-rs/crates/ito-templates/assets/agents/opencode/ito-orchestrator.md`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: Task 2.1
- **Action**: Add the orchestrator skill family, workflow skill scaffold, default `orchestrate.md` prompt stub, and harness command/agent assets used by setup and execution.
- **Verify**: `cargo test -p ito-templates -- --nocapture`
- **Done When**: Template assets for orchestrate setup and execution are embedded and covered by template tests.
- **Requirements**: orchestrate-setup:first-run-detection, orchestrate-setup:stack-detection, orchestrate-setup:cross-reference, orchestrate-setup:outputs, orchestrate-workflow-skill:convention-load, orchestrate-workflow-skill:living-doc
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Wire installer/default project outputs and end-to-end instruction rendering

- **Files**: `ito-rs/crates/ito-core/src/installers/**`, `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: None
- **Action**: Ensure init/update/install flows include the new prompt and skill assets, add end-to-end tests for first-run guidance and rendered orchestrate output, and keep JSON/text outputs deterministic.
- **Verify**: `cargo test -p ito-core installers -- --nocapture && cargo test -p ito-cli instruction -- --nocapture`
- **Done When**: Newly initialized projects include the orchestrate assets and the orchestrate instruction path is covered by integration tests.
- **Requirements**: orchestrate-instruction:artifact-type, orchestrate-setup:outputs, agent-instructions:orchestrate-artifact
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: Full verification and demos

- **Files**: `.ito/changes/028-01_ito-orchestrate-command-and-agent/**`, `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `ito-rs/crates/ito-domain/**`, `ito-rs/crates/ito-templates/**`
- **Dependencies**: Task 3.1
- **Action**: Run strict change validation, crate-level tests/checks, create Showboat demos for the logical implementation batches, and fix any drift uncovered by verification.
- **Verify**: `ito validate 028-01_ito-orchestrate-command-and-agent --strict`
- **Done When**: The change validates cleanly, verification commands pass, and demos exist for the completed work.
- **Requirements**: orchestrate-instruction:artifact-type, orchestrate-user-prompt:schema, orchestrate-user-prompt:gate-overrides, orchestrate-run-state:layout, orchestrate-run-state:event-log, orchestrate-run-state:resumability, orchestrate-gates:pipeline, orchestrate-gates:semantics, orchestrate-gates:remediation, orchestrate-presets:library, orchestrate-presets:agent-roles, orchestrate-setup:first-run-detection, orchestrate-setup:stack-detection, orchestrate-setup:cross-reference, orchestrate-setup:outputs, orchestrate-workflow-skill:convention-load, orchestrate-workflow-skill:living-doc, orchestrate-parallelism:max-parallel-flag, orchestrate-parallelism:dependency-graph, agent-instructions:orchestrate-artifact, change-repository:lifecycle-aware-canonical-access
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
