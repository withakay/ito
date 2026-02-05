# Tasks for: 013-01_opencode-adapter

## Execution Notes

- **Tool**: OpenCode (development), any (implementation)
- **Mode**: Sequential
- **Created**: 2026-01-31
- **Tracking**: Prefer the tasks CLI
- **Rust**: When modifying Rust/template plumbing, follow the `rust-style` skill

```bash
ito tasks status 013-01_opencode-adapter
ito tasks next 013-01_opencode-adapter
ito tasks start 013-01_opencode-adapter 1.1
ito tasks complete 013-01_opencode-adapter 1.1
ito tasks show 013-01_opencode-adapter
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement OpenCode plugin that injects bootstrap instructions

- **Files**: `ito-skills/adapters/opencode/ito-skills.js`, `.ito/changes/013-01_opencode-adapter/design.md`
- **Dependencies**: None
- **Action**:
  - Add a Ito-owned OpenCode plugin at `ito-skills/adapters/opencode/ito-skills.js`.
  - Use `experimental.chat.system.transform` to inject a short bootstrap that delegates to:
    - `ito agent instruction bootstrap --tool opencode`
  - Resolve skills from `${OPENCODE_CONFIG_DIR}/skills/ito-skills/` (never via relative paths).
  - Keep plugin stateless and avoid intercepting tools beyond the prompt transform.
- **Verify**:
  - `node -c ito-skills/adapters/opencode/ito-skills.js` (syntax)
  - `ito-skills/tests/opencode/run-tests.sh` (if applicable)
- **Done When**: Plugin can be copy-installed and always points to a stable skills location
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Add template assets for OpenCode plugin + ito-skills skill bundle

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/`
- **Dependencies**: Task 1.1
- **Action**:
  - Embed the OpenCode plugin into the default project template.
  - Embed the `ito-skills` skill bundle into the default project template under OpenCode skills.
  - Ensure installed layout matches the manifest in `.ito/changes/013-05_distribution-fetch-mechanics/proposal.md`.
  - When editing Rust for template embedding, apply the `rust-style` skill conventions.
- **Verify**: `make test`
- **Done When**: `ito init --tools opencode` installs both plugin and skills without repo-relative assumptions
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Files**: `.ito/changes/013-01_opencode-adapter/proposal.md`, `ito-skills/adapters/opencode/ito-skills.js`
- **Dependencies**: None
- **Action**: Review the OpenCode bootstrap approach and destination paths
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [x] completed
