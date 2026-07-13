<!-- ITO:START -->
## Why

Ito currently installs roughly thirty narrowly scoped skills plus harness-specific role skills, command wrappers, and routing helpers. The surface makes Ito harder for agents to understand, duplicates lifecycle policy across files, and obscures its core use case: research and review a specification, integrate it, implement it, and archive the accepted result.

The default installation should present a small, memorable lifecycle. Supporting guidance still matters, but it belongs inside the lifecycle phase that owns it rather than as another top-level skill competing for activation.

## What Changes

- Define one canonical default inventory containing exactly seven skills: `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`.
- Make every supported harness manifest and installer use that inventory and assert the same result.
- Fold intake, brainstorming, planning, tasks, worktree setup, verification, finish/commit, memory, wiki, orchestration, path, update, cleanup, and related helper guidance into the appropriate lifecycle skill phase or CLI-emitted instruction.
- Simplify the `ito` router so it exposes the six lifecycle destinations and does not route to removed helper skills.
- Stop installing delegated agent roles as additional skill directories; retain only harness-native agent definitions where a runtime genuinely needs them.
- Remove redundant Ito-managed skill/command assets and add ownership-aware cleanup so upgrades prune obsolete managed copies while preserving user-authored skills and content outside Ito-managed files.
- Keep Ralph/iteration available by default through `ito-loop`.

## Change Shape
- **Type**: refactor
- **Risk**: high
- **Stateful**: no
- **Public Contract**: cli
- **Design Needed**: yes
- **Design Reason**: This changes the default agent-facing contract across shared templates, five harness adapters, role activation, managed cleanup, commands, routing, and lifecycle documentation.

## Capabilities
### New Capabilities
- `lifecycle-skill-profile`: Define the exact seven-skill default contract and ownership rules for consolidated lifecycle guidance.

### Modified Capabilities
- `ito-skill-routing`: Route agents through the seven-skill lifecycle without exposing redundant helper destinations.
- `agent-surface-taxonomy`: Distinguish harness-native agent definitions from installed skills and prevent delegated roles from expanding the default skill inventory.
- `distribution`: Install the same exact skill inventory for every supported harness.
- `ito-update-repo-skill`: Prune obsolete Ito-managed skills and commands safely during update/upgrade while preserving user-owned content.

## Impact

- Shared skill assets and embedded asset enumeration in `ito-templates`.
- Harness manifests/installers for OpenCode, Claude, Codex, Pi, and GitHub Copilot.
- Agent activation inventory and harness-native role templates.
- Legacy managed-path cleanup, update idempotence, and marker ownership rules.
- Command/prompt wrappers, router documentation, wiki synthesis, tests, and release notes.
- Existing users lose direct activation names for retired helper skills; equivalent lifecycle guidance is reachable from the owning retained skill or CLI instruction.
<!-- ITO:END -->
