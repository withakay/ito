# Tasks for: 001-11_tdd-red-green-coverage-guidance

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-11_tdd-red-green-coverage-guidance
ito tasks next 001-11_tdd-red-green-coverage-guidance
ito tasks start 001-11_tdd-red-green-coverage-guidance 1.1
ito tasks complete 001-11_tdd-red-green-coverage-guidance 1.1
ito tasks show 001-11_tdd-red-green-coverage-guidance
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add TDD + coverage guidance to installed templates

- **Files**:
  - `ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-proposal.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-apply.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.claude/commands/ito/proposal.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.claude/commands/ito/apply.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.codex/prompts/ito-proposal.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.codex/prompts/ito-apply.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.github/prompts/ito-proposal.prompt.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.github/prompts/ito-apply.prompt.md`
- **Dependencies**: None
- **Action**:
  - Add a concise "Testing Policy" section that directs RED/GREEN/REFACTOR and references a configurable coverage target (default 80%).
  - Include a short snippet showing where the project can override the defaults.
- **Verify**: `make test`
- **Done When**: A fresh `ito init --force --tools all` installs templates that include the new guidance.
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 1.2: Extend template config to include testing defaults

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/config.json`
- **Dependencies**: None
- **Action**: Add default keys for testing policy (TDD workflow + coverage target).
- **Verify**: `make test`
- **Done When**: Installed `.ito/config.json` contains the default testing policy keys.
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Plumb testing policy config into instruction generation

- **Files**: `ito-rs/` (instruction generation + config loading)
- **Action**:
  - Read config via existing cascading config system.
  - Render testing policy guidance into `ito agent instruction proposal|apply` outputs, using configured values.
- **Verify**: `make test`
- **Done When**: A unit/integration test demonstrates that instruction output changes with config overrides.
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 2.2: Update docs to describe TDD + coverage guidance and overrides

- **Files**:
  - `.ito/AGENTS.md` (project docs)
  - `docs/agent-workflow.md` (if present)
- **Action**: Add a short section documenting RED/GREEN/REFACTOR and the default coverage target, with config override examples.
- **Verify**: `ito validate 001-11_tdd-red-green-coverage-guidance --strict`
- **Done When**: Documentation clearly explains defaults and how to override them.
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Human review of wording and default policy

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**:
  - `ito-rs/crates/ito-templates/assets/default/project/`
  - `ito-rs/` instruction generation changes
- **Dependencies**: Task 2.1, Task 2.2
- **Action**: Review that guidance is clear, non-noisy, and the defaults (RED/GREEN/REFACTOR + 80%) are appropriate.
- **Done When**: Reviewer approves phrasing and key naming.
- **Updated At**: 2026-02-04
- **Status**: [x] complete
