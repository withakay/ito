# Tasks: Consolidate Workspace Crates

## Phase 1: Merge `ito-schemas` → `ito-domain`

- [ ] Create `ito-domain/src/schemas/` module directory
  - [ ] Create `ito-domain/src/schemas/mod.rs` re-exporting all submodules
  - [ ] Move `ito-schemas/src/workflow.rs` → `ito-domain/src/schemas/workflow.rs`
  - [ ] Move `ito-schemas/src/workflow_plan.rs` → `ito-domain/src/schemas/workflow_plan.rs`
  - [ ] Move `ito-schemas/src/workflow_state.rs` → `ito-domain/src/schemas/workflow_state.rs`
- [ ] Update `ito-domain/src/lib.rs` to declare `pub mod schemas;`
- [ ] Update `ito-domain/Cargo.toml`:
  - [ ] Add `serde_yaml` to `[dependencies]` (from ito-schemas)
  - [ ] Remove `ito-schemas` from `[dependencies]`
- [ ] Update all `ito-core` imports: `ito_schemas::*` → `ito_domain::schemas::*`
  - [ ] Grep for `use ito_schemas` in `ito-core/src/**/*.rs` and update each site
  - [ ] Remove `ito-schemas` from `ito-core/Cargo.toml` `[dependencies]`
- [ ] Add re-export in `ito-core/src/lib.rs`: `pub use ito_domain::schemas;`
- [ ] Update all `ito-cli` imports: `ito_schemas::*` → `ito_core::schemas::*`
  - [ ] Grep for `use ito_schemas` in `ito-cli/src/**/*.rs` and update each site
  - [ ] Remove `ito-schemas` from `ito-cli/Cargo.toml` `[dependencies]`
- [ ] Remove `ito-schemas` crate from workspace:
  - [ ] Remove `"crates/ito-schemas"` from `ito-rs/Cargo.toml` `[workspace.members]`
  - [ ] Delete `ito-rs/crates/ito-schemas/` directory
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 2: Merge `ito-harness` → `ito-core`

- [ ] Create `ito-core/src/harness/` module directory
  - [ ] Create `ito-core/src/harness/mod.rs` re-exporting all submodules
  - [ ] Move `ito-harness/src/types.rs` → `ito-core/src/harness/types.rs`
  - [ ] Move `ito-harness/src/opencode.rs` → `ito-core/src/harness/opencode.rs`
  - [ ] Move `ito-harness/src/stub.rs` → `ito-core/src/harness/stub.rs`
- [ ] Update `ito-core/src/lib.rs` to declare `pub mod harness;`
- [ ] Update internal `ito-core` imports: `ito_harness::*` → `crate::harness::*`
  - [ ] Grep for `use ito_harness` in `ito-core/src/**/*.rs` and update each site
  - [ ] Remove `ito-harness` from `ito-core/Cargo.toml` `[dependencies]`
- [ ] Update all `ito-cli` imports: `ito_harness::*` → `ito_core::harness::*`
  - [ ] Grep for `use ito_harness` in `ito-cli/src/**/*.rs` and update each site
  - [ ] Remove `ito-harness` from `ito-cli/Cargo.toml` `[dependencies]`
- [ ] Remove `ito-harness` crate from workspace:
  - [ ] Remove `"crates/ito-harness"` from `ito-rs/Cargo.toml` `[workspace.members]`
  - [ ] Delete `ito-rs/crates/ito-harness/` directory
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 3: CLI bypass elimination — new core functions

These functions push I/O operations out of the CLI adapter and into `ito-core`, where they belong. Each function encapsulates a read or read-mutate-write cycle that the CLI currently does by hand using `ito_common::io::*` and `ito_common::paths::*`.

- [ ] Create `ito-core/src/state.rs` module with:
  - [ ] `pub fn read_state(ito_path: &Path) -> CoreResult<String>` — reads `{ito_path}/planning/STATE.md`
  - [ ] `pub enum StateAction { AddDecision { text: String }, AddBlocker { text: String }, AddNote { text: String }, SetFocus { text: String }, AddQuestion { text: String } }`
  - [ ] `pub fn update_state(ito_path: &Path, action: StateAction) -> CoreResult<()>` — reads STATE.md, applies the mutation using existing `ito_domain::state::*` functions, writes back
  - [ ] Unit test: `read_state` returns error for missing file
  - [ ] Unit test: `update_state` with each `StateAction` variant produces expected markdown
- [ ] Register `pub mod state;` in `ito-core/src/lib.rs`
- [ ] Create `ito_core::tasks::read_tasks_markdown(ito_path: &Path, change_id: &str) -> CoreResult<String>`
  - [ ] Add function to `ito-core/src/tasks.rs` — reads `{change_dir}/tasks.md` using `tasks_path()` from domain
  - [ ] Unit test: returns contents for existing tasks.md
  - [ ] Unit test: returns error for missing tasks.md
- [ ] Create `ito_core::planning::read_planning_status(ito_path: &Path) -> CoreResult<String>`
  - [ ] Add function to new `ito-core/src/planning.rs` module — reads `{ito_path}/planning/ROADMAP.md`
  - [ ] Register `pub mod planning;` in `ito-core/src/lib.rs`
  - [ ] Unit test: returns contents for existing ROADMAP.md
  - [ ] Unit test: returns error for missing ROADMAP.md
- [ ] Create `ito_core::show::read_module_markdown(ito_path: &Path, module_id: &str) -> CoreResult<String>`
  - [ ] Add function to `ito-core/src/show/mod.rs` — reads `{module_dir}/module.md`
  - [ ] Unit test: returns contents for existing module.md
  - [ ] Unit test: returns error for missing module.md
- [ ] Create `ito_core::validate::validate_tasks_file(ito_path: &Path, change_id: &str) -> CoreResult<Vec<ValidationIssue>>`
  - [ ] Add function to `ito-core/src/validate/mod.rs` — reads tasks.md, parses it, converts diagnostics to `ValidationIssue` items
  - [ ] Unit test: returns empty vec for valid tasks file
  - [ ] Unit test: returns diagnostics for malformed tasks file
- [ ] Run `make check && make test` — verify new functions compile and tests pass

## Phase 4: CLI bypass elimination — re-exports and call-site updates

- [ ] Add re-exports to `ito-core/src/lib.rs`:
  - [ ] `pub use ito_common::match_::nearest_matches;`
  - [ ] `pub use ito_common::id::parse_module_id;`
- [ ] Update `cli/commands/state.rs`:
  - [ ] Replace `ito_common::io::read_to_string(&state_path)` with `ito_core::state::read_state(ito_path)?`
  - [ ] Replace read → mutate → write cycle with `ito_core::state::update_state(ito_path, action)?`
  - [ ] Remove `use ito_common::io` from this file
- [ ] Update `cli/commands/tasks.rs`:
  - [ ] Replace `ito_common::io::read_to_string(&path)` calls (lines ~585, ~629) with `ito_core::tasks::read_tasks_markdown(ito_path, change_id)?`
  - [ ] Replace `core_paths::change_dir(ito_path, &change_id).join("tasks.md")` with `ito_core::tasks::tasks_path(ito_path, &change_id)` (already re-exported from domain)
  - [ ] Remove `use ito_common::paths` and `use ito_common::io` from this file
- [ ] Update `cli/commands/plan.rs`:
  - [ ] Replace `ito_common::io::read_to_string(&roadmap_path)` with `ito_core::planning::read_planning_status(ito_path)?`
  - [ ] Remove `use ito_common::io` and inline path construction
- [ ] Update `cli/app/show.rs`:
  - [ ] Replace manual spec path construction + `ito_common::io::read_to_string` with `ito_core::show::read_spec_markdown(ito_path, &item)?` (existing function, zero new code)
  - [ ] Replace `ito_common::io::read_to_string_or_default(&module_md_path)` with `ito_core::show::read_module_markdown(ito_path, &module_id)?`
  - [ ] Replace `ito_common::match_::nearest_matches` with `ito_core::nearest_matches`
  - [ ] Remove `use ito_common::paths`, `use ito_common::io`, `use ito_common::match_` from this file
- [ ] Update `cli/app/validate.rs`:
  - [ ] Replace `ito_common::match_::nearest_matches` with `ito_core::nearest_matches`
  - [ ] Replace tasks.md validation logic (~40 lines) with `ito_core::validate::validate_tasks_file(ito_path, change_id)?`
  - [ ] Replace `ito_common::io::read_to_string_std` for spec markdown with `ito_core::validate::validate_spec(ito_path, spec_id, strict)`
  - [ ] Remove duplicate module-existence check (lines ~94-104) — `repo_integrity` handles this
  - [ ] Remove `use ito_common::id::parse_change_id`, `use ito_common::io`, `use ito_common::paths` from this file
- [ ] Update `cli/app/common.rs`:
  - [ ] Replace `core_paths::spec_markdown_path` usage with core show/validate functions
  - [ ] Replace `core_paths::specs_dir` usage with a core function or re-export
  - [ ] Remove `use ito_common::paths` from this file
- [ ] Update `cli/commands/create.rs`:
  - [ ] Replace `ito_common::id::parse_module_id` with `ito_core::parse_module_id`
  - [ ] Remove `use ito_common::id` from this file
- [ ] Verify `ito-cli/src/**/*.rs` has no remaining `use ito_common::paths`, `use ito_common::io` (except the 3 legitimate sites in `config.rs` and `ralph.rs`), no `use ito_common::id`, no `use ito_common::match_`
- [ ] Run `make check && make test` — verify clean build and all tests pass

## Phase 5: Guardrails and spec updates

- [ ] Update `ito-rs/tools/arch_guardrails.py`:
  - [ ] Remove `ito-schemas`, `ito-harness`, `ito-models` from all crate edge maps
  - [ ] Update `REQUIRED_CRATE_EDGES`: `ito-core` must depend on `ito-domain`, `ito-config`. `ito-cli` must depend on `ito-core`. `ito-web` must depend on `ito-core` (review: if phantom dep, decide whether to enforce or drop)
  - [ ] Verify domain API bans (`std::fs` baseline of 9 in `discovery.rs`) still pass
  - [ ] Verify core API bans (`miette::`, `miette!`) still pass
- [ ] Run `make arch-guardrails` — verify guardrails pass with new configuration
- [ ] Update `ito-rs/Cargo.toml` workspace: verify `[workspace.members]` lists exactly 9 entries (8 primary + test-support)
- [ ] Update `ito-rs/Cargo.toml` `[workspace.dependencies]`: remove `ito-schemas` and `ito-harness` entries if present
- [ ] Verify `cargo build --workspace` succeeds
- [ ] Verify `cargo test --workspace` passes
- [ ] Verify `cargo clippy --workspace -- -D warnings` passes
- [ ] Verify `ito-cli` builds with `--no-default-features` (no `ito-web` pulled in)
- [ ] Run full `make check && make test` — final verification
