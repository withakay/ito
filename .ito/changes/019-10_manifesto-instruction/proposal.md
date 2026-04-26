# Add Ito Manifesto Instruction

## Summary

Add a generated Ito Manifesto: a strict, config-bound, state-aware prompt-only execution contract for agents that cannot run the Ito CLI or custom executables.

The manifesto augments existing `ito agent instruction ...` flows. When Ito is available, agents should still use live instructions. When Ito is unavailable, the generated manifesto gives an LLM enough deterministic protocol, config, state, and operation guidance to behave close to an Ito-guided agent.

## Problem

Ito currently guides LLM agents through deterministic instruction artifacts rendered by the CLI. This works well when the agent can call Ito, but fails or degrades in environments where:

- the agent cannot run custom executables;
- the agent cannot make tool calls;
- the planning environment is sandboxed and prompt-only;
- the agent should create or refine proposals but must not apply code;
- an external system can ingest text but not execute Ito;
- a user wants a portable representation of Ito's ways of working for a given config.

Without a generated manifesto, agents in those environments must rely on vague memory of Ito's workflow, which increases hallucination and drift.

## Goals

- Add a first-class `manifesto` instruction artifact.
- Generate a strict, state-machine-oriented ways-of-working document.
- Bind the manifesto to the effective merged Ito config.
- Support project-wide and change-scoped output.
- Support `light` and `full` variants.
- Support capability profiles such as `planning`, `proposal-only`, `review-only`, `apply`, `archive`, and `full`.
- Embed machine-readable config and state capsules.
- In full mode, embed relevant rendered Ito instructions verbatim or near-verbatim.
- Preserve existing Ito determinism: exact change IDs, exact paths, exact config-derived rules.

## Non-goals

- Do not replace live Ito CLI instruction rendering.
- Do not require prompt-only agents to invent missing state.
- Do not loosen worktree or coordination branch constraints.
- Do not make the manifesto a generic best-practices document.
- Do not expose secrets or local-only private paths by default.

## Proposed command shape

```bash
ito agent instruction manifesto
ito agent instruction manifesto --variant light
ito agent instruction manifesto --variant full
ito agent instruction manifesto --change <change-id>
ito agent instruction manifesto --change <change-id> --operation proposal
ito agent instruction manifesto --change <change-id> --operation apply
ito agent instruction manifesto --profile proposal-only
ito agent instruction manifesto --json
```

Possible future flags:

```bash
--include-instructions summary|verbatim|all
--include-artifacts none|summary|verbatim
--profile planning|proposal-only|review-only|apply|archive|full
```

## Manifesto model

The manifesto should include:

1. Contract and operating mode.
2. Global MUST/MUST NOT rules.
3. Source-of-truth order.
4. Capability profile.
5. State capsule.
6. State machine with allowed and forbidden moves.
7. Artifact model.
8. Worktree policy rendered from config.
9. Coordination branch policy rendered from config.
10. Config capsule.
11. Operation playbooks.
12. Memory behavior.
13. User/project guidance.
14. Rendered Ito instructions in full mode.
15. Fallback behavior when Ito is unavailable.

## State-machine strictness

This should not be advisory. The manifesto should define valid states and allowed transitions.

Canonical states:

- `no-change-selected`
- `proposal-drafting`
- `review-needed`
- `apply-ready`
- `applying`
- `reviewing-implementation`
- `archive-ready`
- `finished`

Each state should list allowed and forbidden operations. Local agents can later refine the inferred-current-state algorithm.

## Capability profiles

The generated manifesto should declare one active profile:

- `planning`: inspect and advise only.
- `proposal-only`: create or revise proposal/spec/design/task artifacts; do not edit product code.
- `review-only`: inspect and produce findings; do not mutate files.
- `apply`: implement scoped tasks only after worktree/state checks.
- `archive`: archive accepted changes only.
- `full`: full lifecycle through valid state transitions.

Prompt-only planning systems should generally use `planning` or `proposal-only`.

## Output variants

### Light

Compact protocol document with state machine, config capsule, state capsule, worktree/coordination/memory rules, and operation playbooks.

### Full

Everything in light plus embedded rendered operation instructions, ideally using existing instruction rendering so current templates remain the source of truth.

## Implementation notes

A first-pass template already exists in this branch at:

```text
ito-rs/crates/ito-templates/assets/instructions/agent/manifesto.md.j2
```

The remaining work is to wire the CLI context and tests.

Likely Rust changes:

- add `manifesto` handling in `ito-rs/crates/ito-cli/src/app/instructions.rs`;
- add CLI parsing for `--variant`, `--profile`, and possibly `--operation`;
- build a `ManifestoContext` serializable struct;
- load cascading config;
- compute worktree config with paths;
- compute coordination branch settings;
- compute memory config;
- load composed user guidance;
- optionally resolve `--change`;
- optionally render sub-instructions for full mode;
- emit Markdown or `AgentInstructionResponse` JSON.

## Risks

- Full mode can become large. Mitigate with variant/profile flags.
- Embedded instructions may conflict with global rules if context is stale. Make global hard rules take precedence.
- Config capsule could leak secrets. Redact aggressively by default.
- Change-scoped generation may need coordination sync to avoid stale state.

## Acceptance criteria

- `ito agent instruction manifesto` emits a project-wide strict Ito protocol document.
- `ito agent instruction manifesto --change <id>` resolves exact change state and includes a state capsule.
- `--variant light` emits compact protocol and playbooks.
- `--variant full` embeds relevant rendered instructions.
- Worktree-enabled output includes main/control checkout read-only guardrails.
- Coordination-enabled output explains shared coordination branch/worktree behavior.
- Capability profile restrictions render as MUST/MUST NOT rules.
- Memory-configured output includes provider-specific guidance or rendered instructions.
- Output redacts secrets.
- Help output lists `manifesto` as an artifact.
- Tests cover enabled worktrees, disabled worktrees, coordination enabled, memory configured, variants, profiles, and change-scoped generation.
