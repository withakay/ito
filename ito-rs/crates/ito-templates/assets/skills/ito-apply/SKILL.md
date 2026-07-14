---
name: ito-apply
description: Implement an accepted, main-first Ito change through its authoritative tasks and worktree policy. Use when asked to apply, implement, or begin work on a reviewed change proposal.
---

<!-- ITO:START -->
# Apply lifecycle

Apply only a reviewed proposal that satisfies the repository's main-first policy. Never substitute proposal files from a local feature branch, coordination state, or backend state.

1. Determine the full change ID. If missing, run `ito list --ready`; ask when more than one change is ready.
2. Confirm that the reviewed proposal is authoritative:

   ```bash
   ito change preflight "<change-id>" --for prepare --refresh
   ```

   Stop on failure and follow the reported remediation.
3. Keep the main/control checkout read-only. Create or reuse one dedicated full-ID worktree from main, protect locked worktrees, and never reuse one worktree for two changes. Then verify execution readiness:

   ```bash
   CHANGE_DIR=$(ito worktree ensure --change "<change-id>") && \
   cd "$CHANGE_DIR" && \
   ito change preflight "<change-id>" --for execute
   ```

4. Render the source of truth and follow it exactly:

   ```bash
   ito agent instruction apply --change "<change-id>"
   ```

5. Drive progress with `ito tasks next|ready|start|complete`; do not edit task state directly. Use scoped worker packets and self-review when delegating. Ralph remains available through `ito-loop` after execution readiness passes.
6. Follow RED/GREEN/REFACTOR and preserve task and acceptance scope. Make small, change-aligned commits and run relevant checks before every completion claim.

Hand completed implementation evidence to `ito-review`. Do not archive or integrate merely because task boxes are checked.
<!-- ITO:END -->
