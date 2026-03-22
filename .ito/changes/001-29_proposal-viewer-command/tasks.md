<!-- ITO:START -->
# Tasks for: 001-29_proposal-viewer-command

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-29_proposal-viewer-command
ito tasks next 001-29_proposal-viewer-command
ito tasks start 001-29_proposal-viewer-command 1.1
ito tasks complete 001-29_proposal-viewer-command 1.1
```

______________________________________________________________________

## Wave 1: Core domain — artifact collection

- **Depends On**: None

### Task 1.1: Implement artifact collector

- **Files**: `ito-rs/crates/ito-core/src/` (new module, e.g., `viewer/collector.rs`)
- **Dependencies**: None
- **Action**: Implement a function `collect_proposal_artifacts(change_id, ito_root) -> Result<String>` that reads `proposal.md`, all `specs/**/*.md` delta files, and `tasks.md` (if present) for a given change ID, and concatenates them into a single document with clear section separators (e.g., `---\n# specs/foo/spec.md\n`)
- **Verify**: Unit test with a fixture change directory confirms correct concatenation order and separator format
- **Done When**: Function returns expected document string for a change with all three artifact types; error returned for unknown change ID
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Viewer backend trait and implementations

- **Depends On**: Wave 1

### Task 2.1: Define ViewerBackend trait

- **Files**: `ito-rs/crates/ito-core/src/viewer/mod.rs` (or similar)
- **Dependencies**: None
- **Action**: Define a `ViewerBackend` trait with methods: `name() -> &str`, `description() -> &str`, `is_available() -> bool` (checks tool on PATH), and `open(content: &str) -> Result<()>`
- **Verify**: `cargo check -p ito-core` passes; trait is pub
- **Done When**: Trait compiles; documented with rustdoc
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.2: Implement TmuxNvimViewer

- **Files**: `ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs`
- **Dependencies**: Task 2.1
- **Action**: Implement `ViewerBackend` for a `TmuxNvimViewer` struct: writes content to a tempfile, checks `$TMUX` env var and `nvim` on PATH, runs `tmux display-popup -E nvim <tmpfile>` in read-only mode (`-R`)
- **Verify**: Unit tests: `is_available()` returns false when nvim not on PATH; `open()` errors gracefully when `$TMUX` unset
- **Done When**: All unit tests pass; `cargo test -p ito-core viewer::tmux_nvim` green
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.3: Implement BatViewer

- **Files**: `ito-rs/crates/ito-core/src/viewer/bat.rs`
- **Dependencies**: Task 2.1
- **Action**: Implement `ViewerBackend` for `BatViewer`: checks `bat` on PATH; pipes content to `bat --language=markdown --paging=always`
- **Verify**: Unit tests for `is_available()` and graceful error when bat missing
- **Done When**: Unit tests pass
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.4: Implement GlowViewer

- **Files**: `ito-rs/crates/ito-core/src/viewer/glow.rs`
- **Dependencies**: Task 2.1
- **Action**: Implement `ViewerBackend` for `GlowViewer`: checks `glow` on PATH; pipes content to `glow -`
- **Verify**: Unit tests for `is_available()` and graceful error when glow missing
- **Done When**: Unit tests pass
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.5: Implement viewer registry

- **Files**: `ito-rs/crates/ito-core/src/viewer/registry.rs`
- **Dependencies**: Task 2.2, Task 2.3, Task 2.4
- **Action**: Implement `ViewerRegistry` that holds a list of `Box<dyn ViewerBackend>` instances; exposes `available_viewers() -> Vec<&dyn ViewerBackend>` (only those where `is_available()` returns true) and `find_by_name(name: &str) -> Option<&dyn ViewerBackend>`
- **Verify**: Unit test confirms only available viewers are returned; unknown name returns None
- **Done When**: Registry compiles; tests pass
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: CLI command wiring

- **Depends On**: Wave 2

### Task 3.1: Add `ito view proposal` subcommand

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/` (new handler)
- **Dependencies**: None
- **Action**: Add `view proposal <change-id>` subcommand to the CLI (under existing `view` / alongside `dashboard`); add optional `--viewer <name>` flag; wire to a handler that calls the collector and viewer dispatch
- **Verify**: `cargo build -p ito-cli`; `./target/debug/ito view proposal --help` shows the subcommand and flag
- **Done When**: Subcommand visible in help; build passes
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 3.2: Implement command handler with interactive prompt

- **Files**: `ito-rs/crates/ito-cli/src/commands/view_proposal.rs`
- **Dependencies**: Task 3.1
- **Action**: Implement the handler: collect artifacts via `collect_proposal_artifacts`; if `--viewer` flag provided, look up backend by name (error if not found or unavailable); otherwise present an interactive prompt listing available viewers (using `dialoguer` or similar); call `backend.open(content)`
- **Verify**: Manual smoke test: `ito view proposal <known-change-id> --viewer bat` renders output; unknown change ID shows error; unknown `--viewer` shows error with install hint
- **Done When**: All error paths tested; interactive prompt shows only installed viewers
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Integration tests and validation

- **Depends On**: Wave 3

### Task 4.1: Write integration test for command

- **Files**: `ito-rs/crates/ito-cli/tests/` or integration test module
- **Dependencies**: None
- **Action**: Write an integration test that sets up a temporary Ito project with a known change directory (proposal.md + one spec + tasks.md), runs `ito view proposal <id> --viewer bat` (or mocks the viewer call), and asserts the correct exit code and output
- **Verify**: `cargo test -p ito-cli view_proposal` passes
- **Done When**: Integration test green; error cases covered (missing change, missing viewer)
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 4.2: Validate with ito validate

- **Files**: N/A
- **Dependencies**: Task 4.1
- **Action**: Run `ito validate 001-29_proposal-viewer-command --strict` to confirm the change package is complete
- **Verify**: `ito validate 001-29_proposal-viewer-command --strict` exits 0 with no errors
- **Done When**: Validation passes
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
<!-- ITO:END -->
