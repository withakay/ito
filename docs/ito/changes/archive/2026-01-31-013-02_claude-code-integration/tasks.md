# Tasks for: 013-02_claude-code-integration

## Execution Notes

- **Tool**: Claude Code (development), any (implementation)
- **Mode**: Sequential
- **Created**: 2026-01-31
- **Rust**: When modifying Rust/template plumbing, follow the `rust-style` skill

```bash
ito tasks status 013-02_claude-code-integration
ito tasks next 013-02_claude-code-integration
ito tasks start 013-02_claude-code-integration 1.1
ito tasks complete 013-02_claude-code-integration 1.1
ito tasks show 013-02_claude-code-integration
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add a minimal Claude Code skill that delegates to Ito CLI instructions

- **Files**: `.claude/skills/ito-workflow.md`, `.ito/changes/013-02_claude-code-integration/design.md`
- **Dependencies**: None
- **Action**:
  - Create `.claude/skills/ito-workflow.md` that:
    - Mentions `ito agent instruction bootstrap --tool claude` as the canonical preamble.
    - Keeps workflow bodies out of the skill (delegate to `ito agent instruction proposal|apply|review|archive ...`).
- **Verify**: `ito validate 013-02_claude-code-integration --strict`
- **Done When**: Claude Code loads a short skill that points to the CLI instruction artifacts
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Add template assets and (optional) session-start shim

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/`, `ito-skills/adapters/claude/`
- **Dependencies**: Task 1.1
- **Action**:
  - Embed `.claude/skills/ito-workflow.md` in the default project template.
  - If needed for non-project contexts, add a minimal `SessionStart` hook shim that only prints a pointer to:
    - `ito agent instruction bootstrap --tool claude`
  - Document any deprecation path for `ito-skills/hooks/`.
  - When editing Rust for template embedding, apply the `rust-style` skill conventions.
- **Verify**: `make test`
- **Done When**: `ito init --tools claude` installs the Claude integration consistently
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Files**: `.claude/skills/ito-workflow/SKILL.md`, `.ito/changes/013-02_claude-code-integration/proposal.md`
- **Dependencies**: None
- **Action**: Review that workflow content is not duplicated and delegates to CLI
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [x] completed
