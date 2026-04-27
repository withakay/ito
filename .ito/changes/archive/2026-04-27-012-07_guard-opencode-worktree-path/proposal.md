<!-- ITO:START -->
## Why

Instruction-only guidance reduces mistakes, but OpenCode agents can still invoke tools from the wrong checkout. Worktree-enabled projects need a fast, low-friction guard that warns or blocks before tool execution when an agent is on main/control or in a branch/path that does not match the active change ID.

## What Changes

- Add a lightweight CLI validation command that checks whether the current working directory is an acceptable worktree for a specified change.
- Extend the OpenCode `ito-skills.js` pre-tool hook to call the validator quickly before relevant tool use.
- Prioritize detecting the dangerous case: operating from main/control or the configured default path when worktrees are enabled.
- Soft-check that the branch name and worktree path contain the full change ID, without making suffixes or alternate same-change worktrees impossible.
- Provide clear, actionable output telling the agent which worktree path is expected when validation fails.

## Capabilities

### New Capabilities

<!-- None. -->

### Modified Capabilities

- `cli-config`: worktree configuration gains validation behavior for current-change worktree checks.
- `cli-artifact-workflow`: OpenCode adapter pre-tool guidance and generated artifacts use the validator to keep agents on the right worktree.

## Impact

- Affected code: `ito-rs/crates/ito-cli`, `ito-rs/crates/ito-core`, worktree path/config helpers, `ito-rs/crates/ito-templates/assets/adapters/opencode/ito-skills.js`, and installed `.opencode/plugins/ito-skills.js` template output.
- Affected behavior: OpenCode tool use may receive a warning or rejection before running from main/control in worktree-enabled projects.
- No new external dependencies are expected.
<!-- ITO:END -->
