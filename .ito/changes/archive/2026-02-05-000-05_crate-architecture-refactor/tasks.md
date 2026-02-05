# Tasks for: 000-05_crate-architecture-refactor

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential waves, parallel tasks within waves
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use the tasks CLI to drive status updates and pick work

```bash
ito tasks status 000-05_crate-architecture-refactor
ito tasks next 000-05_crate-architecture-refactor
ito tasks start 000-05_crate-architecture-refactor 1.1
ito tasks complete 000-05_crate-architecture-refactor 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Scaffold ito-common crate

- **Files**: ito-rs/crates/ito-common/Cargo.toml, ito-rs/crates/ito-common/src/lib.rs, ito-rs/Cargo.toml
- **Dependencies**: None
- **Action**:
  Create new `ito-common` crate with Cargo.toml (no ito-* dependencies, only external crates like miette, thiserror). Add to workspace members. Create empty lib.rs with module declarations.
- **Verify**: `cargo check -p ito-common`
- **Done When**: Crate compiles with no ito-* dependencies
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.2: Move id module to ito-common

- **Files**: ito-rs/crates/ito-common/src/id/, ito-rs/crates/ito-core/src/id/
- **Dependencies**: Task 1.1
- **Action**:
  Copy `ito-core/src/id/` to `ito-common/src/id/`. Update imports. Export from ito-common lib.rs. Keep re-export in ito-core temporarily for compatibility.
- **Verify**: `cargo test -p ito-common`
- **Done When**: id module tests pass in ito-common
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.3: Move paths module to ito-common

- **Files**: ito-rs/crates/ito-common/src/paths.rs, ito-rs/crates/ito-core/src/paths.rs
- **Dependencies**: Task 1.1
- **Action**:
  Copy `ito-core/src/paths.rs` to `ito-common/src/paths.rs`. Update imports. Export from lib.rs.
- **Verify**: `cargo test -p ito-common`
- **Done When**: paths module compiles in ito-common
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.4: Move io module to ito-common

- **Files**: ito-rs/crates/ito-common/src/io.rs, ito-rs/crates/ito-core/src/io.rs
- **Dependencies**: Task 1.1
- **Action**:
  Copy `ito-core/src/io.rs` to `ito-common/src/io.rs`. Update imports. Export from lib.rs.
- **Verify**: `cargo test -p ito-common`
- **Done When**: io module compiles in ito-common
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.5: Move match module to ito-common

- **Files**: ito-rs/crates/ito-common/src/match_.rs, ito-rs/crates/ito-core/src/match_.rs
- **Dependencies**: Task 1.1
- **Action**:
  Copy `ito-core/src/match_.rs` to `ito-common/src/match_.rs`. Update imports. Export from lib.rs.
- **Verify**: `cargo test -p ito-common`
- **Done When**: match_ module compiles in ito-common
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.6: Add FileSystem trait to ito-common

- **Files**: ito-rs/crates/ito-common/src/fs.rs
- **Dependencies**: Task 1.1
- **Action**:
  Create `fs.rs` with `FileSystem` trait (Send + Sync, methods: read_to_string, write, exists, create_dir_all, read_dir, remove_file, remove_dir_all, is_dir, is_file). Create `StdFs` struct implementing the trait via std::fs. Export from lib.rs.
- **Verify**: `cargo test -p ito-common`
- **Done When**: FileSystem trait and StdFs compile, StdFs is zero-sized
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Scaffold ito-config crate

- **Files**: ito-rs/crates/ito-config/Cargo.toml, ito-rs/crates/ito-config/src/lib.rs, ito-rs/Cargo.toml
- **Dependencies**: None
- **Action**:
  Create new `ito-config` crate with Cargo.toml (depends on ito-common only). Add to workspace members. Create empty lib.rs.
- **Verify**: `cargo check -p ito-config`
- **Done When**: Crate compiles with only ito-common dependency
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 2.2: Move config module to ito-config

- **Files**: ito-rs/crates/ito-config/src/config/, ito-rs/crates/ito-core/src/config/
- **Dependencies**: Task 2.1
- **Action**:
  Copy `ito-core/src/config/` to `ito-config/src/`. Update internal imports to use ito_common. Export from lib.rs.
- **Verify**: `cargo check -p ito-config`
- **Done When**: config module compiles in ito-config
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 2.3: Move ito_dir module to ito-config

- **Files**: ito-rs/crates/ito-config/src/ito_dir/, ito-rs/crates/ito-core/src/ito_dir/
- **Dependencies**: Task 2.1
- **Action**:
  Copy `ito-core/src/ito_dir/` to `ito-config/src/ito_dir/`. Update imports. Export from lib.rs.
- **Verify**: `cargo check -p ito-config`
- **Done When**: ito_dir module compiles in ito-config
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 2.4: Move output module to ito-config

- **Files**: ito-rs/crates/ito-config/src/output/, ito-rs/crates/ito-core/src/output/
- **Dependencies**: Task 2.1
- **Action**:
  Copy `ito-core/src/output/` to `ito-config/src/output/`. Update imports. Export from lib.rs.
- **Verify**: `cargo check -p ito-config`
- **Done When**: output module compiles in ito-config
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 2.5: Create ItoContext struct

- **Files**: ito-rs/crates/ito-config/src/context.rs
- **Dependencies**: Task 2.2, Task 2.3
- **Action**:
  Create `ItoContext` struct with fields: config_dir (Option<PathBuf>), project_root (PathBuf), ito_path (Option<PathBuf>), config (ResolvedConfig). Add `ItoContext::resolve<F: FileSystem>(fs: &F, cwd: &Path)` method.
- **Verify**: `cargo test -p ito-config`
- **Done When**: ItoContext compiles and has resolve method
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 1

### Task 3.1: Add ito-common dependency to ito-domain

- **Files**: ito-rs/crates/ito-domain/Cargo.toml
- **Dependencies**: None
- **Action**:
  Add `ito-common` to ito-domain dependencies.
- **Verify**: `cargo check -p ito-domain`
- **Done When**: ito-domain compiles with new dependency
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 3.2: Move discovery module to ito-domain

- **Files**: ito-rs/crates/ito-domain/src/discovery.rs, ito-rs/crates/ito-core/src/discovery.rs
- **Dependencies**: Task 3.1
- **Action**:
  Copy `ito-core/src/discovery.rs` to `ito-domain/src/discovery.rs`. Update imports to use ito_common for paths, io. Update function signatures to accept `<F: FileSystem>` where needed. Export from lib.rs.
- **Verify**: `cargo test -p ito-domain`
- **Done When**: discovery module compiles and tests pass in ito-domain
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 3.3: Refactor ito-logging to be a leaf crate

- **Files**: ito-rs/crates/ito-logging/Cargo.toml, ito-rs/crates/ito-logging/src/lib.rs
- **Dependencies**: None
- **Action**:
  Remove ito-core dependency from Cargo.toml. Change `Logger::new()` signature from `(ctx: &ConfigContext, ...)` to `(config_dir: Option<PathBuf>, ...)`. Update all internal uses of ConfigContext.
- **Verify**: `cargo check -p ito-logging`
- **Done When**: ito-logging has no ito-* dependencies
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 2, Wave 3

### Task 4.1: Add ito-config and ito-common dependencies to ito-core

- **Files**: ito-rs/crates/ito-core/Cargo.toml
- **Dependencies**: None
- **Action**:
  Add `ito-config` and `ito-common` to ito-core dependencies. Keep ito-domain, ito-templates, ito-harness.
- **Verify**: `cargo check -p ito-core`
- **Done When**: ito-core compiles with new dependencies
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.2: Inline ito-fs into ito-core

- **Files**: ito-rs/crates/ito-core/src/installers/markers.rs, ito-rs/crates/ito-fs/
- **Dependencies**: Task 4.1
- **Action**:
  Copy `update_file_with_markers` function from ito-fs to new file `ito-core/src/installers/markers.rs`. Update imports in installers/mod.rs. Remove ito-fs from ito-core dependencies.
- **Verify**: `cargo test -p ito-core -- markers`
- **Done When**: Marker functionality works without ito-fs crate
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.3: Remove moved modules from ito-core

- **Files**: ito-rs/crates/ito-core/src/lib.rs, ito-rs/crates/ito-core/src/
- **Dependencies**: Task 4.1, Task 4.2
- **Action**:
  Delete old module files from ito-core: id/, paths.rs, io.rs, match_.rs, config/, ito_dir/, output/, discovery.rs. Update lib.rs to remove these module declarations. Add re-exports from ito-common and ito-config for backward compatibility (temporary).
- **Verify**: `cargo check -p ito-core`
- **Done When**: ito-core no longer contains moved modules
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Update ito-cli imports

- **Files**: ito-rs/crates/ito-cli/src/**/*.rs, ito-rs/crates/ito-cli/Cargo.toml
- **Dependencies**: None
- **Action**:
  Add ito-config and ito-common to CLI dependencies. Update all imports: `ito_core::config` -> `ito_config`, `ito_core::io` -> `ito_common::io`, etc. Update Logger::new() calls to pass config_dir explicitly.
- **Verify**: `cargo check -p ito-cli`
- **Done When**: CLI compiles with new import paths
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.2: Update ito-web imports

- **Files**: ito-rs/crates/ito-web/src/**/*.rs, ito-rs/crates/ito-web/Cargo.toml
- **Dependencies**: None
- **Action**:
  Add ito-config dependency if needed. Update imports for any config or utility usage.
- **Verify**: `cargo check -p ito-web`
- **Done When**: ito-web compiles with new import paths
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.3: Update any remaining crates

- **Files**: ito-rs/crates/*/Cargo.toml, ito-rs/crates/*/src/**/*.rs
- **Dependencies**: Task 5.1, Task 5.2
- **Action**:
  Grep for any remaining uses of old import paths (ito_core::config, ito_core::io, etc.). Update all found occurrences.
- **Verify**: `cargo check --workspace`
- **Done When**: All crates compile
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 6

- **Depends On**: Wave 5

### Task 6.1: Remove ito-fs from workspace

- **Files**: ito-rs/Cargo.toml, ito-rs/crates/ito-fs/
- **Dependencies**: None
- **Action**:
  Remove ito-fs from workspace members in root Cargo.toml. Delete ito-rs/crates/ito-fs/ directory.
- **Verify**: `cargo check --workspace`
- **Done When**: Workspace compiles without ito-fs
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.2: Remove temporary re-exports from ito-core

- **Files**: ito-rs/crates/ito-core/src/lib.rs
- **Dependencies**: Task 6.1
- **Action**:
  Remove any temporary re-exports added for backward compatibility. ito-core should only export business logic modules.
- **Verify**: `cargo check --workspace`
- **Done When**: ito-core lib.rs only exports business logic
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.3: Run full test suite

- **Files**: N/A
- **Dependencies**: Task 6.2
- **Action**:
  Run `make check` to verify all tests pass, lints pass, and build succeeds.
- **Verify**: `make check`
- **Done When**: All tests pass, no warnings
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 7

- **Depends On**: Wave 6

### Task 7.1: Review architecture

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: ito-rs/crates/*/Cargo.toml
- **Dependencies**: None
- **Action**:
  Review the final crate dependency graph. Verify no circular dependencies. Confirm layering matches design (common -> config -> domain -> core -> cli).
- **Done When**: Human confirms architecture is correct
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
