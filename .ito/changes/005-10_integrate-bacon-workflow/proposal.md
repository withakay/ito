# Change: Integrate Bacon into Development Workflow

## Why

[Bacon](https://github.com/Canop/bacon) is a background Rust code checker that provides continuous feedback on compilation errors, warnings, and test failures. Integrating it into Ito's development workflow benefits both:

- **Humans**: Real-time feedback in a dedicated terminal without manually running `cargo check`
- **Agents**: Structured error output that can be parsed for automated fixes, plus a persistent error state that survives context switches

Currently developers must manually run `cargo check`, `cargo test`, or `cargo clippy` after each change. Bacon runs continuously in the background and shows results instantly.

## What Changes

- Add `bacon.toml` configuration with project-specific jobs
- Add bacon installation to development setup docs
- Create custom bacon jobs for common workflows (check, test, clippy, coverage)
- Optionally: Add bacon export format for agent consumption
- Update AGENTS.md with bacon usage guidance for AI assistants

## Capabilities

### New Capabilities

- `bacon-config`: Project-specific bacon.toml with jobs for check, clippy, test, and coverage workflows
- `bacon-agent-integration`: Documentation and potentially export formats enabling agents to consume bacon output for automated error fixing

### Modified Capabilities

<!-- None -->

## Impact

- **New file**: `bacon.toml` in project root (or `ito-rs/bacon.toml`)
- **Documentation**: Updates to README, AGENTS.md, and/or CONTRIBUTING.md
- **Developer experience**: Faster feedback loop for Rust development
- **Agent workflows**: Structured error data for Ralph and other agent loops
