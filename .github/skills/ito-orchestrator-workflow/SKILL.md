---
name: ito-orchestrator-workflow
description: Optional repo-specific supplement for orchestrators after `ito agent instruction orchestrate` has been loaded.
---

<!-- ITO:START -->

This is an optional, repo-specific supplement for the rendered orchestrator instruction.

Use it only for local conventions that cannot be inferred from `ito agent instruction orchestrate`, such as:

- Repository-specific verification commands
- Services or environment variables required before gates run
- Local reviewer expectations or escalation contacts
- Known project gotchas for worker dispatch

Do not copy generic Ito gate order, run-state schema, or remediation packet rules into this skill. Keep those in the generated orchestrate instruction.

<!-- ITO:END -->
