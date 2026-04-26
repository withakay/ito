<!-- ITO:START -->
## Context

This change adds a new `manifesto` agent-instruction artifact on top of Ito's existing instruction-rendering system. The branch already contains a draft template at `ito-rs/crates/ito-templates/assets/instructions/agent/manifesto.md.j2`, but the CLI and rendering pipeline still need a typed request surface, merged config and change-state context assembly, and rules for how manifesto output interacts with embedded instruction text.

The project now uses worktree-backed coordination storage, so change-scoped rendering must resolve authoritative change state from the coordination worktree rather than from branch-local copies. The manifesto also needs to be useful in prompt-only environments without weakening the existing live-Ito workflow.

## Goals / Non-Goals

**Goals:**

- Introduce `ito agent instruction manifesto` as a first-class artifact in the existing agent-instruction pipeline.
- Reuse the existing instruction-rendering system and effective merged Ito configuration instead of inventing a parallel configuration path.
- Support project-wide and change-scoped rendering, `light` and `full` variants, and one active capability profile per render.
- Keep manifesto output deterministic and safe by default through redaction and explicit source-of-truth precedence.

**Non-Goals:**

- Replacing live Ito instruction rendering when the CLI is available.
- Changing worktree, coordination, or memory behavior themselves beyond how they are represented in the manifesto.
- Implementing generic best-practices prose that is detached from Ito config and state.

## Decisions

- Decision: Treat this as an extension of the existing `agent-instructions` capability.
  Rationale: The manifesto is another generated instruction artifact, not a separate product surface with its own lifecycle.
  Alternatives considered: A new standalone capability was rejected because it would split closely related instruction behavior across multiple specs.

- Decision: Build a typed `ManifestoContext` from the same merged config and change-resolution path used by other instruction artifacts.
  Rationale: This keeps manifesto output aligned with the rest of Ito and avoids a second source of truth for worktree, coordination, memory, and user-guidance data.
  Alternatives considered: Passing ad hoc template variables directly from the CLI was rejected because it would make redaction and state handling fragile.

- Decision: Make `light` and `full` variants explicit request modes rather than a single adaptive render.
  Rationale: The output size and fidelity trade-off is central to the feature, and explicit variants make behavior predictable for humans and tests.
  Alternatives considered: Auto-sizing the output based on environment hints was rejected because it would be hard to test and easy to misinterpret.

- Decision: In `full` mode, embed relevant existing rendered instruction artifacts, but keep manifesto-level MUST and MUST NOT rules authoritative.
  Rationale: Existing templates stay canonical while the manifesto still acts as the hard execution contract for prompt-only environments.
  Alternatives considered: Duplicating instruction content inside the manifesto template was rejected because it would drift from the live instruction artifacts.

- Decision: Redact secrets and local-only paths by default in config and state capsules.
  Rationale: The manifesto is meant to be portable and may be shared with systems that should not receive machine-local details.
  Alternatives considered: Raw config dumps were rejected because they risk leaking secrets and environment-specific paths.

## Risks / Trade-offs

- Full-mode output can become too large -> Keep `light` as the default compact contract and scope embedded instructions to the requested operation when possible.
- Embedded instruction text can drift from manifesto-level rules -> Treat manifesto rules as the authoritative layer and compose live instruction renders instead of hand-copying content.
- Change-scoped output can become stale if coordination state is not resolved correctly -> Always resolve change data from the coordination-backed repository path used by the current project wiring.
- Redaction can hide useful debugging context -> Prefer structured capsules that preserve intent while removing secrets and local-only details.

## Migration Plan

1. Extend CLI argument parsing and instruction dispatch to accept the manifesto artifact and its variant, profile, and optional operation inputs.
2. Build and test `ManifestoContext` assembly from merged config, worktree and coordination settings, user guidance, memory config, and optional change state.
3. Wire the existing `manifesto.md.j2` template into the renderer for `light` and `full` modes.
4. Add rendering and regression tests for profiles, redaction, change-scoped state, and embedded-instruction precedence.

Rollback is straightforward because the feature is additive: remove the CLI surface, the render path, and the new template wiring if the contract proves too unstable.

## Open Questions

- Should `--operation` remain optional metadata or become required for some `full` renders to limit output size?
- Which config fields should be summarized versus shown verbatim in redacted capsules?
- Should help and JSON output expose supported profiles and variants directly, or only surface the artifact name at first?
<!-- ITO:END -->
