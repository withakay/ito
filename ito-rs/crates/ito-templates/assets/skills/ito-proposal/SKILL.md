---
name: ito-proposal
description: Use when creating and writing an Ito change proposal (new change or existing change id). Delegates to Ito CLI instruction artifacts.
---

This skill is a thin wrapper. The Ito CLI is the source of truth for proposal workflow instructions.

## New Change

```bash
ito agent instruction proposal
```

Follow the printed instructions (collaboration, schema selection, module selection, change creation).

## Existing Change

```bash
ito agent instruction proposal --change "<change-id>"
ito agent instruction specs --change "<change-id>"
ito agent instruction tasks --change "<change-id>"
```

Follow the printed instructions for each artifact exactly.
