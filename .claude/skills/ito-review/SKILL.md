---
name: ito-review
description: Review and validate Ito changes, specs, or implementations. Use when the user wants a quality check, code review, or validation of project artifacts.
---

Run the CLI-generated review instructions for a specific change.

This skill uses:

```bash
ito agent instruction review --change "<change-id>"
```

to generate a structured peer-review checklist before implementation.

**Steps**

1. Determine the target change ID (ask the user if unclear).

2. Generate instructions (source of truth):

   ```bash
   ito agent instruction review --change "<change-id>"
   ```

3. Follow the printed instructions exactly.

4. Return findings using tags and verdict required by the instruction template:

   - Prefix each item with `[blocking]`, `[suggestion]`, or `[note]`.
   - End with `Verdict: approve`, `Verdict: request-changes`, or `Verdict: needs-discussion`.
