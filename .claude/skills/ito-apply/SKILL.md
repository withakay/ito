---
name: ito-apply
description: |
    Apply a Change Proposal.
    Triggered by the user saying "Apply change <change-id>" or "Implement change <change-id>".
    Use when implementing, executing, applying, building, coding, or developing a feature, change, requirement, enhancement, fix, or modification. Use when running tasks from a spec, proposal, or plan.
---

Run the CLI-generated apply instructions for a specific change.

**Steps**

1. Determine the target change ID.

   - If the user provides one, use it.
   - Otherwise run `ito list --ready` to see changes ready for implementation.
   - Ask the user which change to apply if multiple are ready.

2. Generate instructions (source of truth):
   ```bash
   ito agent instruction apply --change "<change-id>"
   ```

3. Follow the printed instructions exactly.
   - If the instructions tell you to run `ito sync`, do that before continuing.
   - Do not duplicate apply workflow policy inside this skill.

4. Use `ito tasks ready <change-id>` to see actionable tasks at any point.
