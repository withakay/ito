# Change: Deterministic Completion Gate

## Why

Prior sessions found changes and modules marked complete while implementation was missing or incomplete. Agents treated task checkboxes, module checkboxes, or previous summaries as proof of completion. That caused loops where another agent had to rediscover the real repository state.

Ito should provide a deterministic completion verdict that is stronger than task markdown. Ralph, archive, module status, and agents should all use the same verifier.

## What

Add a completion verifier for changes:

```bash
ito change verify-complete <change-id> --json
```

The verifier evaluates task state, required artifacts, spec deltas, validation results, configured checks, archive eligibility, and relevant git/worktree state. It returns a structured verdict with blocking reasons and evidence.

## Impact

LLMs no longer need to decide whether a change is really complete from prose. Ito owns completion truth and can reject premature completion promises.

## Out Of Scope

This change does not implement every project-specific validation. It provides a verifier framework and integrates the default Ito checks. Project-specific validation is covered by `030-06_validation-contract-and-ci-doctor`.

## Success Criteria

- `ito change verify-complete <change> --json` returns `complete`, `incomplete`, or `blocked` with reasons.
- Ralph uses the verifier before accepting a completion promise.
- Archive flows refuse to archive changes that fail the verifier unless explicitly bypassed.
- Tests cover checkbox-only false positives and missing-implementation evidence.
