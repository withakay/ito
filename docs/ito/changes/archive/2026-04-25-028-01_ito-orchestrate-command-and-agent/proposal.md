<!-- ITO:START -->
## Why

Applying a change proposal today is a single-agent affair: one agent applies, reviews, and gates its own work, which collapses the quality signal. As change counts grow, there is no way to fan out work across a set of changes, enforce consistent gate ordering, or track run state in a machine-readable way. The orchestrator closes this gap by introducing a dedicated coordinator that manages the full apply lifecycle — parallelism, gate sequencing, remediation, and run state — without ever writing code itself.

## What Changes

- New `ito agent instruction orchestrate` artifact type — renders an instruction document that drives the orchestrator agent
- New `ito-orchestrator-workflow` skill scaffold in templates — generated per project by the setup wizard, evolved by the user; loaded by the orchestrator by convention (no configuration required)
- New setup wizard skill (`ito-orchestrate-setup`) — agent-driven, not CLI-driven; detects stack, cross-references available skills and agents, recommends a preset, and generates `orchestrate.md` + the `ito-orchestrator-workflow` skill for the project
- New `orchestrate.md` user prompt format — YAML front matter (parallelism, failure policy, gate overrides) + `## MUST` / `## PREFER` / `## Notes` markdown sections
- New per-change orchestration metadata fields in `.ito/changes/<id>/.ito.yaml` — `depends_on` and `preferred_gates` for static per-change policy
- New run state layout at `.ito/.state/orchestrate/runs/<run-id>/` — `run.json`, `plan.json`, `events.jsonl` (append-only), `changes/<id>.json` per-change gate results
- New built-in presets in templates — `rust`, `typescript`, `python`, `go`, `generic`; each is a YAML file specifying gate config, recommended skills, and agent role suggestions
- Worker agent definitions (thin role wrappers) — `ito-apply-worker`, `ito-review-worker`, `ito-security-worker`; presented to the user as suggestions, not auto-wired

## Capabilities

### New Capabilities

- `orchestrate-instruction`: The `ito agent instruction orchestrate` artifact — renders the full orchestrator instruction document from `orchestrate.md.j2`, injecting project config, per-change metadata, and detected run context
- `orchestrate-user-prompt`: The `orchestrate.md` user prompt schema — YAML front matter + structured markdown sections that the orchestrator reads to configure parallelism, gate order, failure policy, and gate overrides
- `orchestrate-run-state`: Run state layout and lifecycle — `run.json` (run metadata), `plan.json` (resolved execution plan), `events.jsonl` (append-only event log), `changes/<id>.json` (per-change gate results); read by the orchestrator to resume interrupted runs and emit progress
- `orchestrate-gates`: Gate sequencing, remediation, and policy — ordered gate pipeline (`apply-complete → format → lint → tests → style → code-review → security-review`), per-gate pass/fail/skip semantics, remediation packet dispatch to a fresh apply worker on gate failure, rerun of only failed gate and its downstream gates
- `orchestrate-presets`: Built-in workflow presets — `rust`, `typescript`, `python`, `go`, `generic`; each specifies gate config, recommended skills, and agent role suggestions; preset files live in `ito-rs/crates/ito-templates/assets/presets/orchestrate/<stack>.yaml`
- `orchestrate-setup`: Setup wizard skill (`ito-orchestrate-setup`) — detects stack (scans for `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, `Makefile`), cross-references available skills, detects available agents (suggests, does not auto-wire), recommends a preset, generates `orchestrate.md` and the `ito-orchestrator-workflow` skill; triggered on first run (no `orchestrate.md` present) or explicitly via `ito orchestrate --setup`
- `orchestrate-workflow-skill`: The `ito-orchestrator-workflow` skill scaffold — generated per project by the setup wizard; the orchestrator loads it by convention (checks if skill exists, no configuration required); lives in the project's skill directory as a living markdown document evolved by the user
- `orchestrate-parallelism`: Parallelism model — `--max-parallel` flag accepting numeric values and named aliases (`serial`/`sync`/`synchronous` → 1; `parallel`/`fan-out`/`swarm`/`distributed` → cap); default mode `auto`, capped at `max_parallel: 4`; dependency graph respects `depends_on` in `.ito.yaml`

### Modified Capabilities

- `agent-instructions`: Add `orchestrate` as a new artifact type handled by `ito agent instruction`
- `change-repository`: Expose `orchestrate.depends_on` and `orchestrate.preferred_gates` fields from `.ito/changes/<id>/.ito.yaml`

## Impact

- **New crate module**: `ito-rs/crates/ito-core/src/orchestrate/` — `mod.rs`, `discovery.rs`, `gates.rs`, `state.rs`, `user_prompt.rs`, `types.rs`
- **CLI**: `ito-rs/crates/ito-cli/src/app/instructions.rs` — handle `orchestrate` artifact; `ito-rs/crates/ito-cli/src/cli/agent.rs` — add flags for orchestrate instruction generation
- **Templates**: new files under `ito-rs/crates/ito-templates/assets/`
  - `instructions/agent/orchestrate.md.j2`
  - `skills/ito-orchestrate/SKILL.md` (orchestrator entry-point skill)
  - `skills/ito-orchestrate-setup/SKILL.md` (setup wizard skill)
  - `skills/ito-orchestrator-workflow/SKILL.md` (per-project workflow skill scaffold — generated, then evolved)
  - `commands/ito-orchestrate.md`
  - `default/project/.ito/user-prompts/orchestrate.md`
  - `agents/opencode/ito-orchestrator.md`
  - `presets/orchestrate/rust.yaml`, `typescript.yaml`, `python.yaml`, `go.yaml`, `generic.yaml`
- **Schema**: `.ito/changes/<id>/.ito.yaml` gains `orchestrate:` block (`depends_on`, `preferred_gates`)
- **State directory**: `.ito/.state/orchestrate/runs/` (runtime only, not committed)
- No breaking changes to existing CLI surface or artifact types
<!-- ITO:END -->
