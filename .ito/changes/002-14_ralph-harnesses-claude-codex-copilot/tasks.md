# Tasks for: 002-14_ralph-harnesses-claude-codex-copilot

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 002-14_ralph-harnesses-claude-codex-copilot
ito tasks next 002-14_ralph-harnesses-claude-codex-copilot
ito tasks start 002-14_ralph-harnesses-claude-codex-copilot 1.1
ito tasks complete 002-14_ralph-harnesses-claude-codex-copilot 1.1
ito tasks show 002-14_ralph-harnesses-claude-codex-copilot
```

______________________________________________________________________

## Wave 1

- **Depends On**: None
- **Goal**: Core harness implementations

### Task 1.1: Add Claude/Codex/Copilot harness implementations in ito-core

- **Files**: `ito-rs/crates/ito-core/src/harness/`
- **Dependencies**: None
- **Action**:
  - Add `ClaudeCodeHarness`, `CodexHarness`, and `GitHubCopilotHarness` implementing `Harness`.
  - Extend `HarnessName` with constants for the three harnesses.
  - Implement each harness by spawning the documented CLI in non-interactive mode:
    - `claude -p ...` (+ `--model`, + `--dangerously-skip-permissions` when `--allow-all`)
    - `codex exec ...` (+ `--model`, + `--yolo` when `--allow-all`)
    - `copilot -p ...` (+ `--model`, + `--yolo` when `--allow-all`)
  - Reuse the existing streaming + inactivity timeout pattern from `OpencodeHarness`.
- **Verify**: `make test`
- **Done When**: new harness types compile, tests pass, no network required
- **Updated At**: 2026-02-13
- **Status**: [x] complete

### Task 1.2: Make Ralph count git changes for non-stub harnesses

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Update change counting so harnesses that can modify the working tree are not treated as OpenCode-only.
  - Keep `stub` excluded.
- **Verify**: `cargo test -p ito-core --test ralph`
- **Done When**: ralph tests continue to pass and change counting behavior is exercised
- **Updated At**: 2026-02-13
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1
- **Goal**: CLI wiring + user-facing behavior

### Task 2.1: Wire `ito ralph --harness` to new harnesses

- **Files**: `ito-rs/crates/ito-cli/src/commands/ralph.rs`
- **Dependencies**: None
- **Action**:
  - Extend harness selection match to accept `claude`, `codex`, and `github-copilot`.
  - Keep `opencode` and `stub` working.
  - Ensure unknown harnesses return a clear error (per spec delta).
- **Verify**: `cargo test -p ito-cli --test ralph_smoke`
- **Done When**: smoke tests cover the selection and pass
- **Updated At**: 2026-02-13
- **Status**: [x] complete

### Task 2.2: Update CLI help snapshots if needed

- **Files**: `ito-rs/crates/ito-cli/tests/cli_snapshots.rs`, `ito-rs/crates/ito-cli/tests/snapshots/*ralph*`
- **Dependencies**: Task 2.1
- **Action**:
  - Ensure `ito ralph --help` output and documented examples remain accurate.
  - Re-record `insta` snapshots if harness names appear in help text or examples.
- **Verify**: `cargo test -p ito-cli --test cli_snapshots`
- **Done When**: snapshot tests pass
- **Updated At**: 2026-02-13
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2
- **Goal**: Documentation + strict validation

### Task 3.1: Validate change artifacts

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: `.ito/changes/002-14_ralph-harnesses-claude-codex-copilot/proposal.md`, `.ito/changes/002-14_ralph-harnesses-claude-codex-copilot/design.md`, `.ito/changes/002-14_ralph-harnesses-claude-codex-copilot/specs/cli-ralph/spec.md`, `.ito/changes/002-14_ralph-harnesses-claude-codex-copilot/specs/rust-ralph/spec.md`
- **Dependencies**: None
- **Action**:
  - Run strict Ito validation for the change.
  - Confirm the proposal/spec deltas match the intended user behavior.
- **Verify**: `ito validate 002-14_ralph-harnesses-claude-codex-copilot --strict`
- **Done When**: strict validation passes and reviewer agrees the plan is correct
- **Updated At**: 2026-02-13
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
