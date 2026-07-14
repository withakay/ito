---
name: ito-apply
description: Implement an accepted, main-first Ito change through its authoritative tasks and worktree policy.
---

<!-- ITO:START -->
# Apply lifecycle

Apply only a reviewed proposal that satisfies the repository's main-first policy. If the change is being requested from another branch, verify that the proposal exists on the configured main branch before implementation begins.

1. Determine the full change ID. If it is missing, use `ito list --ready`; ask when more than one change is ready.
2. Render the source of truth and follow it exactly:

```bash
ito agent instruction apply --change "<change-id>"
```

3. Keep the main/control checkout read-only. Create one dedicated full-ID worktree from main, protect locked worktrees, and run the instructed baseline verification before edits.
4. Drive progress with `ito tasks next|ready|start|complete`; do not edit task state directly. Use scoped worker packets and self-review when delegating.
5. Follow RED/GREEN/REFACTOR and preserve task/acceptance scope. Run relevant checks before every completion claim.
6. Make small, change-aligned commits. Require explicit commit confirmation unless auto-mode was explicitly authorized. Run `make check` before commit; do not use `--no-verify` until required checks pass, and avoid stash/pre-commit races.

Hand completed implementation evidence to `ito-review`. Do not archive or integrate merely because task boxes are checked.
<!-- ITO:END -->
