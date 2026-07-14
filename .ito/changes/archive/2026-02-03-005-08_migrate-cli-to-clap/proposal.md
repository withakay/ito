## Why

The ito CLI is 100% hand-rolled with zero clap usage, resulting in ~900 lines of unnecessary boilerplate: ~400 lines of manual argument parsing and ~500 lines of hardcoded help text constants. This creates maintenance burden, inconsistent UX, and prevents leveraging free features like shell completions and man pages.

## What Changes

- Replace manual argument parsing (`std::env::args()` + string matching) with clap derive macros
- Delete all hardcoded help text constants (`HELP`, `TASKS_HELP`, etc.) in favor of auto-generated help from doc comments
- Add shell completion generation via `clap_complete` for bash/zsh/fish/powershell
- Add type-safe argument parsing with `ValueEnum` and custom value parsers
- Implement consistent styled/colored help output via clap's `Styles` API

## Capabilities

### New Capabilities

- `cli-shell-completions`: Generate shell completion scripts for bash, zsh, fish, and powershell via `ito completions <shell>`

### Modified Capabilities

- `cli-core`: Replace hand-rolled parsing infrastructure with clap derive API while preserving all existing command names, flags, and behaviors

## Impact

- **Code**: `ito-cli` crate - major refactor of `app/mod.rs`, `app/help.rs`, and all command handlers
- **Dependencies**: Add `clap` (with derive feature), `clap_complete`
- **Tests**: Add snapshot tests for CLI output; existing behavior must be preserved
- **User Experience**: Tab completion, consistent help formatting, colored output
- **Build**: ~2-5s additional compile time (acceptable tradeoff)
