## Why

The `writing-plans` skill duplicates functionality that `ito-proposal` provides. Both create structured task lists for implementation. Maintaining two parallel planning skills creates confusion and inconsistent task formats.

## What Changes

- **Merge `writing-plans` into `ito-proposal`**: Add valuable patterns from `writing-plans`:
  - Bite-sized task granularity guidance (2-5 min steps)
  - TDD flow per task (failing test → run → implement → run → commit)
  - Task structure guidance: exact file paths, complete code, exact commands
  - Plan document header template (goal, architecture, tech stack)
- **Remove `writing-plans`**: Delete from `ito-skills/skills/` and embedded templates
- **Update `subagent-driven-development`**: Remove references to `writing-plans`

## Capabilities

### Modified Capabilities

- `ito-proposal`: Enhanced with task granularity guidance, TDD flow, task structure best practices

### Removed Capabilities

- `writing-plans`: Merged into `ito-proposal` and removed

## Impact

- **ito-proposal skill**: Enhanced with writing-plans patterns (lives in ito workflow skills)
- **ito-skills/skills/writing-plans/**: Deleted
- **ito-skills/skills/subagent-driven-development/SKILL.md**: Update references
- **Embedded templates**: Remove `ito-writing-plans`
- **distribution.rs**: Remove `writing-plans` from ITO_SKILLS list
