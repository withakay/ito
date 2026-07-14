---
name: ito-apply
description: |
    Apply a Change Proposal.
    Triggered by the user saying "Apply change <change-id>" or "Implement change <change-id>".
    Use when implementing, executing, applying, building, coding, or developing a feature, change, requirement, enhancement, fix, or modification. Use when running tasks from a spec, proposal, or plan.
---

<!-- ITO:START -->


Run the CLI-generated apply instructions for a specific change.

**Steps**

1. Determine the target change ID.

   - If the user provides one, use it.
   - Otherwise run `ito list --ready` to see changes ready for implementation.
   - Ask the user which change to apply if multiple are ready.

2. Confirm the reviewed proposal is available on authoritative main:

   ```bash
   ito change preflight "<change-id>" --for prepare --refresh
   ```

   Stop on failure and follow the reported remediation. Do not substitute local proposal files, coordination state, or backend state.

3. Create or reuse the verified implementation worktree:

   ```bash
   CHANGE_DIR=$(ito worktree ensure --change "<change-id>") || exit 1
   cd "$CHANGE_DIR"
   ito change preflight "<change-id>" --for execute
   ```

4. Generate instructions (source of truth):
   ```bash
   ito agent instruction apply --change "<change-id>"
   ```

5. Follow the printed instructions exactly.

6. Use `ito tasks ready <change-id>` to see actionable tasks at any point. Iteration/Ralph remains available after execute readiness passes.

<!-- ITO:END -->
