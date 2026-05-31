---
name: ito-cleanup
description: Find and remove legacy Ito-managed files left behind by older Ito versions. Use when a repo may contain stale or renamed Ito skills, commands, prompts, adapters, or planning artifacts.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->

# Ito Cleanup

Use this skill to audit a repository for legacy Ito-managed files and remove only the files the user approves.

## Workflow

1. Run the cleanup instruction generator:

   ```bash
   ito agent instruction cleanup
   ```

2. Follow the returned instruction step by step.

3. Report every deletion candidate before removing anything. Include:
   - path
   - reason
   - replacement path, when available
   - whether the file appears Ito-managed

4. Ask the user to confirm the exact deletion list.

5. Delete only confirmed paths, then rerun the scan and show `git status --short`.

## Rules

- Do not delete anything before confirmation.
- Do not delete user-owned files.
- Treat files outside Ito-managed directories as out of scope unless the cleanup instruction explicitly identifies them as legacy Ito artifacts.
- If the generated instruction and this skill disagree, follow the generated instruction.

<!-- ITO:END -->
