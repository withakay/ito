# Prek Index Corruption in Worktree Repos

This repo uses `prek` for git hooks. In the bare-repo-with-worktrees layout, we have observed intermittent (and sometimes repeatable) index corruption during hook execution.

## Symptom

During `git commit`, hooks run and then fail with:

```
error: Command `git diff` exited with an error:

[stderr]
fatal: unable to read <hash>
```

Common follow-on symptoms:

- `git diff` fails with `fatal: unable to read <hash>`
- `git fsck` reports `missing blob <hash>`
- `git ls-files --stage` shows the missing blob referenced by some path

In at least one failure, the missing blob was referenced as `README.md`.

## Diagnosis

1) Confirm the repo has a missing object:

```bash
git fsck --no-progress --full
```

2) Identify which path in the index references the missing object:

```bash
git fsck --no-progress --full --name-objects
```

3) Confirm the index has a bad entry:

```bash
git ls-files --stage | rg "missing-hash-here" || true
```

## Why This Happens (Hypothesis)

`prek` stashes and restores changes around hook execution. In the worktree layout, the index lives under the worktree gitdir (for example `.bare/worktrees/<branch>/index`). If a hook run is interrupted or races with other file operations, the index can end up referencing a blob that is no longer present in the shared object store.

This matches the observed failure mode:

- hooks modify/restore the index
- subsequent `git diff` during hook execution fails because it cannot read a referenced blob

## Workarounds

### A) Fast fix when a single path is broken

If `git fsck --name-objects` points at a specific file (for example `README.md`), re-add it:

```bash
git add README.md
```

Then confirm:

```bash
git ls-files --stage | rg "missing-hash-here" || true
git diff >/dev/null
```

### B) Reliable fix: rebuild the worktree index

This fully rebuilds the index from `HEAD` (unstages everything):

```bash
WTGITDIR="$(git rev-parse --git-dir)"
rm "$WTGITDIR/index"
git reset HEAD
git add -A
```

### C) Commit without hooks (temporary)

If hooks repeatedly reintroduce corruption, bypass hooks for the commit:

```bash
git commit --no-verify -m "..."
```

If pre-push hooks also trigger the issue:

```bash
git push --no-verify
```

## Follow-Up

- Reproduce with `PREK_LOG_LEVEL=trace` (or `prek -v`) and capture which hook invocation triggers the broken index.
- Confirm whether the corruption is tied to stash/unstash within `prek`.
- Consider adding a repo-local guard that detects an unreadable index early (and prints the repair steps) before running expensive hooks.
