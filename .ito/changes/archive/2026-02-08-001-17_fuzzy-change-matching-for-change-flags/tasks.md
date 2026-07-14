# Tasks for: 001-17_fuzzy-change-matching-for-change-flags

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-17_fuzzy-change-matching-for-change-flags
ito tasks next 001-17_fuzzy-change-matching-for-change-flags
ito tasks start 001-17_fuzzy-change-matching-for-change-flags 1.1
ito tasks complete 001-17_fuzzy-change-matching-for-change-flags 1.1
ito tasks show 001-17_fuzzy-change-matching-for-change-flags
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define resolver behavior and tests

- **Files**: `ito-rs/` (new/updated resolver module + tests)
- **Dependencies**: None
- **Action**:
  - Add a shared change-target resolver used by all `--change` / `--change-id` flags.
  - Implement deterministic matching per `cli-change-targets` scenarios.
  - Ensure support for dropping leading zeros (`1-12` -> `001-12_*`).
  - Ensure archived changes are excluded by default.
- **Verify**: `make test`
- **Done When**: unit tests cover unique match, ambiguity, and not-found.
- **Updated At**: 2026-02-06
- **Status**: [x] complete

### Task 1.2: Wire resolver into all `--change`/`--change-id` flags

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**:
  - Find every CLI arg named `--change` or `--change-id` and route it through the shared resolver.
  - Keep behavior non-interactive when a flag is provided (error on ambiguity).
- **Verify**: `make test`
- **Done When**: integration tests demonstrate consistent resolution across representative commands.
- **Updated At**: 2026-02-06
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review matching UX

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review error messages and candidate suggestions for ambiguity/not-found.
- **Done When**: User confirms UX is acceptable.
- **Updated At**: 2026-02-05
- **Status**: [x] complete
