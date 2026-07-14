# Tasks for: 006-18_dedupe-harness-prompts

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Created**: 2026-01-30

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Document Harness Compatibility (Agent Skills First)

- **Files**: `.ito/changes/006-18_dedupe-harness-prompts/research/harness-compat.md`
- **Dependencies**: None
- **Action**:
  - Capture the Agent Skills spec as the baseline.
  - Summarize how Claude Code, OpenCode, Codex, and GitHub Copilot support skills + custom commands.
  - Call out explicit incompatibilities / deviations only.
- **Verify**: `ito validate 006-18_dedupe-harness-prompts --strict`
- **Done When**: The change includes a durable, referenced compatibility matrix that we can implement against.
- **Updated At**: 2026-01-30
- **Status**: \[x\] complete

### Task 1.2: Decide/OpenCode Path Strategy (Singular vs Plural)

- **Files**: `.ito/changes/006-18_dedupe-harness-prompts/proposal.md`
- **Dependencies**: Task 1.1
- **Action**:
  - Reconcile Ito's existing OpenCode paths (currently referenced as `.opencode/skill/...` in specs/templates) with OpenCode's current docs (which use `.opencode/skills/...` and `.opencode/commands/...`).
  - Pick a compatibility strategy (e.g., install both paths with thin wrappers).
- **Verify**: `ito validate 006-18_dedupe-harness-prompts --strict`
- **Done When**: Proposal calls out the chosen strategy and its impact on templates/specs.
- **Updated At**: 2026-01-30
- **Status**: \[x\] complete

### Task 1.3: Draft Proposal + Spec Deltas

- **Files**: `.ito/changes/006-18_dedupe-harness-prompts/proposal.md`, `.ito/changes/006-18_dedupe-harness-prompts/specs/**/spec.md`
- **Dependencies**: Task 1.2
- **Action**:
  - Write `proposal.md` describing centralizing instruction bodies behind `ito agent instruction <artifact>`.
  - Add delta specs for impacted capabilities (at least `cli-init` and `cli-update`), reflecting that templates ship thin wrappers and the CLI generates instruction bodies.
- **Verify**: `ito validate 006-18_dedupe-harness-prompts --strict`
- **Done When**: Proposal + deltas pass strict validation and are ready for review.
- **Updated At**: 2026-01-30
- **Status**: \[x\] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-30
- **Status**: \[x\] complete
