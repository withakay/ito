---
name: ito-finish
description: "Use when implementation is complete, all tests pass, and you need to decide how to integrate the work"
---

# Finishing a Development Branch

Use the CLI-generated finish instructions as the source of truth.

## Steps

1. Determine the target change/branch.
   - If the user provides one, use it.
   - Otherwise inspect the current branch and ask only if needed.

2. Generate instructions:
   ```bash
   ito agent instruction finish --change "<change-id-or-branch>"
   ```

3. Follow the printed instructions exactly.
   - If the finish instruction asks `Do you want to archive this change now?`, ask that question verbatim.
   - If the user chooses to archive, delegate to the CLI-generated archive instructions.
   - Do not maintain a separate finish/archive workflow in this skill.

   If the command fails or returns an error:
   - Report the error message to the user.
   - Suggest checking that the change/branch exists and the working tree is clean.
