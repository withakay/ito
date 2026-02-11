---
name: ito-workflow
description: Ito workflow delegation - delegates all workflow content to Ito CLI instruction artifacts.
---

This skill delegates workflow operations to the Ito CLI.

**Principle**: The Ito CLI is the source of truth for workflow instructions. Skills should be thin wrappers that invoke the CLI and follow its output.

## Available CLI Commands

### Change Management

```bash
ito create change "<name>" --module <module-id>
ito list [--json]
ito list --ready                    # Show only changes ready for implementation
ito list --pending                  # Show changes with 0/N tasks complete
ito list --partial                  # Show changes with 1..N-1/N tasks complete
ito list --completed                # Show changes with N/N tasks complete
ito status --change "<change-id>"
```

### Agent Instructions

```bash
ito agent instruction proposal --change "<change-id>"
ito agent instruction specs --change "<change-id>"
ito agent instruction tasks --change "<change-id>"
ito agent instruction apply --change "<change-id>"
ito agent instruction review --change "<change-id>"
ito agent instruction archive --change "<change-id>"

# Worktrees / multi-branch workflow (per-developer)
ito agent instruction worktrees
```

### Task Management

```bash
ito tasks status <change-id>
ito tasks next <change-id>
ito tasks ready                     # Show ready tasks across all changes
ito tasks ready <change-id>         # Show ready tasks for a specific change
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
```

## Workflow Pattern

1. Run the appropriate `ito agent instruction` command
2. Read the output carefully
3. Follow the printed instructions exactly
4. Use `ito tasks` to track progress

## Related Skills

- `ito-write-change-proposal` - Create new changes
- `ito-apply-change-proposal` - Implement changes
- `ito-review` - Review changes
- `ito-archive` - Archive completed changes
- `ito-tasks` - Manage tasks
- `ito-commit` - Create commits
