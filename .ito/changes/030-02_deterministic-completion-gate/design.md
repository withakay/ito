# Design: Deterministic Completion Gate

## Overview

Create a core completion verifier that computes a structured verdict from repository state. The CLI, Ralph, archive, and module status should consume the same API.

## Verifier Inputs

- Change ID and resolved change path.
- Schema requirements for required artifacts.
- Parsed `tasks.md` task states.
- Validation output from Ito artifact validators.
- Optional validation contract from `030-06_validation-contract-and-ci-doctor` when available.
- Git/worktree status relevant to the change.

## Verifier Output

Return JSON with:

- `verdict`: `complete`, `incomplete`, or `blocked`
- `change_id`
- `checks`
- `blocking_reasons`
- `warnings`
- `evidence`
- `suggested_next_commands`

## Integration Points

- CLI command: `ito change verify-complete <change-id> --json`.
- Ralph completion gate: call verifier before accepting completion promise.
- Archive: call verifier before moving active changes to archive.
- Module status: optionally aggregate verifier results for module-level readiness.

## Risks

The verifier could become too strict if it assumes implementation evidence that a documentation-only change cannot provide. Use schema-aware rules and allow explicit task shelving with reasons.
