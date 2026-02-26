---
name: ito-tasks
description: Use Ito tasks CLI to manage tasks.md (status/next/start/complete/shelve/add).
---

Use the `ito tasks` CLI to track and update implementation tasks for a change.

**Rules**

- Prefer `ito tasks ...` over manual editing of `tasks.md`.
- Enhanced tasks.md supports `start`, `shelve`, `unshelve`, and `add`.
- Checkbox-only tasks.md is supported in compat mode (supports in-progress via `[~]` / `ito tasks start`, but no shelving); start/complete tasks by 1-based index.

**Common Commands**

```bash
ito tasks status <change-id>
ito tasks next <change-id>
ito tasks ready                               # Show ready tasks across ALL changes
ito tasks ready <change-id>                   # Show ready tasks for a specific change
ito tasks ready --json                        # JSON output for automation
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
ito tasks complete <change-id> <index>
ito tasks shelve <change-id> <task-id>
ito tasks unshelve <change-id> <task-id>
ito tasks add <change-id> "<task name>" --wave <n>
ito tasks show <change-id>
```

**If tasks.md is missing**

- Create enhanced tracking file: `ito tasks init <change-id>`

**If the user asks "what should I do next?"**

- If working on a specific change: Run `ito tasks next <change-id>`
- If looking for any ready work: Run `ito tasks ready` to see all actionable tasks
- Follow the printed Action/Verify/Done When for the chosen task.

**Guardrails**

- If a task is blocked, run `ito tasks status <change-id>` and either resolve blockers or shelve the task (enhanced only).
- If `ito tasks shelve` fails because the file is checkbox-only, explain that checkbox compat mode does not support shelving.
- If `ito tasks start` fails in compat mode, it is usually because the task id is not a 1-based index, or another task is already in-progress.
