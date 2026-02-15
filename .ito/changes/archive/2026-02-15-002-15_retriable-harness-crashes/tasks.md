# Tasks

- [x] Add `is_retriable()` method and `MAX_RETRIABLE_RETRIES` constant to `HarnessRunResult` in `types.rs`
- [x] Add retriable exit code handling to Ralph runner before error threshold logic
- [x] Introduce `CliHarness` trait in `streaming_cli.rs` with blanket `Harness` impl
- [x] Refactor `ClaudeCodeHarness` to implement `CliHarness`
- [x] Refactor `CodexHarness` to implement `CliHarness`
- [x] Refactor `GitHubCopilotHarness` to implement `CliHarness`
- [x] Refactor `OpencodeHarness` to implement `CliHarness`
- [x] Add test: retriable exit code retries without counting against threshold
- [x] Add test: retriable exit code retries even with `--exit-on-error`
- [x] Add test: gives up after max consecutive retriable retries
- [x] Add test: successful iteration resets retriable counter
- [x] Add test: non-retriable exits still count against threshold
- [x] Verify all existing tests pass with refactored harness code
