---
name: ito-lite-apply
description: Implement Ito Lite markdown changes by reading proposal/spec/design/tasks artifacts and updating manual task statuses. Use when applying or executing a prompt-only Ito Lite change without the ito CLI.
compatibility: No external dependencies; markdown and project-native verification only.
---

# Ito Lite Apply

Use this skill to implement `.ito-lite/changes/<change-id>/` tasks without any Ito CLI support.

If the portable agent prompts are installed, delegated implementation packets should use `ito-worker`, review packets should use `ito-reviewer`, and verification-only packets should use `ito-test-runner`.

## Pre-Implementation Gate

Before editing code, read:

- `.ito-lite/project.md`
- `.ito-lite/changes/<change-id>/proposal.md`
- `.ito-lite/changes/<change-id>/specs/**/*.md`
- `.ito-lite/changes/<change-id>/design.md` when present
- `.ito-lite/changes/<change-id>/tasks.md`

Then apply the manual validation checklist:

- Proposal explains Why, What Changes, Capabilities, and Impact.
- At least one spec delta exists.
- Every requirement has `### Requirement:` and `#### Scenario:`.
- Tasks are concrete and have verification.
- If Requirement IDs are used, every ID is covered by tasks.

If validation fails, fix artifacts or ask the user before implementing.

## Task Selection

Use `tasks.md` as the source of truth.

Manual status values:

- `[ ] pending`
- `[>] in-progress`
- `[x] complete`
- `[-] shelved`

Pick the first pending task whose dependencies are complete. Keep exactly one task in progress unless the user asks for parallel work.

## Status Updates

When starting a task, update only that task:

```markdown
- **Status**: [>] in-progress
- **Updated At**: YYYY-MM-DD
```

When completing a task, update:

```markdown
- **Status**: [x] complete
- **Updated At**: YYYY-MM-DD
```

If a task cannot be completed, mark it shelved only with a reason in the task description or immediately below the status.

## Implementation Rules

- Implement the smallest correct change that satisfies the requirement scenarios.
- Prefer test-first work when the schema is `tdd` or the task describes a regression.
- Use project-native build/test/lint commands from `.ito-lite/project.md` when available.
- Do not change current specs under `.ito-lite/specs/` during implementation; current specs change during archive.
- If the implementation reveals a requirement gap, update the proposal package before continuing.

## Verification

For each task:

1. Perform the task's `Action`.
2. Run or perform the task's `Verify` check.
3. Confirm `Done When` is true.
4. Mark the task complete.

Before reporting the change complete:

- All required tasks are `[x] complete`.
- Any shelved tasks have explicit user acceptance or a documented reason.
- Relevant tests/checks were run, or the reason they could not be run is stated.
- Implementation still matches all requirement scenarios.

## Completion Summary

Report:

- Tasks completed.
- Files changed.
- Verification performed.
- Any skipped verification or shelved work.
- Whether the change is ready for `ito-lite-archive`.
