---
name: ito-finishing-a-development-branch
description: "Use when implementation is complete, all tests pass, and you need to decide how to integrate the work — presents structured options for merge, PR, or cleanup"
---

# Finishing a Development Branch

Verify tests, present options, execute choice, clean up.

## Step 1: Verify Tests

Run the project's test suite before offering any options. If tests fail, stop — fix them first.

## Step 2: Determine Base Branch

```bash
git merge-base HEAD main 2>/dev/null || git merge-base HEAD master 2>/dev/null
```

Or ask: "This branch split from main — is that correct?"

## Step 3: Detect Ito Change

```bash
ito list --changes 2>/dev/null
```

If an Ito change is detected, include Option 5 (Archive).

## Step 4: Present Options

```
Implementation complete. What would you like to do?

1. Merge back to <base-branch> locally
2. Push and create a Pull Request
3. Keep the branch as-is (I'll handle it later)
4. Discard this work
5. Archive Ito change (integrates specs, marks complete)  [if Ito change detected]

Which option?
```

Keep options concise. Don't add explanation.

## Step 5: Execute Choice

### Option 1: Merge Locally

**Important:** If using worktrees, perform the merge from the main worktree, not the feature worktree.

```bash
# From the main worktree:
git merge <feature-branch>
<test command>          # verify on merged result
git branch -d <feature-branch>
```

Then: Cleanup (Step 6).

### Option 2: Push and Create PR

```bash
git push -u origin <feature-branch>
gh pr create --title "<title>" --body "$(cat <<'EOF'
## Summary
<2-3 bullets>
EOF
)"
```

Then: Cleanup (Step 6). Keep worktree until PR merges if needed.

### Option 3: Keep As-Is

Report: "Keeping branch `<name>`. Worktree preserved at `<path>`."

No cleanup.

### Option 4: Discard

**Require typed confirmation:**
```
This will permanently delete branch <name> and all commits.
Type 'discard' to confirm.
```

After confirmation:
```bash
git branch -D <feature-branch>
```

Then: Cleanup (Step 6).

### Option 5: Archive Ito Change

```bash
ito agent instruction archive --change <change-id>
```

Follow printed instructions. Then: Cleanup (Step 6).

## Step 6: Cleanup Worktree

For Options 1, 4, 5 — remove the worktree if one exists:

```bash
git worktree remove <worktree-path> 2>/dev/null || true
git worktree prune
```

For Options 2 and 3 — keep the worktree.

## Quick Reference

| Option | Merge | Push | Archive | Keep Worktree | Delete Branch |
|--------|-------|------|---------|---------------|---------------|
| 1. Merge | Yes | - | - | No | Yes |
| 2. PR | - | Yes | - | Optional | No |
| 3. Keep | - | - | - | Yes | No |
| 4. Discard | - | - | - | No | Yes (force) |
| 5. Archive | - | - | Yes | No | Yes |

## Rules

- Never proceed with failing tests
- Never merge without verifying tests on the result
- Never delete work without typed confirmation
- Never force-push without explicit request
- Always detect Ito changes and include the archive option
- Always present structured options, not open-ended questions
