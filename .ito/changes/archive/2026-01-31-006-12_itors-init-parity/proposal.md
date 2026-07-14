## Why

`itors` is intended to be a port of the TypeScript CLI, but `itors init` currently behaves differently (non-interactive, installs all supported tools by default). This divergence breaks user expectations, complicates documentation/support, and undermines the Rust parity goal.

## What Changes

- Make `itors init` follow the same interaction model as the TypeScript CLI: interactive tool selection when run interactively, and non-interactive configuration via `--tools`.
- Align `--tools` parsing and validation behavior (including error cases) with the TypeScript CLI.
- Extend the Rust parity harness to include automated parity coverage for `init` (non-interactive and interactive PTY flows).
- Use well-maintained interactive CLI crates for the Rust implementation (recommended: `dialoguer` for prompts, `crossterm` for terminal handling, `indicatif` for progress/spinners; `ratatui` only if a full-screen TUI becomes necessary).

## Capabilities

### New Capabilities

- `rust-cli-init-parity`: `itors init` matches TypeScript `ito init` behavior for tool selection, non-interactive flags, and extend/fresh init flows.

### Modified Capabilities

- `rust-parity-harness`: add parity tests and fixtures specifically covering `init` behavior.

## Impact

- Affected code: `ito-rs/crates/ito-cli` (CLI flags + UX), `ito-rs/crates/ito-core` (init orchestration and installers, if shared), `ito-rs` parity harness and fixtures.
- User-visible changes: `itors init` becomes interactive in TTY contexts by default; users can keep non-interactive behavior with `--tools`.
- Risk: interactive prompt behavior must remain stable across platforms; mitigated via PTY-based parity tests.
- Dependencies: add Rust crates for prompts/terminal UX (see above); prefer cross-platform support (especially Windows).
