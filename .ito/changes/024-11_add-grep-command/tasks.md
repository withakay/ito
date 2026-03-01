# Tasks for: 024-11_add-grep-command

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-11_add-grep-command
ito tasks next 024-11_add-grep-command
ito tasks start 024-11_add-grep-command 1.1
ito tasks complete 024-11_add-grep-command 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add core grep/search module using ripgrep crates

- **Files**: `ito-rs/crates/ito-core/src/`
- **Dependencies**: None
- **Action**:
  - Add a `ito_core::grep` (or `ito_core::search`) module
  - Use `grep-regex` + `grep-searcher` to search files on disk
  - Return structured match results for CLI formatting
  - Implement an overall match limit
- **Verify**: `cd ito-rs && cargo test -p ito-core grep`
- **Done When**:
  - Core exposes a tested API that searches a list of file paths and returns matches
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 1.2: Add CLI command parsing and output formatting for ito grep

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`, `ito-rs/crates/ito-cli/src/commands/`
- **Dependencies**: Task 1.1
- **Action**:
  - Add `ito grep` subcommand
  - Support targets:
    - `ito grep <change-id> <regex>`
    - `ito grep --module <module-id> <regex>`
    - `ito grep --all <regex>`
  - Add `--limit <n>`
  - Print matches as `<path>:<line>:` lines
- **Verify**: `cd ito-rs && cargo test -p ito-cli`
- **Done When**:
  - CLI produces stable, line-oriented output suitable for piping to bash tools
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement backend-mode cache materialization for grep scopes

- **Files**: `ito-rs/crates/ito-cli/src/`, `ito-rs/crates/ito-core/src/backend_client.rs` (if shared helpers)
- **Dependencies**: None
- **Action**:
  - Add a cache directory (XDG-aware)
  - For a requested scope (change/module/all), ensure relevant artifacts are present in cache
  - Use conditional requests (`ETag` / `If-None-Match`) to avoid downloading unchanged artifacts
- **Verify**: `cd ito-rs && cargo test -p ito-cli`
- **Done When**:
  - Re-running `ito grep` does not re-download unchanged artifacts
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 2.2: Add tests for grep across change/module/all

- **Files**: `ito-rs/crates/ito-core/tests/`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**:
  - Add unit tests for core grep matching and limiting
  - Add CLI tests for target parsing and output limiting
- **Verify**: `cd ito-rs && cargo test`
- **Done When**:
  - Tests cover change, module, and all-project scopes
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
