## Context

Ito's agent instruction system generates contextual prompts that guide LLMs through workflow phases (propose, spec, design, implement, archive). The system dispatches through `handle_agent_instruction()` in `instructions.rs`, with special-cased handlers for `bootstrap`, `project-setup`, `new-proposal`, and `apply`, while other artifact IDs fall through to a generic `resolve_instructions()` path that looks up artifacts in the schema.

**Current gap**: The bootstrap template (`bootstrap.md.j2`) and the `ito-review` skill both advertise `ito agent instruction review --change <id>`, but no handler exists. The command falls through to `resolve_instructions()`, which returns `ArtifactNotFound("review")` because "review" is not a schema-defined artifact. This change fixes that gap by adding a dedicated `review` handler focused on peer-reviewing proposals before implementation begins.

**Existing review mechanisms** are exclusively post-implementation: `requesting-code-review` works against git diffs, `ito validate` checks structural format only (scenario presence, header format), and the verification-before-completion skill is a pre-commit checklist. None evaluate proposal quality, spec completeness, design soundness, or task feasibility.

## Goals / Non-Goals

**Goals:**

- Add a `review` instruction type that produces structured guidance for peer-reviewing change proposals
- Integrate with the existing instruction dispatch system as a special-cased handler (like `apply`)
- Provide a multi-section review checklist covering proposal, specs, design, and tasks
- Include validation results and artifact inventory in the review context
- Fix the broken `ito agent instruction review` command that skills already reference
- Gate implementation start behind meaningful proposal-level review

**Non-Goals:**

- Post-implementation code review (already handled by `requesting-code-review` skill)
- Automated pass/fail judgments — the reviewer makes the call
- Generating a stored `review.md` artifact — review output goes to the conversation, not a file
- Adding `archive` instruction (similar gap, separate change)
- Modifying the schema system — review is a workflow step, not a generated artifact

## Decisions

### D1: Special-case handler, not a schema artifact

**Decision**: Add `review` as a special-cased `if artifact == "review"` block in `handle_agent_instruction()`, alongside `apply`, `bootstrap`, etc.

**Rationale**: Review is a workflow step that reads existing artifacts, not a step that generates a new file. The schema system's `resolve_instructions()` is designed for artifacts that produce output files (`proposal.md`, `specs/`, `design.md`, `tasks.md`). Review doesn't fit that model. A special-case handler gives full control over what context to assemble.

**Alternative considered**: Adding `review` as a schema artifact with `generates: review.md`. Rejected because it would imply review produces a persisted file, which conflicts with the non-goal of keeping reviews conversational.

### D2: Dedicated template at `agent/review.md.j2`

**Decision**: Create a new Jinja2 template following the same pattern as `apply.md.j2` — structured sections with conditional blocks based on which artifacts exist.

**Rationale**: The template needs to reference all change artifacts (proposal, specs, design, tasks) and include review checklists specific to each. This is fundamentally different from `artifact.md.j2` (which renders a single artifact's schema template) or `apply.md.j2` (which focuses on task progress).

### D3: Build a `PeerReviewContext` struct in core workflow

**Decision**: Add a `compute_review_context()` function in `ito-core::workflow` that assembles:
- Change metadata (name, module, schema)
- Artifact inventory (which artifacts exist, their paths)
- Structural validation results (run `validate_change()` programmatically)
- Task summary (total, by status, wave count — if tasks.md exists)
- List of affected main specs (from delta `operation` fields)

**Rationale**: The CLI handler should stay thin. Core logic lives in `ito-core::workflow` consistent with `compute_apply_instructions()`. The context struct is serializable for the template engine.

**Alternative considered**: Assembling context directly in the CLI handler. Rejected for consistency and testability.

### D4: Multi-section review checklist with severity-tagged output format

**Decision**: The template instructs the reviewer to evaluate each artifact section independently and produce findings tagged as `[blocking]`, `[suggestion]`, or `[note]`. A final verdict section asks for `approve`, `request-changes`, or `defer`.

**Rationale**: Structured output makes review findings actionable. Severity tags let the proposal author triage feedback. The three-verdict model mirrors standard code review practice (GitHub PR reviews use the same categories).

**Alternative considered**: Free-form review output. Rejected because unstructured feedback is harder to act on and track.

### D5: Conditional sections based on artifact presence

**Decision**: The template includes review sections only for artifacts that exist. If no `design.md` exists, the design review section is skipped (not an error — not all changes need design docs). If no `tasks.md` exists, task review is skipped.

**Rationale**: Changes progress through artifacts in order (proposal → specs → design → tasks). A review can happen at any point during that progression. Early review (proposal-only) is valuable and should be encouraged, not blocked by missing downstream artifacts.

### D6: Embed validation results, don't just reference the command

**Decision**: The `compute_review_context()` function runs `validate_change()` and `validate_tasks_file()` internally and includes the results (issues list, pass/fail) in the template context. The reviewer sees validation results without needing to run a separate command.

**Rationale**: Reduces friction. If the reviewer has to run `ito validate` separately, they might skip it. Embedding results ensures structural issues are always surfaced.

**Trade-off**: Slightly more compute per review instruction generation. Acceptable — validation is fast (sub-second).

## Risks / Trade-offs

**[Risk: Template bloat]** → The review template could become very long if all artifact sections are present. Mitigate by keeping checklists focused (5-8 items per section) and using conditional rendering to skip irrelevant sections.

**[Risk: Review fatigue]** → If every proposal requires a full peer review, it could slow velocity on small changes. Mitigate by making review optional in the workflow (not a hard gate in the schema) and noting in the template that small/obvious changes can have abbreviated review.

**[Risk: Stale validation results]** → Validation results are computed at instruction generation time. If the proposal is modified after generating the review instruction but before completing the review, results may be stale. Mitigate by noting the generation timestamp in the template output.

**[Trade-off: No persisted review artifact]** → Reviews live in conversation history, not in a file. This means review feedback may be lost between sessions. Accepted because: (a) persisting reviews adds artifact management complexity, (b) the audit log (009-02) will eventually capture review events, (c) reviews can be re-generated cheaply.

## Open Questions

- Should `ito agent instruction archive` be fixed in the same change? It has the same `ArtifactNotFound` problem. Leaning toward a separate change to keep scope tight.
- Should the review template suggest creating an issue or comment if run in a CI/PR context? Deferred to a future enhancement.
