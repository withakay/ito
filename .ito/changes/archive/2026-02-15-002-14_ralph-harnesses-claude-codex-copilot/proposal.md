<!-- ITO:START -->
## Why

`ito ralph` can currently run only the `opencode` and `stub` harnesses. Ito already installs project scaffolding for Claude Code (`.claude/`), OpenAI Codex (`.codex/`), and GitHub Copilot (`.github/`), but Ralph cannot drive those CLIs today.

Adding first-class harnesses for Claude, Codex, and Copilot lets developers use the same Ralph loop across their preferred agent runtimes, without changing Ito workflows.

## What Changes

- Add three new Ralph harness integrations: `claude`, `codex`, and `github-copilot` (alias: `copilot`).
- Wire `ito ralph --harness <name>` to the new harnesses and keep `opencode`/`stub` working.
- Pass `--model` through to the selected harness when supported.
- Map `--allow-all` (yolo) to each harness's equivalent permission-bypass/auto-approval mode.
- Count git working-tree changes after an iteration for harnesses that can edit files (not just OpenCode).
- Keep tests offline by continuing to use the `stub` harness for all test coverage.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `cli-ralph`: accept and document additional `--harness` values.
- `rust-ralph`: provide harness implementations for Claude Code, Codex, and GitHub Copilot.

## Impact

- **Code**: `ito-rs/crates/ito-core/src/harness/`, `ito-rs/crates/ito-cli/src/commands/ralph.rs`, and `ito-rs/crates/ito-core/src/ralph/runner.rs`.
- **External dependencies**: requires the relevant CLI (`claude`, `codex`, `copilot`) to be installed and authenticated for real runs; tests remain network-free.
- **Security**: `--allow-all` becomes meaningful across more harnesses; default behavior should remain non-destructive unless explicitly enabled.
<!-- ITO:END -->
