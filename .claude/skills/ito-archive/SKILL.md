---
name: ito-archive
description: Archive a completed change and update main specifications. Use when the user has finished implementing and wants to integrate the change into the main codebase.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->
# Archive lifecycle

Require explicit user confirmation before archive. Determine the full change ID, reverify the integrated result, and render the source of truth:

```bash
ito agent instruction archive --change "<change-id>"
```

Follow its spec promotion and archive sequence exactly. `ito-archive` owns accepted delta-spec promotion; there is no separate archive-change or sync-specs skill. Report the archive location, schema, promoted specs, verification evidence, and any change with no delta specs.

After success, refresh relevant `.ito/wiki/` topic/index/status material when useful and capture durable lessons through the configured provider:

```bash
ito agent instruction memory-capture --context "<decision and rationale>"
```

Use the finish/cleanup instruction for branch and worktree follow-through. Preserve locked worktrees, require typed confirmation before destructive discard, and never force-push implicitly. Wiki or memory follow-through is recommended and must not hide an archive failure.
<!-- ITO:END -->
