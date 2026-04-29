---
name: ito-finish
description: "Use when implementation is complete, all tests pass, and you need to decide how to integrate the work — presents structured options for merge, PR, or cleanup"
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->


# Finishing a Development Branch

Verify tests, offer the right integration option, execute it safely, then clean up.

## 1. Verify Tests

Run the project's test suite before offering options. If tests fail, stop.

## 2. Determine Base Branch

```bash
git merge-base HEAD main 2>/dev/null || git merge-base HEAD master 2>/dev/null
```

If detection is unclear, ask the user.

## 3. Detect Ito Change

```bash
ito list --changes 2>/dev/null
```

If an Ito change is present, include Option 5.

## 4. Present Options

```
Implementation complete. What would you like to do?

1. Merge back to <base-branch> locally
2. Push and create a Pull Request
3. Keep the branch as-is (I'll handle it later)
4. Discard this work
5. Archive Ito change (integrates specs, marks complete)  [if Ito change detected]

Which option?
```

Keep options concise.

## 5. Execute Choice

| Option | Action | Key rules |
|---|---|---|
| 1 | Merge locally | Merge from the main worktree when worktrees are in use; re-run tests on the merged result |
| 2 | Push + PR | Push branch, open PR, keep worktree if still needed |
| 3 | Keep as-is | Report branch + worktree path; no cleanup |
| 4 | Discard | Require typed `discard` confirmation before deleting branch |
| 5 | Archive Ito change | Run `ito agent instruction archive --change <change-id>` and follow it |

### Option 1: Merge locally

If using worktrees, merge from the main worktree, not the feature worktree.

```bash
# From the main worktree:
git merge <feature-branch>
<test command>          # verify on merged result
```

Then continue to cleanup.

### Option 2: Push and create PR

```bash
git push -u origin <feature-branch>
gh pr create --title "<title>" --body "$(cat <<'EOF'
## Summary
<2-3 bullets>
EOF
)"
```

Then continue to cleanup; keep the worktree if the PR still needs it.

### Option 3: Keep As-Is

Report: `Keeping branch <name>. Worktree preserved at <path>.`

### Option 4: Discard

Require typed confirmation:
```
This will permanently delete branch <name> and all commits.
Type 'discard' to confirm.
```

After confirmation:
```bash
git branch -D <feature-branch>
```

Then continue to cleanup.

### Option 5: Archive Ito Change

```bash
ito agent instruction archive --change <change-id>
```

Follow the printed instructions, then continue to cleanup.

## 6. Cleanup Worktree

For Options 1 and 5, use the CLI-generated cleanup instructions:

```bash
ito agent instruction finish --change <feature-branch>
```

If the worktree is locked, assume that was intentional and keep it.

For Options 2 and 3, keep the worktree.

## Rules

- Never proceed with failing tests.
- Never merge without re-verifying the merged result.
- Never delete work without typed confirmation.
- Never force-push without explicit request.
- If a worktree is locked, do NOT unlock/remove it unless the user explicitly asks.
- Always include the archive option when an Ito change is present.
- Always present the structured option list, not an open-ended question.

<!-- ITO:END -->
