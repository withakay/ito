# Tasks: Consolidate Workspace Crates

## Phase 1: Merge `ito-schemas` → `ito-domain`

- [x] Create `ito-domain/src/schemas/` module directory
  - [x] Create `ito-domain/src/schemas/mod.rs` re-exporting all submodules
  - [x] Move `ito-schemas/src/workflow.rs` → `ito-domain/src/schemas/workflow.rs`
  - [x] Move `ito-schemas/src/workflow_plan.rs` → `ito-domain/src/schemas/workflow_plan.rs`
  - [x] Move `ito-schemas/src/workflow_state.rs` → `ito-domain/src/schemas/workflow_state.rs`
- [x] Update `ito-domain/src/lib.rs` to declare `pub mod schemas;`
- [x] Update `ito-domain/Cargo.toml`:
  - [x] Add `serde_yaml` to `[dependencies]` (from ito-schemas)
  - [x] Remove `ito-schemas` from `[dependencies]`
- [x] Update all `ito-core` imports: `ito_schemas::*` → `ito_domain::schemas::*`
  - [x] Grep for `use ito_schemas` in `ito-core/src/**/*.rs` and update each site
  - [x] Remove `ito-schemas` from `ito-core/Cargo.toml` `[dependencies]`
- [x] Add re-export in `ito-core/src/lib.rs`: `pub use ito_domain::schemas;`
- [x] Update all `ito-cli` imports: `ito_schemas::*` → `ito_core::schemas::*`
  - [x] Grep for `use ito_schemas` in `ito-cli/src/**/*.rs` and update each site
  - [x] Remove `ito-schemas` from `ito-cli/Cargo.toml` `[dependencies]`
- [x] Remove `ito-schemas` crate from workspace:
  - [x] Remove `"crates/ito-schemas"` from `ito-rs/Cargo.toml` `[workspace.members]`
  - [x] Delete `ito-rs/crates/ito-schemas/` directory
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 2: Merge `ito-harness` → `ito-core`

- [x] Create `ito-core/src/harness/` module directory
  - [x] Create `ito-core/src/harness/mod.rs` re-exporting all submodules
  - [x] Move `ito-harness/src/types.rs` → `ito-core/src/harness/types.rs`
  - [x] Move `ito-harness/src/opencode.rs` → `ito-core/src/harness/opencode.rs`
  - [x] Move `ito-harness/src/stub.rs` → `ito-core/src/harness/stub.rs`
- [x] Update `ito-core/src/lib.rs` to declare `pub mod harness;`
- [x] Update internal `ito-core` imports: `ito_harness::*` → `crate::harness::*`
  - [x] Grep for `use ito_harness` in `ito-core/src/**/*.rs` and update each site
  - [x] Remove `ito-harness` from `ito-core/Cargo.toml` `[dependencies]`
- [x] Update all `ito-cli` imports: `ito_harness::*` → `ito_core::harness::*`
  - [x] Grep for `use ito_harness` in `ito-cli/src/**/*.rs` and update each site
  - [x] Remove `ito-harness` from `ito-cli/Cargo.toml` `[dependencies]`
- [x] Remove `ito-harness` crate from workspace:
  - [x] Remove `"crates/ito-harness"` from `ito-rs/Cargo.toml` `[workspace.members]`
  - [x] Delete `ito-rs/crates/ito-harness/` directory
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 3: CLI bypass elimination — new core functions

These functions push I/O operations out of the CLI adapter and into `ito-core`, where they belong. Each function encapsulates a read or read-mutate-write cycle that the CLI currently does by hand using `ito_common::io::*` and `ito_common::paths::*`.

- [x] Create `ito-core/src/state.rs` module with:
  - [x] `pub fn read_state(ito_path: &Path) -> CoreResult<String>` — reads `{ito_path}/planning/STATE.md`
  - [x] `pub enum StateAction { AddDecision { text: String }, AddBlocker { text: String }, AddNote { text: String }, SetFocus { text: String }, AddQuestion { text: String } }`
  - [x] `pub fn update_state(ito_path: &Path, action: StateAction) -> CoreResult<()>` — reads STATE.md, applies the mutation using existing `ito_domain::state::*` functions, writes back
  - [x] Unit test: `read_state` returns error for missing file
  - [x] Unit test: `update_state` with each `StateAction` variant produces expected markdown
- [x] Register `pub mod state;` in `ito-core/src/lib.rs`
- [x] Create `ito_core::tasks::read_tasks_markdown(ito_path: &Path, change_id: &str) -> CoreResult<String>`
  - [x] Add function to `ito-core/src/tasks.rs` — reads `{change_dir}/tasks.md` using `tasks_path()` from domain
  - [x] Unit test: returns contents for existing tasks.md
  - [x] Unit test: returns error for missing tasks.md
- [x] Create `ito_core::planning::read_planning_status(ito_path: &Path) -> CoreResult<String>` <!-- NOTE: lives in planning_init.rs, not planning.rs -->
  - [x] Add function to new `ito-core/src/planning.rs` module — reads `{ito_path}/planning/ROADMAP.md` <!-- NOTE: actually in planning_init.rs -->
  - [x] Register `pub mod planning;` in `ito-core/src/lib.rs` <!-- NOTE: registered as pub mod planning_init -->
  - [x] Unit test: returns contents for existing ROADMAP.md
  - [x] Unit test: returns error for missing ROADMAP.md
- [x] Create `ito_core::show::read_module_markdown(ito_path: &Path, module_id: &str) -> CoreResult<String>`
  - [x] Add function to `ito-core/src/show/mod.rs` — reads `{module_dir}/module.md`
  - [x] Unit test: returns contents for existing module.md
  - [x] Unit test: returns error for missing module.md
- [x] Create `ito_core::validate::validate_tasks_file(ito_path: &Path, change_id: &str) -> CoreResult<Vec<ValidationIssue>>`
  - [x] Add function to `ito-core/src/validate/mod.rs` — reads tasks.md, parses it, converts diagnostics to `ValidationIssue` items
  - [x] Unit test: returns empty vec for valid tasks file
  - [x] Unit test: returns diagnostics for malformed tasks file
- [x] Run `make check && make test` — verify new functions compile and tests pass

## Phase 4: CLI bypass elimination — re-exports and call-site updates

- [x] Add re-exports to `ito-core/src/lib.rs`:
  - [x] `pub use ito_common::match_::nearest_matches;`
  - [x] `pub use ito_common::id::parse_module_id;`
- [x] Update `cli/commands/state.rs`:
  - [x] Replace `ito_common::io::read_to_string(&state_path)` with `ito_core::state::read_state(ito_path)?`
  - [x] Replace read → mutate → write cycle with `ito_core::state::update_state(ito_path, action)?`
  - [x] Remove `use ito_common::io` from this file
- [x] Update `cli/commands/tasks.rs`:
  - [x] Replace `ito_common::io::read_to_string(&path)` calls (lines ~585, ~629) with `ito_core::tasks::read_tasks_markdown(ito_path, change_id)?`
  - [x] Replace `core_paths::change_dir(ito_path, &change_id).join("tasks.md")` with `ito_core::tasks::tasks_path(ito_path, &change_id)` (already re-exported from domain)
  - [x] Remove `use ito_common::paths` and `use ito_common::io` from this file
- [x] Update `cli/commands/plan.rs`:
  - [x] Replace `ito_common::io::read_to_string(&roadmap_path)` with `ito_core::planning::read_planning_status(ito_path)?`
  - [x] Remove `use ito_common::io` and inline path construction
- [x] Update `cli/app/show.rs`:
  - [x] Replace manual spec path construction + `ito_common::io::read_to_string` with `ito_core::show::read_spec_markdown(ito_path, &item)?` (existing function, zero new code)
  - [x] Replace `ito_common::io::read_to_string_or_default(&module_md_path)` with `ito_core::show::read_module_markdown(ito_path, &module_id)?`
  - [x] Replace `ito_common::match_::nearest_matches` with `ito_core::nearest_matches`
  - [x] Remove `use ito_common::paths`, `use ito_common::io`, `use ito_common::match_` from this file
- [x] Update `cli/app/validate.rs`:
  - [x] Replace `ito_common::match_::nearest_matches` with `ito_core::nearest_matches`
  - [x] Replace tasks.md validation logic (~40 lines) with `ito_core::validate::validate_tasks_file(ito_path, change_id)?`
  - [x] Replace `ito_common::io::read_to_string_std` for spec markdown with `ito_core::validate::validate_spec(ito_path, spec_id, strict)`
  - [x] Remove duplicate module-existence check (lines ~94-104) — `repo_integrity` handles this
  - [x] Remove `use ito_common::id::parse_change_id`, `use ito_common::io`, `use ito_common::paths` from this file
- [x] Update `cli/app/common.rs`:
  - [x] Replace `core_paths::spec_markdown_path` usage with core show/validate functions
  - [x] Replace `core_paths::specs_dir` usage with a core function or re-export
  - [x] Remove `use ito_common::paths` from this file
- [x] Update `cli/commands/create.rs`:
  - [x] Replace `ito_common::id::parse_module_id` with `ito_core::parse_module_id`
  - [x] Remove `use ito_common::id` from this file
- [x] Verify `ito-cli/src/**/*.rs` has no remaining `use ito_common::paths`, `use ito_common::io` (except the 3 legitimate sites in `config.rs` and `ralph.rs`), no `use ito_common::id`, no `use ito_common::match_`
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 5: Guardrails and spec updates

- [x] Update `ito-rs/tools/arch_guardrails.py`:
  - [x] Remove `ito-schemas`, `ito-harness`, `ito-models` from all crate edge maps
  - [ ] Update `REQUIRED_CRATE_EDGES`: `ito-core` must depend on `ito-domain`, `ito-config`. `ito-cli` must depend on `ito-core`. `ito-web` must depend on `ito-core` (review: if phantom dep, decide whether to enforce or drop) <!-- NOTE: ito-config missing from ito-core required edges -->
  - [x] Verify domain API bans (`std::fs` baseline of 9 in `discovery.rs`) still pass
  - [-] Verify core API bans (`miette::`, `miette!`) still pass <!-- N/A: no core API bans exist in guardrails script -->
- [ ] Run `make arch-guardrails` — verify guardrails pass with new configuration
- [x] Update `ito-rs/Cargo.toml` workspace: verify `[workspace.members]` lists exactly 9 entries (8 primary + test-support)
- [x] Update `ito-rs/Cargo.toml` `[workspace.dependencies]`: remove `ito-schemas` and `ito-harness` entries if present
- [ ] Verify `cargo build --workspace` succeeds
- [ ] Verify `cargo test --workspace` passes
- [ ] Verify `cargo clippy --workspace -- -D warnings` passes
- [ ] Verify `ito-cli` builds with `--no-default-features` (no `ito-web` pulled in)
- [ ] Run full `make check && make test` — final verification
