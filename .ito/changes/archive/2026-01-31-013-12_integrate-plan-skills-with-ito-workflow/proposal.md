## Why

The `executing-plans` skill duplicates functionality that `ito-apply` provides. Both execute tasks from a plan with progress tracking. Maintaining two parallel execution skills creates confusion and inconsistent behavior.

## What Changes

- **Merge `executing-plans` into `ito-apply`**: Add valuable patterns from `executing-plans`:
  - Batch execution with review checkpoints (default: 3 tasks per batch)
  - Critical review step before starting
  - Explicit "when to stop and ask for help" guidance
  - Handoff to `ito-finish` on completion
  - Safety check: never start on main/master without consent
- **Remove `executing-plans`**: Delete from `ito-skills/skills/` and embedded templates
- **Update `subagent-driven-development`**: Remove `superpowers:*` references, point to `ito-apply`

## Capabilities

### Modified Capabilities

- `ito-apply`: Enhanced with batch execution, review checkpoints, stop conditions, and completion handoff

### Removed Capabilities

- `executing-plans`: Merged into `ito-apply` and removed

## Impact

- **ito-apply skill**: Enhanced with executing-plans patterns (lives in ito workflow skills, not ito-skills)
- **ito-skills/skills/executing-plans/**: Deleted
- **ito-skills/skills/subagent-driven-development/SKILL.md**: Update references
- **Embedded templates**: Remove `ito-executing-plans`
- **distribution.rs**: Remove `executing-plans` from ITO_SKILLS list
