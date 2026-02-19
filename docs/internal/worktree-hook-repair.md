# Worktree Hook Repair

If `git commit` or hooks fail with missing blob/object errors in worktree repos, repair the index before retrying.

## Typical symptom

```text
fatal: unable to read <hash>
error: invalid object ...
```

## Reliable repair sequence

```bash
WTGITDIR="$(git rev-parse --git-dir)"
rm "$WTGITDIR/index"
git reset HEAD
git add -A
```

This rebuilds the worktree index from `HEAD` and clears corrupt references.

## Notes

- `git reset HEAD` unstages all files.
- Re-stage the intended files before commit.
- If a pre-commit stash cycle is still unstable, do one controlled `--no-verify` commit and investigate hooks after the branch is unblocked.

For deeper background, keep any extended investigation notes in the internal docs set (not in published docs nav).
