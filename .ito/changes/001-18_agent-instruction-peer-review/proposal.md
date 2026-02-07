# Change: Add agent instruction for peer reviewing change proposals

## Why

`ito agent instruction review --change <id>` is already referenced by the bootstrap template, the `ito-review` skill, and the `ito-workflow` skill, but no handler exists -- the command falls through to the generic artifact resolver and returns `ArtifactNotFound("review")`. Beyond fixing this broken command, there is no structured mechanism for an agent to peer-review a change proposal (proposal/specs/design/tasks) before implementation begins. Existing validation (`ito validate`) only checks structural format (scenarios exist, headers correct, delta operations valid) but does not assess whether the proposal is sound, specs are complete, or tasks are well-scoped. A dedicated peer-review instruction fills the gap between "structurally valid" and "ready to implement".

## What Changes

- Add a special-cased `review` handler in the instruction dispatcher (`instructions.rs`) alongside the existing `apply`, `bootstrap`, `project-setup`, and `new-proposal` handlers
- Create a new Jinja2 template (`agent/review.md.j2`) that provides structured peer-review guidance with checklists for proposal quality, spec completeness, design soundness, and task coverage
- Add a `compute_review_context()` function in `ito-core` that gathers change artifacts, runs structural validation, identifies affected existing specs, and packages the context for the template
- The template outputs a structured review protocol: what to read, what to check, how to report findings, and a clear verdict (approve / request-changes / needs-discussion)
- Wire the existing `ito-review` skill to work with the now-functional instruction command

## Capabilities

### New Capabilities

- `peer-review-instruction`: The agent instruction for peer-reviewing change proposals before implementation. Covers the review template, context gathering, checklist structure, and output format.

### Modified Capabilities

- `agent-instructions`: Extend the agent instruction dispatcher to handle the `review` artifact type as a special-cased instruction (same pattern as `apply`, `bootstrap`).
- `stable-instruction-generation`: The review instruction must honor the same configurable testing policy and user guidance injection patterns.

## Impact

- **Code**: `ito-cli/src/app/instructions.rs` (new dispatch branch), `ito-core/src/workflow/mod.rs` (new context builder), `ito-templates/assets/instructions/agent/review.md.j2` (new template)
- **Skills**: `ito-templates/assets/skills/ito-review/SKILL.md` becomes functional (currently broken)
- **Workflow**: Adds a review gate between proposal completion and implementation start
- **Dependencies**: No new crate dependencies; uses existing minijinja template engine and validation infrastructure
