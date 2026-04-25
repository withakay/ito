# Task 2.3: Skill wrappers around CLI instructions

*2026-04-24T06:40:43Z by Showboat 0.6.1*
<!-- showboat-id: 14a0c8fd-2be8-477d-9d85-fa06af5a4ae8 -->

Updated the repo-installed OpenCode and Claude ito skill wrappers so apply/archive/finish defer to CLI-generated instructions and the worktree helpers tell agents to run ito sync before relying on coordination-backed state.

```bash
ito validate 025-09_add-worktree-sync-command --strict
```

```output
Change '025-09_add-worktree-sync-command' is valid
```
