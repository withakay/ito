<!-- ITO:START -->
## Why

CLI harnesses (codex, claude, copilot, opencode) can crash with signal-based exit codes (e.g. 128, 137) that are transient process failures — the harness binary died, not the agent's work. Currently these count against the error threshold or immediately abort the loop with `--exit-on-error`, wasting retry budget on failures the agent can't fix and potentially killing long-running autonomous sessions.

Separately, all four CLI harness implementations were near-identical copies differing only in binary name, subcommand, and flag names — violating DRY and making it easy for them to drift out of sync.

## What Changes

- Classify signal-based exit codes (128–143) as "retriable" and retry them automatically without counting against the error threshold
- Cap consecutive retriable retries at 3 to prevent infinite crash loops
- Introduce a `CliHarness` trait that captures the CLI-specific contract (binary name, arg building) with a blanket `Harness` impl for process spawning, streaming, and timeout monitoring
- Refactor `ClaudeCodeHarness`, `CodexHarness`, `GitHubCopilotHarness`, and `OpencodeHarness` to implement `CliHarness` instead of duplicating `Harness` trait logic

## Capabilities

### New Capabilities

- `retriable-harness-crashes`: Automatic retry of transient harness process crashes in the Ralph loop

### Modified Capabilities

- `rust-ralph`: Error handling behavior changes for signal-based exit codes

## Impact

- `ito-core::harness::types` — new `is_retriable()` method on `HarnessRunResult`, `MAX_RETRIABLE_RETRIES` constant
- `ito-core::harness::streaming_cli` — new public `CliHarness` trait with blanket `Harness` impl
- `ito-core::harness::{claude_code,codex,github_copilot,opencode}` — simplified to `CliHarness` impls (~45 lines each, down from ~85-94)
- `ito-core::ralph::runner` — retriable exit code handling added before error threshold logic
- No CLI surface changes, no breaking API changes
<!-- ITO:END -->
