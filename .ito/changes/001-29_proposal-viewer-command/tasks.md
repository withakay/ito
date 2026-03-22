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

- **Files**: `ito-rs/crates/ito-core/src/viewer/collector.rs`
- **Dependencies**: None
- **Action**: Implement a function `collect_proposal_artifacts(change_id, ito_root) -> Result<String>` that reads `proposal.md`, all `specs/**/*.md` delta files, and `tasks.md` (if present) for a given change ID, and concatenates them into a single document with clear section separators
- **Verify**: Unit test with a fixture change directory confirms correct concatenation order and separator format
- **Done When**: Function returns expected document string for a change with all three artifact types; error returned for unknown change ID
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Viewer backend trait and implementations

- **Depends On**: Wave 1

### Task 2.1: Define ViewerBackend trait

- **Files**: `ito-rs/crates/ito-core/src/viewer/mod.rs`
- **Dependencies**: None
- **Action**: Define a `ViewerBackend` trait with methods: `name() -> &str`, `description() -> &str`, `is_available() -> bool`, and `open(content: &str) -> Result<()>`
- **Verify**: `cargo check -p ito-core` passes; trait is pub
- **Done When**: Trait compiles; documented with rustdoc
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.2: Implement TmuxNvimViewer

- **Files**: `ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs`
- **Dependencies**: Task 2.1
- **Action**: Implement `ViewerBackend` for `TmuxNvimViewer`: writes content to a tempfile, checks `$TMUX` env var and `nvim` on PATH, runs `tmux display-popup -E nvim <tmpfile>` in read-only mode
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
- **Action**: Implement `ViewerRegistry` holding a list of `Box<dyn ViewerBackend>`; exposes `available_viewers()` (only those where `is_available()` is true) and `find_by_name(name: &str)`
- **Verify**: Unit test confirms only available viewers returned; unknown name returns None
- **Done When**: Registry compiles; tests pass
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: CLI command wiring

- **Depends On**: Wave 2

### Task 3.1: Add `ito view proposal` subcommand

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/view_proposal.rs`
- **Dependencies**: None
- **Action**: Add `view proposal <change-id>` subcommand to the CLI; add optional `--viewer <name>` flag; wire to handler
- **Verify**: `cargo build -p ito-cli`; `./target/debug/ito view proposal --help` shows the subcommand and flag
- **Done When**: Subcommand visible in help; build passes
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 3.2: Implement command handler with interactive prompt

- **Files**: `ito-rs/crates/ito-cli/src/commands/view_proposal.rs`
- **Dependencies**: Task 3.1
- **Action**: Implement handler: collect artifacts; if `--viewer` flag provided look up backend by name (error if not found/unavailable); otherwise present interactive prompt listing available viewers; call `backend.open(content)`
- **Verify**: Smoke test `ito view proposal <id> --viewer bat` renders output; unknown change ID shows error; unknown `--viewer` shows error with install hint
- **Done When**: All error paths tested; interactive prompt shows only installed viewers
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Integration tests and validation

- **Depends On**: Wave 3

### Task 4.1: Write integration test for command

- **Files**: `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Write an integration test that sets up a temporary Ito project with a known change directory, runs `ito view proposal <id> --viewer bat` (or mocks the viewer call), and asserts correct exit code; cover error cases (missing change, missing viewer)
- **Verify**: `cargo test -p ito-cli view_proposal` passes
- **Done When**: Integration test green; error cases covered (missing change, missing viewer)
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 4.2: Validate with ito validate

- **Files**: N/A
- **Dependencies**: Task 4.1
- **Action**: Run `ito validate 001-29 --strict`
- **Verify**: Exits 0 with no errors
- **Done When**: Validation passes
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
<!-- ITO:END -->
