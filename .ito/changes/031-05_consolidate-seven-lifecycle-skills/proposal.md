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
- Retire dedicated planning, orchestration-workflow, update-repo, archive-change, spec-sync, and tmux skill/command surfaces; keep their in-scope behavior in the owning lifecycle phase or direct CLI and remove tmux integration entirely.
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
- `cli-skills`: Install the same exact skill inventory for every supported harness through init/update.
- `cli-update`: Prune obsolete Ito-managed skills and commands safely during update/upgrade while preserving user-owned content.
- `template-assets`: Stop expressing delegated Codex roles as additional installed skills and consolidate obsolete orchestration assets.
- `agent-memory-abstraction`: Retain memory instruction artifacts while folding their lifecycle guidance into retained skills instead of installing `ito-memory`.
- `planning-workflow`: Move exploratory pre-proposal planning into `ito-proposal` while retaining topic-specific `.ito/planning/` artifacts.
- `cli-plan`: Keep direct planning-workspace commands and point their guidance to `ito-proposal` rather than a retired planning skill.
- `ito-slash-command`: Remove the dedicated `ito-plan` wrapper from managed harness command surfaces.
- `ito-update-repo-skill`: Retire the standalone update-repo skill and its command shells; keep essential managed update behavior in the CLI and retained `ito` lifecycle guidance.
- `ito-init`: Retire helper-specific post-init setup advisories; retained `ito` guidance and direct CLI validation remain available.
- `orchestrate-setup`: Replace the standalone orchestration setup skill with instruction-backed guidance reachable from retained lifecycle skills.
- `orchestrate-instruction`: Keep orchestration policy authoritative in the rendered instruction without requiring retired setup/workflow skills.
- `orchestrate-workflow-skill`: Retire generated workflow skills and keep project orchestration policy in user prompts composed by authoritative instructions.
- `pre-commit-hooks`: Preserve opt-in downstream hook guidance without routing through the retired `ito-update-repo` skill.
- `ito-managed-asset-versioning`: Expose stamp diagnostics through direct update/validation tooling without a helper-skill dependency.
- `ito-managed-asset-naming`: Enforce the exact lifecycle inventory in the templates bundle instead of treating every prefixed helper as valid.
- `validate-repo-coordination-rules`: Replace legacy symlink-rule remediation that names `ito-update-repo` with direct CLI/instruction guidance.
- `ito-tmux-skill`: Remove tmux skill distribution and its managed helper scripts from Ito.
- `ito-archive-change-skill`: Make retained `ito-archive` own archive, accepted spec promotion, and archive output.
- `ito-sync-specs-skill`: Retire the standalone spec-sync skill and fold reconciliation into archive.

## Impact

- Shared skill assets and embedded asset enumeration in `ito-templates`.
- Harness manifests/installers for OpenCode, Claude, Codex, Pi, and GitHub Copilot.
- Agent activation inventory and harness-native role templates.
- Legacy managed-path cleanup, update idempotence, and marker ownership rules.
- Command/prompt wrappers, router documentation, wiki synthesis, tests, and release notes.
- Planning workspace hints, orchestration setup/project prompts, archive/spec-promotion guidance, validation remediation, and managed version/naming diagnostics.
- Tmux skill assets and scripts are removed with no Ito-managed replacement.
- Existing users lose direct activation names for retired helper skills; equivalent lifecycle guidance is reachable from the owning retained skill or CLI instruction.
<!-- ITO:END -->
