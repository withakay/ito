# Tasks for: 016-13_optimize-agent-instructions

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 016-13_optimize-agent-instructions
ito tasks next 016-13_optimize-agent-instructions
ito tasks start 016-13_optimize-agent-instructions 1.1
ito tasks complete 016-13_optimize-agent-instructions 1.1
ito tasks shelve 016-13_optimize-agent-instructions 1.1
ito tasks unshelve 016-13_optimize-agent-instructions 1.1
ito tasks show 016-13_optimize-agent-instructions
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Cache cascading config once per invocation

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-config/**`
- **Dependencies**: None
- **Action**:
  - Introduce a per-invocation cache for the merged cascading project config.
  - Refactor instruction helpers to accept a resolved config value (or access it via runtime) instead of reloading the config multiple times.
  - Add unit tests (using injected filesystem if needed) that assert the config is resolved at most once per invocation.
- **Verify**: `make check`
- **Done When**: Instruction generation paths reuse a single resolved config view and tests cover the one-load behavior.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.2: Make coordination-branch fetch opt-in for apply instructions

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `.ito/specs/**`
- **Dependencies**: Task 1.1
- **Action**:
  - Change `ito agent instruction apply` to skip `git fetch` by default.
  - Add an explicit opt-in (CLI flag and/or config) to fetch the coordination branch before printing apply instructions.
  - Preserve existing error classification behavior (missing remote branch is non-fatal and becomes a warning).
- **Verify**: `make check`
- **Done When**: Apply instruction generation no longer blocks on network I/O by default and supports opt-in sync.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add regression tests for no-fetch-by-default

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-core/src/git.rs`
- **Dependencies**: None
- **Action**:
  - Add tests that assert `ito agent instruction apply` does not call `git fetch` by default.
  - Add tests that assert fetch is attempted when opt-in sync is enabled.
- **Verify**: `make test`
- **Done When**: Tests fail if a future change reintroduces default network I/O into instruction generation.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 2.2: Document the opt-in coordination sync behavior

- **Files**: `.ito/specs/**`, `docs/**` (if applicable)
- **Dependencies**: None
- **Action**:
  - Update relevant specs or docs so the default and opt-in behaviors are clear.
- **Verify**: `ito validate 016-13_optimize-agent-instructions --strict`
- **Done When**: Specs/docs match implemented behavior and validation passes.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review the proposal/spec deltas after `ito validate 016-13_optimize-agent-instructions --strict` passes
- **Done When**: User approves the proposal
- **Updated At**: 2026-02-25
- **Status**: [ ] pending
