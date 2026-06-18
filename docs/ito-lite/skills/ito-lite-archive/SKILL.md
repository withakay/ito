---
name: ito-lite-archive
description: Archive completed Ito Lite changes by manually merging spec deltas into current specs and moving the change to archive. Use after implementation and verification when no ito CLI is available.
compatibility: No external dependencies; markdown and file editing only.
---

# Ito Lite Archive

Use this skill to archive a completed `.ito-lite/changes/<change-id>/` change without the Ito CLI.

## Safety Rule

Do not archive without explicit user confirmation. Archiving updates current specs and moves the change.

## Pre-Archive Checklist

Read:

- `.ito-lite/changes/<change-id>/proposal.md`
- `.ito-lite/changes/<change-id>/specs/**/*.md`
- `.ito-lite/changes/<change-id>/design.md` when present
- `.ito-lite/changes/<change-id>/tasks.md`
- Relevant current specs under `.ito-lite/specs/`

Confirm:

- All required tasks are `[x] complete`.
- Any `[-] shelved` tasks have an explicit reason and user acceptance.
- Verification has been run or skipped with an explicit reason.
- Manual validation passes for spec delta structure.

## Merge Delta Specs

For each `.ito-lite/changes/<change-id>/specs/<capability>/spec.md`, update `.ito-lite/specs/<capability>/spec.md`.

If the target current spec does not exist, create it with a title and the added requirements.

### ADDED Requirements

Append each `### Requirement:` block under the capability's current requirements.

Do not duplicate a requirement that already exists. If the same behavior already exists, stop and ask whether to convert to `MODIFIED` or skip.

### MODIFIED Requirements

Find the matching existing `### Requirement:` block in the current spec and replace the entire block with the modified block.

The replacement block extends from its `### Requirement:` header through the line before the next `### Requirement:` header or the end of the file.

If no matching requirement exists, stop and ask whether this should be `ADDED`.

### REMOVED Requirements

Remove the matching existing requirement block from the current spec.

Preserve the removal rationale in the archived change; do not leave removed requirements in current specs unless the project intentionally keeps a deprecated section.

If no matching requirement exists, record that it was already absent in archive notes.

### RENAMED Requirements

Rename the matching requirement header exactly as specified:

```markdown
- FROM: `### Requirement: <old name>`
- TO: `### Requirement: <new name>`
```

If behavior changes too, apply the corresponding `MODIFIED` block after renaming.

## Post-Merge Validation

Validate current specs manually:

- Every requirement has `### Requirement:`.
- Every requirement has at least one `#### Scenario:`.
- Scenarios still use clear `WHEN` and `THEN` behavior.
- Requirement IDs remain unique within each capability when used.
- No old and new names both remain after a rename.
- Removed requirements are absent from current specs.

## Archive Notes

Add or update `.ito-lite/changes/<change-id>/archive-notes.md` before moving the change:

```markdown
# Archive Notes: <change-id>

## Date

YYYY-MM-DD

## Summary

- <what was implemented and merged>

## Specs Updated

- `.ito-lite/specs/<capability>/spec.md`: <added/modified/removed/renamed>

## Verification

- <tests/checks/manual validation performed>

## Follow-Up

- <none or list>
```

## Move To Archive

Move:

```text
.ito-lite/changes/<change-id>/
```

to:

```text
.ito-lite/changes/archive/YYYY-MM-DD-<change-id>/
```

If `.ito-lite/wiki/` exists, refresh the relevant topic page with links or references to the archived change and affected specs.

## Final Report

Report:

- Archived path.
- Specs updated.
- Validation performed.
- Any follow-up or unresolved risk.
