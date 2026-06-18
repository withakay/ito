---
name: ito-lite-review
description: Review Ito Lite markdown proposal packages before implementation. Use to check proposal.md, spec deltas, design.md, and tasks.md for consistency, risk, testability, and manual validation without the ito CLI.
compatibility: No external dependencies; markdown and file reading only.
---

# Ito Lite Review

Use this skill to review `.ito-lite/changes/<change-id>/` before implementation.

## Inputs

Read these files when present:

- `.ito-lite/project.md`
- `.ito-lite/changes/<change-id>/proposal.md`
- `.ito-lite/changes/<change-id>/specs/**/*.md`
- `.ito-lite/changes/<change-id>/design.md`
- `.ito-lite/changes/<change-id>/tasks.md`
- Relevant current specs under `.ito-lite/specs/`
- Other active changes that touch the same capabilities

## Review Priorities

Findings first. Focus on issues that can cause wrong implementation, bad requirements, missed edge cases, or unsafe archive merges.

## Structural Checks

- Change directory exists.
- Required artifacts exist: `proposal.md`, `tasks.md`, and at least one spec delta.
- `design.md` exists when the proposal marks `Design Needed: yes`.
- Spec delta directories match proposal capabilities.
- No unrelated files are required to understand the proposal.

## Proposal Checks

- Problem framing is specific and evidence-based.
- Scope is bounded and exclusions are explicit.
- Claimed impact aligns with listed changes.
- Risks and trade-offs are concrete.
- Success criteria are measurable or falsifiable.
- Public contracts are identified when affected.
- The selected schema fits the change shape.

## Spec Checks

- Only allowed delta headers are used.
- Requirement statements use `SHALL` or `MUST` consistently.
- Every requirement has at least one `#### Scenario:`.
- Scenarios have clear `WHEN` and `THEN` behavior.
- Edge cases and failure behavior are covered, not implied.
- `MODIFIED` deltas include full updated requirement blocks.
- `REMOVED` deltas include `Reason` and `Migration`.
- `RENAMED` deltas are name-only unless paired with `MODIFIED`.
- Requirement IDs are all present or all omitted for the change.
- Requirements do not contradict current specs or each other.

## Design Checks

- Architecture choices directly satisfy requirements.
- Decisions include rationale and alternatives.
- Operational concerns are addressed where relevant: observability, rollback, migration, failure modes.
- Security and performance implications are explicit when relevant.
- The design avoids over-prescribing line-by-line implementation.

## Task Checks

- Tasks map directly to proposal/spec/design deliverables.
- Every requirement ID is covered by at least one task when IDs are used.
- Dependencies and wave sequencing are realistic.
- Verification is concrete for every task.
- Task granularity is actionable and reviewable.
- The plan supports incremental, reversible delivery.

## Output Format

Return findings using this exact severity tag at the start of each item:

- `[blocking]` for issues that must be fixed before implementation proceeds.
- `[suggestion]` for useful non-blocking improvements.
- `[note]` for informational context.

End with exactly one verdict line:

- `Verdict: approve`
- `Verdict: request-changes`
- `Verdict: needs-discussion`

## Approval Rule

Use `request-changes` if any blocking finding exists. Use `needs-discussion` when scope, behavior, or risk cannot be resolved from the artifacts. Use `approve` only when the package is implementable as written.
