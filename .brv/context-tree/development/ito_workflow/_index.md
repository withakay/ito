---
children_hash: 1679a12bedaeb4588be6b1cf411664756ea0c992690cdeb1895e02e4b5ef2fa8
compression_ratio: 0.3681632653061224
condensation_order: 1
covers: [repo_level_ito_refresh_audit.md, worktree_validation_flow.md]
covers_token_total: 1225
summary_level: d1
token_count: 451
type: summary
---
# Development / Ito Workflow

This level groups two related workflow notes for the Ito repo: one about **repo-level refresh auditing** and one about **worktree validation behavior**. Together they describe how the Ito tooling stays safe and repeatable across managed harness assets and change-specific worktrees.

## repo_level_ito_refresh_audit.md
- Defines the scope of **managed Ito harness assets** for refreshes:
  - `ito-rs/crates/ito-templates/assets/skills`
  - `ito-rs/crates/ito-templates/assets/commands`
  - default project command: `ito-project-setup`
- Establishes the refresh flow:
  - refresh harness assets -> audit for `ito-*` orphans -> skip user-owned entries -> rerun `ito init --update --tools all` -> confirm unchanged git diff hash
- Key outcome: **no `ito-*` orphan skills or commands remained**, and rerunning the refresh was **idempotent**.
- Important boundary: non-Ito files such as `.claude/skills/byterover*` and `.opencode/commands/compare-workflow-tool.md` are **user-owned** and must not be touched.
- Drill down for details on audit scope, idempotence, and ownership rules.

## worktree_validation_flow.md
- Documents the dedicated **read-only worktree validation flow** used by `ito worktree validate --change <id> [--json]`.
- Validation behavior:
  - emits **machine-readable status**
  - **hard-fails** main/control checkouts
  - returns **advisory mismatch guidance** for non-main mismatches
  - matches on **exact change-id prefixes** to avoid false positives
- Key relationship: OpenCode **pre-tool hooks** depend on the machine-readable status to gate execution correctly.
- Important nuance: suffix worktrees like `<change>-review` are handled by prefix matching, not broad substring matching.
- Drill down for the CLI policy, failure modes, and matching rules.