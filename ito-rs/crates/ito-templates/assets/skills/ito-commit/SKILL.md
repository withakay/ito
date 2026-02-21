---
name: ito-commit
description: Create atomic git commits aligned to Ito changes. Use when you want to commit work after applying a change, optionally with auto-mode.
---

Create atomic git commits aligned to Ito changes.

**Concept:** In Ito-driven workflows, you typically make progress by creating/applying a change. After applying and verifying a change, you should usually create a git commit that corresponds to that change.

**Key behavior**

- Prefer 1 commit per applied Ito change (or a small number of commits if the change is large).
- Include the Ito change id in the commit message when practical (e.g. `001-02_add-tasks`).
- Use Ito inspection commands to anchor the commit to what was actually applied.

## Parameters

When invoking this skill, check for these parameters in context:

- **auto_mode**: boolean flag
  - `true`: create commits immediately without asking for confirmation
  - `false` or missing: ask for confirmation of each commit message
  - CRITICAL: this only applies to the current invocation and is reset afterwards

- **change_id**: optional, an Ito change id (recommended)
  - If missing, prompt the user to pick from `ito list --json`

- **stacked_mode**: optional boolean
  - If `true`, create stacked branches per commit (only if tooling exists)
  - If `false` or missing, commit on current branch

- **ticket_id**: optional identifier to include in commit messages

## Prerequisites

1. Verify repo has changes:
   - `git status --short`
   - If no changes, stop with: "No changes found to commit"

2. Identify Ito change context:
   - If `change_id` not provided, run `ito list --json` and ask user to select
   - Then inspect the change: `ito status --change "<change-id>"`

3. Confirm the change is in a good commit state:
   - Ensure artifacts/tasks are complete enough that a commit makes sense
   - If the change is unfinished, ask user whether to commit "WIP" or wait

## Commit Message Format

Use conventional commit format:

- Format: `type(scope): description`
- Prefer scope = Ito module name or ticket id
- Description should mention the change goal
- Include Ito change id at end, in parentheses, when practical

Examples:

- `feat(todo): add task model and parsing (001-02_add-task-core)`
- `fix(storage): persist tasks atomically (002-01_storage-save)`

## Pre-commit Safety (Agents)

prek (the pre-commit runner) stashes unstaged changes before running hooks during `git commit`. If another process modifies the working tree mid-run, the stash pop can conflict or lose work.

**Agents MUST use the check-then-commit pattern to avoid stash races:**

```bash
# 1. Run all checks WITHOUT stashing (operates on all files, no stash involved)
make check

# 2. Stage files
git add <files>

# 3. Commit with --no-verify to skip the hook (checks already passed)
git commit --no-verify -m "type(scope): description"
```

**Why `--no-verify`?** The `make check` run already validated the code. Running the hook again during commit would re-stash and re-introduce the race condition. The `--no-verify` flag is safe here because validation already happened.

**Advisory lock:** When the pre-commit hook does run (human commits), it acquires an advisory lock at `<gitdir>/precommit.lock`. Before modifying the working tree, agents SHOULD check for this lock:

```bash
# Check if a pre-commit hook is currently running
if ito-rs/tools/precommit-lock.sh check 2>/dev/null; then
    echo "Pre-commit hook is running — wait before modifying files"
    ito-rs/tools/precommit-lock.sh wait --timeout 120
fi
```

## Procedure

1. Read diffs: `git diff` and `git status --short`

2. Run pre-commit checks: `make check`
   - If checks fail, fix issues before proceeding
   - Do NOT skip this step — `--no-verify` is only safe after `make check` passes

3. Stage files for the selected change (prefer staging only files touched by that change)

4. Decide commit messages:
   - If `auto_mode` is true: commit immediately
   - Otherwise: present recommended message + alternatives, ask user to confirm

5. Commit with `--no-verify` flag: `git commit --no-verify -m "<message>"`

6. Verify after each commit: `git status --short`

## Output

After committing, show:

- Change committed: <change-id>
- Commit SHA + message (`git log -1 --oneline`)
- Remaining uncommitted changes (if any)
