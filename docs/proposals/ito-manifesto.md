# Proposal: Ito Manifesto / Prompt-Only Ito

## Status

Exploratory design proposal.

This proposal assumes the manifesto is **strict**, **state-machine oriented**, and **generated from effective Ito configuration**. It is intended to augment Ito, not replace it.

## Problem

Ito currently guides agents by emitting deterministic, config-aware instruction artifacts from the CLI. Commands such as `ito agent instruction proposal --change <id>`, `ito agent instruction apply --change <id>`, `ito agent instruction worktrees`, `ito agent instruction review --change <id>`, and `ito agent instruction finish --change <id>` resolve project configuration and state before producing targeted instructions.

That model works when an agent can run Ito. It works less well when:

- the agent cannot make tool calls;
- the agent cannot run arbitrary local executables;
- the planning system is intentionally sandboxed;
- the agent should be allowed to create or revise proposals but not apply code;
- a third-party system can only accept prompt text;
- Ito should be represented as a portable ways-of-working contract.

The goal is to generate a **compiled Ito Manifesto**: a self-contained, config-bound, state-aware protocol document that can be handed to a capable LLM and cause it to behave closely to an Ito-guided agent even without live CLI access.

## Core thesis

Ito is not primarily a command-line wrapper. Ito is a protocol compiler for disciplined AI-assisted software development.

- The CLI resolves deterministic project facts.
- Configuration defines constraints and execution policy.
- Templates encode operation-specific prompts.
- Change IDs anchor state and avoid hallucination.
- Worktrees and coordination branches isolate concurrent work.
- User prompts inject local rules without forking the protocol.
- Instruction artifacts are compiled execution plans.

The manifesto should make the implicit protocol explicit.

## Desired operating mode

The manifesto should be **strict**, not merely advisory.

It should define:

1. a state model;
2. allowed moves from each state;
3. forbidden moves;
4. artifact requirements;
5. config-derived constraints;
6. fallback behavior when Ito cannot be run;
7. exact embedded instruction text where useful.

The manifesto is expected to be consumed by LLMs, but it should be structured consistently enough that programmatic consumption remains possible later.

## Relationship to Ito

The manifesto **augments** Ito.

When Ito is available, agents should still prefer live Ito calls because live calls can resolve current state, run validation, sync coordination, and render exact operation prompts.

When Ito is unavailable, the manifesto becomes the fallback protocol.

This gives Ito three operating layers:

1. **Live mode** — agent can call Ito and should use `ito agent instruction ...`.
2. **Manifesto mode** — agent cannot call Ito, but has a generated manifesto.
3. **Partial mode** — agent has a manifesto and some files, but not full repository/tool access.

The manifesto must explicitly say which mode applies and what obligations follow.

## Output variants

Generate at least two output variants:

### Light manifesto

Purpose: portable behavioral contract.

Contains:

- protocol identity;
- state machine;
- artifact model;
- config-derived hard rules;
- concise playbooks;
- redacted config capsule;
- optional change state capsule.

### Full manifesto

Purpose: maximal fidelity when pasted into an LLM with enough context budget.

Contains everything in the light manifesto plus:

- verbatim or near-verbatim rendered operation instructions;
- worktree instructions;
- schema guidance;
- memory instructions;
- archive/finish guidance;
- user prompt overlays;
- selected specs/change artifact summaries or full text when requested.

Suggested command shape:

```bash
ito agent instruction manifesto
ito agent instruction manifesto --variant light
ito agent instruction manifesto --variant full
ito agent instruction manifesto --change <change-id>
ito agent instruction manifesto --change <change-id> --operation proposal
ito agent instruction manifesto --change <change-id> --operation apply
ito agent instruction manifesto --json
```

Possible future flags:

```bash
--include-instructions summary|verbatim
--include-artifacts summary|verbatim|none
--capability planning|proposal|review|apply|archive|full
```

## Capability profiles

The manifesto should support sandboxed environments where agents are intentionally limited.

Recommended profiles:

### `planning`

Agent may read context and produce recommendations, but must not create or modify Ito artifacts unless explicitly asked.

### `proposal-only`

Agent may create or revise proposal/spec/design/task artifacts, but must not edit product code or apply implementation tasks.

### `review-only`

Agent may inspect artifacts and code, produce review findings, and suggest changes, but must not mutate files.

### `apply`

Agent may implement tasks, subject to worktree, validation, and scope constraints.

### `archive`

Agent may merge accepted deltas into durable specs and follow archive integration policy.

### `full`

Agent may perform the full Ito lifecycle, subject to state machine and config.

The generated manifesto should state the active capability profile. If omitted, default to `full` only when the environment is intended to permit mutation. For prompt-only planning systems, default to `proposal-only` or `planning`.

## State machine

The manifesto should encode Ito as a strict state machine.

### State dimensions

State should be represented as a capsule, not prose only.

```json
{
  "mode": "manifesto",
  "capability_profile": "proposal-only",
  "project_path": ".ito",
  "change_id": null,
  "schema": null,
  "operation": null,
  "artifacts": {
    "proposal": "unknown",
    "specs": "unknown",
    "design": "unknown",
    "tasks": "unknown"
  },
  "worktree": {
    "enabled": true,
    "current_checkout_role": "unknown",
    "required_before_writes": true
  },
  "coordination_branch": {
    "enabled": false,
    "storage": null,
    "synced_at_generation": null
  },
  "validation": {
    "last_known_status": "unknown"
  }
}
```

### Canonical states

#### `no-change-selected`

No change ID is known.

Allowed moves:

- inspect project context;
- propose a new change name/slug;
- ask the user to select a change;
- create a proposal draft only if the active profile permits artifact creation and the target path is known.

Forbidden moves:

- edit product code;
- claim implementation progress;
- archive;
- finish a change.

#### `proposal-drafting`

A change exists or is being created, and proposal/spec/design/task artifacts are being drafted or revised.

Allowed moves:

- write or revise proposal;
- write or revise spec deltas;
- write or revise design notes;
- write or revise tasks;
- request review;
- run validation when available.

Forbidden moves:

- implement product code unless the user explicitly escalates capability and the state transitions to `apply-ready`;
- archive;
- mark tasks complete without implementation evidence.

#### `review-needed`

Artifacts exist but have not been approved or are known to need review.

Allowed moves:

- perform artifact review;
- identify inconsistencies;
- update proposal/spec/design/tasks if profile permits;
- request user decision.

Forbidden moves:

- begin implementation unless review is waived explicitly;
- archive.

#### `apply-ready`

The change has sufficient proposal/spec/task context and the active profile permits implementation.

Allowed moves:

- ensure or enter the correct worktree;
- re-read artifacts;
- apply tasks in order;
- update task status;
- run validation/tests;
- record deviations.

Forbidden moves:

- write from main/control checkout when worktrees are enabled;
- implement unrelated work;
- skip validation silently;
- archive unreviewed or failing work.

#### `applying`

Implementation is in progress.

Allowed moves:

- continue scoped implementation;
- update tasks honestly;
- revise artifacts when implementation reveals necessary changes;
- run tests;
- request review.

Forbidden moves:

- expand scope silently;
- claim completion without evidence;
- archive before validation/review requirements are satisfied.

#### `reviewing-implementation`

Implementation exists and should be checked against artifacts.

Allowed moves:

- compare implementation to proposal/specs/tasks;
- produce findings;
- request corrections;
- mark ready for archive only with evidence.

Forbidden moves:

- merge/archive unresolved findings;
- ignore spec drift.

#### `archive-ready`

The change is accepted and ready to merge into durable specs.

Allowed moves:

- archive according to config;
- merge deltas into durable specs;
- preserve historical change artifacts;
- follow configured integration mode.

Forbidden moves:

- archive rejected, incomplete, or unvalidated changes;
- drop deltas without recording rationale.

#### `finished`

The change is integrated, abandoned, or intentionally stopped.

Allowed moves:

- cleanup worktrees/branches according to config;
- capture memory if configured;
- refresh archive/spec state;
- report final status.

Forbidden moves:

- continue editing as if the change were active without reopening or creating a follow-up.

## Allowed move table

The manifesto should include a machine-readable table like this:

| State | Allowed operations | Forbidden operations |
| --- | --- | --- |
| `no-change-selected` | inspect, select-change, propose-change | apply, archive, finish |
| `proposal-drafting` | proposal, specs, design, tasks, validate, review | apply unless escalated, archive |
| `review-needed` | review, revise-artifacts | apply unless review waived, archive |
| `apply-ready` | worktree-ensure, apply, validate | main-write, unrelated-edits, archive |
| `applying` | implement, task-update, validate, revise-artifacts | scope-expansion, unsupported-complete |
| `reviewing-implementation` | review, fix, validate | archive-with-findings |
| `archive-ready` | archive, reconcile | implementation-expansion |
| `finished` | cleanup, memory-capture, report | further-edits-without-reopen |

## Global hard rules

The manifesto must state these as MUST/MUST NOT rules:

1. MUST prefer deterministic project facts over model guesses.
2. MUST use the exact supplied change ID when present.
3. MUST treat config-derived worktree rules as binding.
4. MUST NOT write product code in `planning`, `proposal-only`, or `review-only` profiles.
5. MUST NOT write from the main/control checkout when worktrees are enabled.
6. MUST NOT reuse one worktree for two changes.
7. MUST NOT claim validation, tests, archive, sync, or review succeeded unless actually observed.
8. MUST record material scope changes back into proposal/spec/design/tasks.
9. MUST preserve user-authored guidance unless explicitly superseded by the user.
10. MUST disclose uncertainty when operating without Ito CLI access.

## Source-of-truth hierarchy

The manifesto should define the trust order:

1. Latest explicit user instruction.
2. Repository state and files visible to the agent.
3. Manifesto state capsule generated by Ito.
4. Manifesto config capsule generated by Ito.
5. Change artifacts under `.ito/changes/<change-id>/`.
6. Durable specs under `.ito/specs/`.
7. Rendered Ito instruction text embedded in the manifesto.
8. User guidance embedded in the manifesto.
9. Prior model memory or assumptions.

If sources conflict, the agent must stop and surface the conflict unless the user has clearly resolved it.

## Artifact model

The manifesto should describe canonical Ito artifacts:

```text
.ito/
  project.md
  specs/
    <capability>/
      spec.md
      design.md
  changes/
    <change-id>/
      .ito.yaml
      proposal.md
      design.md
      tasks.md
      specs/
        <capability>/
          spec.md
  modules/
  planning/
```

Rules:

- Proposals explain why a change exists, what it does, what it does not do, and how success will be judged.
- Specs describe durable externally observable behavior and constraints.
- Spec deltas describe how durable specs should change once the proposal is accepted.
- Design docs explain architecture, trade-offs, migration, compatibility, and operational impact.
- Tasks are ordered, checkable implementation work items.
- Archive merges accepted deltas into durable specs and preserves history.

## Worktree policy rendering

When `worktrees.enabled = true`, the manifesto should render a hard section like:

> Worktrees are enabled. Treat the main/control checkout as read-only for proposal artifacts, code edits, generated files, commits, and implementation work. Before any write operation, create or move into the dedicated worktree for the current change. Use the full change ID as the branch and primary worktree directory name unless config explicitly says otherwise. Do not reuse one worktree for two changes.

It should include config-specific values:

- strategy;
- default branch;
- layout base dir;
- layout dir name;
- init include/copy rules;
- init setup commands;
- apply integration mode.

When `worktrees.enabled = false`, the manifesto should say worktrees are not required, but scope isolation still is.

## Coordination branch policy rendering

When `changes.coordination_branch.enabled = true`, the manifesto should render a hard section like:

> Coordination branch mode is enabled. Ito artifacts are coordinated through the configured coordination branch/storage. Multiple change worktrees may operate concurrently, but proposal/change/spec/task artifacts must coordinate through that shared substrate. Sync before reading or acting on change state whenever Ito is available. Do not confuse the coordination worktree with the implementation worktree.

It should include:

- branch name;
- storage mode;
- known worktree path if configured;
- sync obligations;
- artifact ownership expectations.

When disabled, say coordination branch mode is off and artifacts live according to the normal project layout.

## Config capsule

The manifesto should embed a redacted machine-readable capsule.

Example:

```json
{
  "projectPath": ".ito",
  "worktrees": {
    "enabled": true,
    "strategy": "bare_control_siblings",
    "layout": {
      "base_dir": null,
      "dir_name": "ito-worktrees"
    },
    "default_branch": "main",
    "init": {
      "include": [".env", ".envrc"],
      "setup": ["make init"]
    },
    "apply": {
      "enabled": true,
      "integration_mode": "commit_pr"
    }
  },
  "changes": {
    "coordination_branch": {
      "enabled": true,
      "name": "ito/coordination",
      "storage": "worktree",
      "worktree_path": "<redacted-or-relative>"
    }
  },
  "defaults": {
    "testing": {
      "coverage": { "target_percent": 80 },
      "tdd": { "workflow": "red-green-refactor" }
    }
  },
  "memory": {
    "capture": { "configured": true, "kind": "command" },
    "search": { "configured": true, "kind": "command" },
    "query": { "configured": false }
  },
  "backend": {
    "enabled": false,
    "url": "<redacted-or-local>",
    "token": "<redacted>"
  }
}
```

Secrets, tokens, and machine-private absolute paths should be redacted unless explicitly requested and safe.

## Change state capsule

When generated with `--change`, include exact state:

```json
{
  "change_id": "016-18_example-change",
  "change_dir": ".ito/changes/016-18_example-change",
  "schema": "spec-driven",
  "module": {
    "id": "016",
    "name": "agent-instructions"
  },
  "available_artifacts": ["proposal", "specs", "tasks"],
  "missing_artifacts": ["design"],
  "task_progress": {
    "total": 8,
    "complete": 3,
    "in_progress": 1,
    "pending": 4
  },
  "known_worktree_path": null,
  "generated_at": "2026-04-26T00:00:00Z"
}
```

This is one of the most important parts of the feature. It preserves Ito’s determinism in prompt-only environments.

## Embedded instructions

Because the preferred answer is “verbatim when possible,” the full manifesto should include rendered instruction sections.

Recommended structure:

```markdown
## Rendered Ito Instructions

### `proposal`

<verbatim rendered proposal instruction, if change-scoped>

### `specs`

<verbatim rendered specs instruction, if applicable>

### `tasks`

<verbatim rendered tasks instruction, if applicable>

### `apply`

<verbatim rendered apply instruction, if capability profile allows apply>

### `review`

<verbatim rendered review instruction>

### `archive`

<verbatim rendered archive instruction>

### `finish`

<verbatim rendered finish instruction>
```

Light manifesto can include concise operation playbooks instead.

## User guidance and custom prompts

The manifesto should include composed user guidance from the same sources already used by Ito instruction rendering.

It should separate:

- Ito protocol rules;
- config-derived rules;
- user/project guidance;
- operation-specific rendered instructions.

User guidance should be preserved verbatim where possible. If the manifesto compiler transforms it, the output should say so.

## Memory behavior

When memory is configured:

- include exact rendered memory capture/search/query instructions or invocations;
- require agents to search/query before major design or implementation decisions when relevant;
- require capture during apply/finish for durable decisions, gotchas, reusable patterns, and follow-ups.

When memory is not configured:

- state that no memory provider is configured;
- prohibit the agent from pretending it saved or searched memory;
- recommend writing durable knowledge to Ito artifacts instead.

## Failure behavior

The manifesto should define how to proceed when Ito is unavailable.

Rules:

- If exact state is missing, do not invent it.
- If a change ID is unknown, remain in `no-change-selected`.
- If worktree policy is enabled but current checkout role is unknown, avoid writes.
- If validation cannot be run, report it as not run.
- If instructions mention a CLI command that cannot be executed, follow the surrounding rule text and mark the CLI-dependent outcome as unverified.
- If the capability profile forbids mutation, provide patches or proposed content instead of editing files.

## Implementation plan

### Phase 1: Proposal and template skeleton

Add an embedded template:

```text
ito-rs/crates/ito-templates/templates/instructions/agent/manifesto.md.j2
```

Add template tests covering:

- state-machine section renders;
- worktrees enabled guardrails;
- worktrees disabled guidance;
- coordination branch enabled guidance;
- capability profile restrictions;
- config capsule rendering.

### Phase 2: CLI artifact

Add `manifesto` to `ito agent instruction`.

The handler should:

- load cascading project config;
- compute worktree config with resolved paths;
- compute coordination branch settings;
- compute archive config;
- compute memory config;
- load composed user guidance;
- resolve optional `--change`;
- resolve optional `--operation`;
- resolve optional `--variant`;
- render the template;
- emit Markdown or `AgentInstructionResponse` JSON.

### Phase 3: Full variant with embedded instruction rendering

For `--variant full`, render and embed relevant existing instructions.

For project-wide output:

- include general instructions such as worktrees, schemas, archive, finish, memory setup guidance, and operation playbooks.

For change-scoped output:

- include exact rendered proposal/specs/tasks/apply/review/archive/finish sections when available and relevant to capability profile.

### Phase 4: Capability profiles

Add `--capability` or `--profile`.

Examples:

```bash
ito agent instruction manifesto --profile proposal-only
ito agent instruction manifesto --profile review-only --change <id>
ito agent instruction manifesto --profile apply --change <id>
```

Profiles should be rendered as hard rules, not suggestions.

### Phase 5: Skill wrapper

Add an `ito-manifesto` skill that prefers:

```bash
ito agent instruction manifesto --variant full
```

or, for a change:

```bash
ito agent instruction manifesto --variant full --change <change-id>
```

When the CLI is not available, the skill should ask the user for the generated manifesto or for the minimum config/artifact bundle needed to reconstruct one.

## Open questions

1. Should `--variant full` embed all rendered instructions by default, or only those relevant to the selected profile?
2. Should profiles be purely prompt-level, or also reflected in JSON output for harnesses?
3. Should the default profile be `full`, `planning`, or derived from environment/tool capability?
4. Should `manifesto --change <id>` sync coordination state before rendering, like apply/proposal/review?
5. Should absolute worktree paths be included, redacted, or made configurable?
6. Should repo-level files such as `AGENTS.md` be embedded, summarized, or referenced?
7. Should there be a max-token budget option?
8. Should `manifesto` be a CLI artifact only, or also a first-class top-level command?

## Recommended answers

- Default to `light` unless `--variant full` is requested.
- In full mode, embed only instructions relevant to the selected profile by default.
- Allow `--include-instructions all` for complete embedding.
- Add `--profile`; default to `full` for local CLI use and `planning` or `proposal-only` for skill-driven sandboxed contexts.
- Sync coordination state before change-scoped manifesto rendering when coordination is enabled.
- Redact secrets and private absolute paths by default.
- Keep the primary output Markdown, but maintain stable headings and JSON capsules.

## Acceptance criteria

- `ito agent instruction manifesto` emits a project-wide strict Ito protocol document.
- `ito agent instruction manifesto --change <id>` resolves the exact change and includes a state capsule.
- `--variant light` emits compact protocol, state machine, config capsule, and playbooks.
- `--variant full` embeds relevant rendered instructions verbatim or near-verbatim.
- Worktree-enabled output includes main/control checkout read-only guardrails.
- Coordination-enabled output explains shared coordination branch/worktree behavior.
- Capability profile restrictions are rendered as MUST/MUST NOT rules.
- Memory-configured output includes exact provider-specific instructions or rendered invocations.
- Output redacts secrets.
- Help output lists `manifesto` as an artifact.
- Tests cover enabled worktrees, disabled worktrees, coordination enabled, memory configured, full/light variants, and change-scoped state generation.

## Draft manifesto preamble

> You are operating under the Ito protocol. Ito is a change-driven, spec/context-driven workflow for AI-assisted software development. Your job is to advance a named change through explicit artifacts, not merely to edit files. Prefer deterministic project facts over guesses. Respect the configured worktree and coordination branch policy. Follow the state machine and capability profile in this manifesto. When the Ito CLI is available, use it to resolve exact state. When it is not available, follow this manifesto, disclose uncertainty, and avoid inventing project facts.
