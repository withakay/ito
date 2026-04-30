# Change: Compress selected Ito template prompt assets

## Why
The template prompt assets in `ito-rs/crates/ito-templates` have grown verbose enough to make maintenance and downstream prompt budgets harder to manage. We need a bounded compaction pass that targets operational prompt assets while preserving reviewer-facing proposal and guidance markdown.

## What Changes
- Compact markdown assets for template `AGENTS.md`, skills, agents, commands, and instructions under `ito-rs/crates/ito-templates`.
- Apply an explicit filename exclusion list for change-proposal template artifacts: `spec.md`, `design.md`, `proposal.md`, and `tasks.md`.
- Keep the change scoped to template source assets and leave `.autopilot` and unrelated installed outputs untouched.

## Change Shape
- **Type**: internal template/tooling refinement
- **Risk**: medium
- **Non-goals**: compacting change-proposal/spec authoring templates or changing runtime behavior outside template text compaction

## Impact
- **Affected specs**: `template-markdown-compression`
- **Affected code**: `ito-rs/crates/ito-templates/AGENTS.md`, `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/**`, `ito-rs/crates/ito-templates/assets/agents/**`, `ito-rs/crates/ito-templates/assets/commands/**`, `ito-rs/crates/ito-templates/assets/instructions/**`
