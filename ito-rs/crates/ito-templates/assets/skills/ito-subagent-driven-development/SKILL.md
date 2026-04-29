---
name: ito-subagent-driven-development
description: Use for sequential per-task subagent delegation within one Ito change in the current session.
---

<!-- ITO:START -->

# Subagent-Driven Development

Use this only when implementing one Ito change in the current session with delegated worker agents.

## Steps

1. Run `ito agent instruction apply --change <change-id>` and follow the rendered apply instruction for task tracking, worktree rules, and testing policy.
2. Use `ito tasks ready/start/complete` for task state; do not edit `tasks.md` directly.
3. Dispatch one scoped worker per task with the task text, relevant context, expected files, and verification command.
4. Review each worker result before moving to the next task.
5. Run final verification and then use `ito agent instruction finish --change <change-id>` or the `ito-finish` skill.

Do not duplicate the full apply workflow here; `ito agent instruction apply --change <change-id>` is the source of truth.

<!-- ITO:END -->
