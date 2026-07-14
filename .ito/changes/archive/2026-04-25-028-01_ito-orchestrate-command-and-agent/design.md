<!-- ITO:START -->
## Context

Ito currently has no first-party orchestration layer. Agents apply changes individually with no coordination, gate ordering, or run state. This design introduces the orchestrator: a stateful coordinator that reads project policy, builds an execution plan, dispatches worker agents per gate, tracks run state to disk, and handles remediation — without ever writing code itself.

The orchestrator is entirely agent-driven. It lives as a rendered instruction document (`ito agent instruction orchestrate`) plus a set of template assets (`skills/`, `commands/`, `agents/`). The CLI surface is minimal: one new artifact type, one new `.ito.yaml` schema block, and one new user-prompt file.

## Goals / Non-Goals

**Goals:**
- Coordinate multi-change apply runs with configurable parallelism
- Enforce a consistent gate pipeline per change (apply → format → lint → tests → style → code-review → security-review)
- Persist run state to disk so runs are resumable and inspectable
- Provide a setup wizard that generates a project-specific `ito-orchestrator-workflow` skill and `orchestrate.md`
- Ship five built-in presets covering common stacks

**Non-Goals (v1):**
- External hook execution (events are emitted to `events.jsonl` only; execution is v2)
- Auto-wiring agents (always advisory suggestions, never automatic)
- CI/CD integration (orchestrator runs inside an agent harness, not a pipeline runner)
- Distributed execution across machines

## Decisions

### Orchestrator is instruction-only, not a new CLI subcommand

**Decision:** `ito agent instruction orchestrate` renders the orchestrator document. There is no `ito orchestrate run` subcommand in v1.

**Rationale:** The orchestrator is an agent role, not a CLI process. CLI processes have no event loop, no ability to dispatch agents, and no way to handle remediation interactively. Keeping it instruction-only preserves the existing pattern and avoids building a process runner.

**Alternative:** `ito orchestrate run` as a CLI command that shells out to agent APIs — rejected because it would require harness-specific integrations and tight coupling.

### State layout: append-only events + per-change gate files

**Decision:** `.ito/.state/orchestrate/runs/<run-id>/events.jsonl` (append-only log) + `changes/<change-id>.json` (per-gate result snapshot).

**Rationale:** Append-only log gives a complete timeline for debugging. Per-change gate files give O(1) resume lookup without replaying the full log. The two sources are complementary and kept in sync by the orchestrator after each gate transition.

**run-id format:** `YYYYMMDD-HHMMSS-<short-uuid>` (sortable, human-readable, collision-resistant).

### Preset files are YAML, not embedded in templates

**Decision:** `ito-rs/crates/ito-templates/assets/presets/orchestrate/<stack>.yaml` — separate files, not hardcoded into template logic.

**Rationale:** Presets need to evolve independently of template rendering logic. Separate files allow `ito update` to refresh them without touching user-authored files. The rendering logic reads them at instruction-generation time.

**Preset schema:**
```yaml
name: rust
gate_config:
  format: { tool: "cargo fmt --check", skip: false }
  lint: { tool: "cargo clippy -- -D warnings", skip: false }
  tests: { tool: "cargo test", skip: false }
  style: { skill: "rust-style", skip: false }
  code-review: { agent_role: review-worker, skip: false }
  security-review: { agent_role: security-worker, skip: false }
recommended_skills:
  - rust-style
  - rust-code-reviewer
agent_roles:
  apply-worker: rust-engineer
  review-worker: rust-code-reviewer
  security-worker: rust-quality-checker
```

### ito-orchestrator-workflow skill loaded by convention

**Decision:** The orchestrator checks for `ito-orchestrator-workflow` in the project skill directory at instruction-render time. No configuration entry needed. Absent skill → no error, just renders without it.

**Rationale:** Convention over configuration. The skill name is unambiguous. Any project using the orchestrator will have it after running setup. Projects that don't use orchestration are unaffected.

### Worker agents are thin role wrappers, not auto-wired

**Decision:** Worker definitions (`ito-apply-worker`, `ito-review-worker`, `ito-security-worker`) are template assets describing the role contract. The orchestrator instruction names a suggested agent per role from the active preset. The user wires the actual agent.

**Rationale:** Agent availability is harness-specific. Auto-wiring would require harness detection at instruction-render time and would break silently in harnesses where the suggested agent doesn't exist. Advisory suggestions are always safe.

### orchestrate.md front matter + MUST/PREFER markdown sections

**Decision:** User prompt is `---` YAML front matter for machine-readable policy, followed by `## MUST` / `## PREFER` / `## Notes` for natural-language orchestrator guidance. Same pattern as other Ito user prompts.

**Front matter schema:**
```yaml
preset: rust                  # built-in preset name
max_parallel: 4               # or 'auto', 'serial', 'parallel'
failure_policy: remediate     # 'remediate' | 'stop' | 'continue'
gate_overrides:
  security-review: skip       # per-gate override
```

### per-change orchestrate block in .ito.yaml

**Decision:** Extend `.ito/changes/<id>/.ito.yaml` with an optional `orchestrate:` block:
```yaml
orchestrate:
  depends_on:
    - 028-01_ito-orchestrate-command-and-agent
  preferred_gates:
    - apply-complete
    - tests
    - code-review
```

**Rationale:** Per-change policy belongs alongside the change, not in a global config. The `depends_on` field drives the execution plan's dependency graph. `preferred_gates` overrides the default pipeline for a single change without touching `orchestrate.md`.

## Module Layout: ito-core/src/orchestrate/

```
ito-rs/crates/ito-core/src/orchestrate/
├── mod.rs            # pub re-exports, module doc
├── types.rs          # RunId, GateName, GateOutcome, RunStatus, OrchestrateConfig, ChangeOrchestrateMeta
├── user_prompt.rs    # parse orchestrate.md (front matter + sections)
├── preset.rs         # load and merge preset YAML files
├── discovery.rs      # find open changes, read orchestrate meta from .ito.yaml
├── plan.rs           # build RunPlan from changes + dependency graph + gate config
├── state.rs          # read/write run.json, plan.json, events.jsonl, changes/<id>.json
└── gates.rs          # gate pipeline definitions, remediation packet construction
```

## CLI Changes

**`ito-rs/crates/ito-cli/src/cli/agent.rs`**
- Add `Orchestrate` variant to `AgentInstructionArtifact` enum
- No new top-level subcommand

**`ito-rs/crates/ito-cli/src/app/instructions.rs`**
- Handle `AgentInstructionArtifact::Orchestrate`
- Check for `orchestrate.md`; emit setup guidance and exit(1) if absent
- Load `ito-orchestrator-workflow` skill if present
- Load active preset from front matter
- Render `orchestrate.md.j2`

## Template Assets

```
ito-rs/crates/ito-templates/assets/
├── instructions/agent/orchestrate.md.j2          # main rendered instruction
├── skills/
│   ├── ito-orchestrate/SKILL.md                  # orchestrator entry-point skill
│   ├── ito-orchestrate-setup/SKILL.md            # setup wizard skill
│   └── ito-orchestrator-workflow/SKILL.md        # per-project workflow skill scaffold
├── commands/ito-orchestrate.md                   # harness command entrypoint
├── default/project/.ito/user-prompts/orchestrate.md  # generated project stub
├── agents/opencode/ito-orchestrator.md           # OpenCode agent definition
└── presets/orchestrate/
    ├── rust.yaml
    ├── typescript.yaml
    ├── python.yaml
    ├── go.yaml
    └── generic.yaml
```

## Risks / Trade-offs

- **State staleness:** If the orchestrator crashes between writing `events.jsonl` and `changes/<id>.json`, the two may diverge. Mitigation: always write `events.jsonl` first; treat gate files as derived cache. On resume, reconcile gate files from the event log if they are absent.
- **Circular dependency detection:** Must happen at plan-build time, not at dispatch time. A topological sort (Kahn's algorithm) over the `depends_on` graph detects cycles early and reports them clearly.
- **Preset YAML schema drift:** Preset files in templates may fall out of sync with the parser in `preset.rs`. Mitigation: schema validation in `preset.rs` with clear error messages; integration tests for all five built-in presets.

## Open Questions

- Should `events.jsonl` be committed? Decision: no — it lives under `.ito/.state/` which is gitignored. Only `orchestrate.md` and the `ito-orchestrator-workflow` skill are committed.
- Should `ito validate` check for circular `depends_on` graphs? Decision: yes — add as a validator warning in `ito.delta-specs.v1` or a new `ito.orchestrate.v1` validator in a follow-up change.
<!-- ITO:END -->
