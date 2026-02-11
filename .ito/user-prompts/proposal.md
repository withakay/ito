<!-- ITO:START -->

# Proposal Guidance

This file is for optional, user-authored guidance specific to `ito agent instruction proposal`.

- Ito may update this header block over time.
- Add your proposal guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Your Proposal Guidance

### Proposal Decision Rule

- Create a proposal for new capabilities, breaking behavior/API/schema changes, architecture shifts, and security/performance behavior changes.
- Skip proposal for typo/format/comment-only edits and straightforward bug fixes that restore intended behavior.

### Required Delta Format

- Use only: `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, `## RENAMED Requirements`.
- Every requirement must include at least one scenario header exactly: `#### Scenario: <name>`.
- Use normative wording (`SHALL`/`MUST`) for requirement statements.

### MODIFIED Requirement Safety Rule

- For `MODIFIED`, include the full updated requirement block (requirement text plus scenarios), not partial fragments.

### Validation

- Always run `ito validate <change-id> --strict` before presenting the proposal.
