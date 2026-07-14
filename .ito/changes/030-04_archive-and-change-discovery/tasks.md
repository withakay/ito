# Tasks for: 030-04_archive-and-change-discovery

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-04_archive-and-change-discovery
ito tasks next 030-04_archive-and-change-discovery
ito tasks start 030-04_archive-and-change-discovery 1.1
ito tasks complete 030-04_archive-and-change-discovery 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define archive discovery tests

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito archive list --json`, `ito archive show <change-id> --json`, `ito list --archived --json`, and `ito list --all --json`.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests cover active, archived, and all scopes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Cover archive resolver edge cases

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-domain/tests/**`
- **Dependencies**: None
- **Action**: Add tests for date-prefixed archive directories, full IDs, partial IDs, ambiguous matches, and active-only misses with archived matches.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Resolver errors include scope and suggested archive-aware commands.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement scoped change resolver

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-domain/src/**`
- **Dependencies**: None
- **Action**: Add active, archived, and all scopes to change discovery and resolution.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Core resolver returns scope, canonical ID, path, and ambiguity details.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Implement archive summary model

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-domain/src/**`
- **Dependencies**: None
- **Action**: Add archive summary metadata with canonical ID, archive path, archived date, module ID, status, and scope.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Archive summaries are produced without filesystem globbing by agents.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add archive and list CLI surfaces

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add `ito archive list --json`, `ito archive show <change-id> --json`, `ito list --archived --json`, and `ito list --all --json`.
- **Verify**: `make check`
- **Done When**: CLI outputs are stable and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
