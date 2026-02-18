# Tasks for: 023-02_claude-audit-hooks

## Execution Notes

- **Tool**: Claude Code + Rust (installer/tests)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Specify Claude hook files to install

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.claude/`, `ito-rs/crates/ito-templates/assets/adapters/`
- **Dependencies**: None
- **Action**: Decide exact hook config location/format and script path (e.g., `.claude/settings.json` + `.claude/hooks/ito-audit.sh`).
- **Verify**: `make test -p ito-templates` (or workspace tests)
- **Done When**: Templates include the hook configuration and script.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

### Task 1.2: Implement hook script behavior

- **Files**: Hook script template under `ito-templates` assets
- **Dependencies**: Task 1.1
- **Action**: Implement stdin JSON parsing and Ito CLI delegation; return structured output for warnings; block on hard validation failures.
- **Verify**: Add a small script-level test harness (or installer-level integration test) to validate JSON output and exit codes.
- **Done When**: Script behavior is deterministic and minimal.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

### Task 1.3: Installer tests for init/update

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-cli/tests/update_smoke.rs` (or new tests)
- **Dependencies**: Task 1.1
- **Action**: Add tests verifying files are installed and updated without clobbering user-owned local config.
- **Verify**: `make test`
- **Done When**: Tests cover init/update behavior for Claude hook assets.
- **Updated At**: 2026-02-18
- **Status**: [x] complete
