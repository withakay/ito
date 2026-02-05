# Change: Centralize Harness Prompts via Agent Skills + `ito agent instruction`

## Why

We currently duplicate long, mostly-identical Markdown instruction bodies across multiple harness layouts in `ito-templates` (Claude/Codex/OpenCode/Copilot). This makes edits error-prone and discourages improvements.

## What Changes

- Establish the Agent Skills spec (`https://agentskills.io/specification`) as the baseline format for “skills”.
- Move the canonical instruction bodies behind `ito agent instruction <artifact>`, so the CLI can generate context-aware instructions (and we can test them).
- Replace per-harness long-form instruction files in templates with thin wrappers (skeletons) that delegate to the CLI-generated instruction artifact.
- Keep harness-specific deviations only where a harness is explicitly incompatible (e.g., GitHub Copilot prompt files are a separate mechanism from Agent Skills).

## Impact

- Affected specs:
  - `cli-init` (installs harness files)
  - `cli-update` (updates installed instruction assets)
  - Potentially: `ito-skill-routing`, `instruction-loader` (depending on implementation approach)
- Affected code:
  - `ito-rs/crates/ito-templates/assets/default/project/**`
  - `ito-rs/crates/ito-cli/src/main.rs` + `ito-rs/crates/ito-core/**` (instruction artifact generation and/or schema)

## Notes / Constraints

- OpenCode historically used singular `.opencode/skill` and `.opencode/command`, but current OpenCode guidance uses `.opencode/skills` and `.opencode/commands`. This change standardizes Ito on the plural paths.
- Codex and Claude Code both claim Agent Skills compatibility, but Codex documents different `name`/`description` length limits; our skills should remain within both sets of limits.
