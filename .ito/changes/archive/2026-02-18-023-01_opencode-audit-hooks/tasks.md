# Tasks for: 023-01_opencode-audit-hooks

## Execution Notes

- **Tool**: OpenCode + Rust (installer/tests)
- **Mode**: Sequential
- **Tracking**: Use `ito tasks` CLI once tasks are initialized

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define OpenCode audit hook assets

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/`, `ito-rs/crates/ito-templates/assets/adapters/`
- **Dependencies**: None
- **Action**: Specify the plugin file(s) to install under `.opencode/plugins/` and any minimal configuration needed for discovery.
- **Verify**: `make test -p ito-templates` (or workspace tests)
- **Done When**: Template assets exist in `ito-templates` and are wired for installation.
- **Updated At**: 2026-02-17
- **Status**: [x] complete

### Task 1.2: Implement OpenCode plugin behavior

- **Files**: Plugin source under `ito-rs/crates/ito-templates/assets/default/project/.opencode/plugins/`
- **Dependencies**: Task 1.1
- **Action**: Implement `tool.execute.before` audit callout with TTL caching and audit status injection.
- **Verify**: `bun test` (if added) and/or lightweight node/bun execution smoke test
- **Done When**: Plugin runs audits pre-tool and blocks on hard validation failures.
- **Updated At**: 2026-02-17
- **Status**: [x] complete

### Task 1.3: Installer tests for init/update

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-cli/tests/update_smoke.rs` (or new tests)
- **Dependencies**: Task 1.1
- **Action**: Add tests verifying `ito init` installs the plugin and `ito update` refreshes it without clobbering user-owned config.
- **Verify**: `make test`
- **Done When**: Tests cover init/update behavior for OpenCode plugin assets.
- **Updated At**: 2026-02-17
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Validation and documentation touch-ups

- **Files**: `docs/` or relevant harness docs (if any)
- **Dependencies**: None
- **Action**: Add a short note describing the audit hook behavior and how to disable it (if supported).
- **Verify**: `ito validate 023-01_opencode-audit-hooks --strict`
- **Done When**: Change validates and docs are updated.
- **Updated At**: 2026-02-17
- **Status**: [x] complete
