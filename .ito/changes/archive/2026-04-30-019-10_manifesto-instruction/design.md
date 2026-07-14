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

- Decision: Default to `variant=light` and `profile=full` when the user omits both flags.
  Rationale: The light variant is the safest portable default for output size, while the full profile best represents the unrestricted lifecycle contract before state narrowing is applied.
  Alternatives considered: Defaulting to `planning` was rejected because it would understate the lifecycle contract for ordinary CLI users, and defaulting to `variant=full` was rejected because it would make flagless output unnecessarily large.

- Decision: Help and machine-readable instruction responses expose supported variants and profiles directly.
  Rationale: The manifesto introduces new request dimensions, so discoverability should cover not just the artifact name but also its legal selectors and the active resolved values.
  Alternatives considered: Surfacing only the artifact name was rejected because downstream users and tools would still need to guess the selector contract.

- Decision: Treat `variant` and `profile` as orthogonal dimensions even though both expose a `full` value.
  Rationale: `variant=full` controls output detail, while `profile=full` controls lifecycle permissions. The renderer and help output must preserve both names but describe them as separate axes.
  Alternatives considered: Renaming one axis was rejected for this change because the existing proposal and template vocabulary already use `full`; explicit disambiguation is sufficient and less disruptive.

- Decision: In `full` mode, embed relevant existing rendered instruction artifacts, but keep manifesto-level MUST and MUST NOT rules authoritative.
  Rationale: Existing templates stay canonical while the manifesto still acts as the hard execution contract for prompt-only environments.
  Alternatives considered: Duplicating instruction content inside the manifesto template was rejected because it would drift from the live instruction artifacts.

- Decision: When `variant=full`, `--operation` is optional. If provided, embed only the requested operation's rendered instruction. If omitted, embed the allowed subset of this fixed ordered artifact list: `proposal`, `specs`, `design`, `tasks`, `apply`, `review`, `archive`, `finish`.
  Rationale: This keeps output deterministic, avoids underdefined "relevant" behavior, and still supports both scoped and broad full renders.
  Alternatives considered: Requiring `--operation` for every full render was rejected because the proposal explicitly supports project-wide full renders.

- Decision: `--operation` is only valid with `variant=full`.
  Rationale: `light` is the compact contract form and should not accept an operation selector that implies embedded operation-specific detail.
  Alternatives considered: Silently ignoring `--operation` for `light` was rejected because it would hide invalid requests.

- Decision: State and scope restrictions always narrow profile permissions; they never expand them.
  Rationale: A requested profile expresses the maximum lifecycle permission, while the resolved state and presence or absence of a change determine what is currently legal.
  Alternatives considered: Letting profile override state was rejected because it would undermine the manifesto's strictness.

- Decision: Embedded instruction selection uses only artifact-mapped operations.
  Rationale: Generic verbs such as `inspect`, `report`, `implement`, and `fix` are useful for the state machine but do not correspond to standalone `ito agent instruction <artifact>` bodies.
  Alternatives considered: Treating every verb as embeddable was rejected because it would invent instruction artifacts that do not exist.

- Decision: State resolution uses explicit repository facts first and only emits review/approval-sensitive states when authoritative signals exist.
  Rationale: The repository can reliably expose artifact presence, task progress, validation status, and archived state, but it may not always expose a separate approval marker.
  Alternatives considered: Always inferring `review-needed` or `archive-ready` heuristically was rejected because it would overstate certainty.

- Decision: Validation-sensitive state resolution uses the validation result observed during the current manifesto render.
  Rationale: Render-time validation avoids stale cached status and keeps `reviewing-implementation` versus `archive-ready` decisions tied to the same invocation that produced the manifesto.
  Alternatives considered: Reusing a historical validation result was rejected because it could be stale and would make state resolution nondeterministic.

- Decision: Approval-sensitive state resolution consumes an optional normalized `review_status` field in manifesto context with values `unknown`, `pending-approval`, `changes-requested`, or `approved`.
  Rationale: This gives the renderer a stable integration contract across local-only, host-provided, and backend-backed environments without requiring heuristics.
  Alternatives considered: Reading tool-specific review records directly inside the renderer was rejected because it would couple manifesto generation to one storage backend.

- Decision: Redact secrets and local-only paths by default in config and state capsules.
  Rationale: The manifesto is meant to be portable and may be shared with systems that should not receive machine-local details.
  Alternatives considered: Raw config dumps were rejected because they risk leaking secrets and environment-specific paths.

- Decision: Render project-scoped paths relative to the project root and replace all non-project absolute paths with placeholders.
  Rationale: Portable manifesto output should never require or reveal a machine-specific absolute path to remain useful.
  Alternatives considered: Allowing project-scoped absolute paths was rejected because it still leaks machine-local path structure.

## Risks / Trade-offs

- Full-mode output can become too large -> Keep `light` as the default compact contract and scope embedded instructions to the requested operation when possible.
- Embedded instruction text can drift from manifesto-level rules -> Treat manifesto rules as the authoritative layer and compose live instruction renders instead of hand-copying content.
- Change-scoped output can become stale if coordination state is not resolved correctly -> Always resolve change data from the coordination-backed repository path used by the current project wiring.
- Redaction can hide useful debugging context -> Prefer structured capsules that preserve intent while removing secrets and local-only details.
- The dual use of `full` can confuse users -> Keep CLI help and rendered output explicit that variant controls detail and profile controls permissions.

## Migration Plan

1. Extend CLI argument parsing and instruction dispatch to accept the manifesto artifact and its variant, profile, and optional operation inputs.
2. Build and test `ManifestoContext` assembly from merged config, worktree and coordination settings, user guidance, memory config, and optional change state.
3. Wire the existing `manifesto.md.j2` template into the renderer for `light` and `full` modes.
4. Add rendering and regression tests for profiles, redaction, change-scoped state, incompatible request failures, and embedded-instruction precedence.

Rollback is straightforward because the feature is additive: remove the CLI surface, the render path, and the new template wiring if the contract proves too unstable.

## Open Questions

- None.
<!-- ITO:END -->
