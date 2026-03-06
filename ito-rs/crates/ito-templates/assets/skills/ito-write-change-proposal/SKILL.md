---
name: ito-write-change-proposal
description: Use when creating, designing, planning, proposing, or specifying a feature, change, requirement, enhancement, fix, modification, or spec. Use when writing tasks, proposals, specifications, or requirements for new work.
---

Delegate to the CLI-generated proposal instructions.

**Primary workflow (source of truth)**

```bash
ito agent instruction proposal
```

Follow the printed instructions. This guide covers collaboration, schema selection, module selection, and change creation.

If the user already has a change ID:

```bash
ito agent instruction proposal --change "<change-id>"
ito agent instruction specs --change "<change-id>"
ito agent instruction tasks --change "<change-id>"
```

**Testing policy**: follow the policy printed by `ito agent instruction proposal --change <id>`.
