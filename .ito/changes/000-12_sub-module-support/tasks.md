# Tasks: 000-12_sub-module-support

## Wave 1 — Domain model and ID parser

**Objective**: Extend the core domain types and ID parsing logic to understand sub-module IDs. All higher-level work depends on this foundation.

**Verify**: `cargo test -p ito-domain` passes with all new ID parsing tests green.

- [x] **1.1** Add `SubModule` domain struct to `ito-domain` with fields: `id`, `parent_module_id`, `sub_id`, `name`, `description: Option<String>`, `change_count: u32`
- [x] **1.2** Add `sub_modules: Vec<SubModule>` field to the `Module` domain struct; default empty
- [x] **1.3** Add `SubModuleSummary` struct (id, name, change_count) and include `sub_modules: Vec<SubModuleSummary>` in `ModuleSummary`
- [x] **1.4** Add `sub_module_id: Option<String>` field to `Change` and `ChangeSummary` domain structs, storing the canonical `NNN.SS` sub-module ID when present
- [x] **1.5** Define `ParsedChangeId` struct: `{ module_id, sub_module_id: Option<String>, change_num, name: Option<String>, canonical }`, where `sub_module_id` uses canonical `NNN.SS` form
- [x] **1.6** Update `parse_change_id` to return `ParsedChangeId` and handle `NNN.SS-NN_name` format; old `NNN-NN_name` sets `sub_module_id = None`
- [x] **1.7** Update `extract_module_id` to strip the sub-module component and return only `NNN` for both old and new formats
- [x] **1.8** Add `parse_sub_module_id(input: &str) -> Result<String>` function to normalize `NNN.SS` / `NNN.SS_name` inputs
- [x] **1.9** Add `ItoIdKind` enum (`ModuleId`, `SubModuleId`, `ModuleChangeId`, `SubModuleChangeId`) and `classify_id` function
- [x] **1.10** Update `flexible-id-parser` spec scenarios to match new `ParsedChangeId` return type (ensure tests cover all 4 ID kinds)

---

## Wave 2 — Filesystem repositories

**Depends On**: Wave 1

**Objective**: The filesystem repositories read sub-module directories, preserve sub-module-qualified change IDs, and expose the right metadata for list/show flows.

**Verify**: `cargo test -p ito-core` passes; manually run `ito list --modules` on a test repo with sub-module directories.

- [x] **2.1** Update `ito-core` filesystem `ModuleRepository::get` to scan `.ito/modules/NNN_*/sub/` for sub-module directories and populate `module.sub_modules`
- [x] **2.2** Implement `ModuleRepository::list_sub_modules(parent_id: &str)` in filesystem backend: enumerate `sub/SS_name/` dirs, read each `module.md`
- [x] **2.3** Implement `ModuleRepository::get_sub_module(composite_id: &str)` — parse composite id, locate directory, read `module.md`, return `SubModule`
- [x] **2.4** Update `ModuleRepository::list()` to populate `sub_modules` in each `ModuleSummary`
- [x] **2.5** Update `ModuleRepository::list_with_changes()` to include sub-module changes alongside module changes
- [x] **2.6** Update filesystem `ChangeRepository` list/get paths to populate `sub_module_id` for `Change` and `ChangeSummary` when a change ID uses `NNN.SS-NN_name`
- [x] **2.7** Add regression tests proving a parent module can retain direct change `024-07_*` while sub-module `024.01` owns `024.01-01_*`

---

## Wave 3 — Change creation for sub-modules

**Depends On**: Wave 1

**Objective**: `ito create change` can allocate and create changes under a sub-module using the `NNN.SS-NN_name` format.

**Verify**: `cargo test -p ito-core -- create` passes; `ito create change my-test --sub-module 024.01` produces `024.01-01_my-test/` directory.

- [~] **3.1** Update change allocation state serialization to handle `NNN.SS` keys alongside plain `NNN` keys, maintaining deterministic sort order (sub-module keys sort after their parent)
- [ ] **3.2** Add `--sub-module <id>` flag to `ito create change` CLI handler; validate it is mutually exclusive with `--module`
- [ ] **3.3** Update the allocation logic to use the sub-module composite key (`NNN.SS`) as the namespace for change numbering
- [ ] **3.4** Update `ito create change` to write the new change ID in `NNN.SS-NN_name` canonical form when `--sub-module` is provided
- [ ] **3.5** Update post-creation module checklist write to target the sub-module's `module.md` (not the parent) when `--sub-module` is used
- [ ] **3.6** Ensure checklist ordering in the owning `module.md` (parent module or sub-module) is ascending by canonical change ID
- [ ] **3.7** Add remote-mode guard test showing `ito create change --sub-module ...` exits with actionable local-mode guidance when remote persistence is active

---

## Wave 4 — CLI sub-module commands

**Depends On**: Wave 2, Wave 3

**Objective**: Users can create, list, and show sub-modules via CLI.

**Verify**: End-to-end test: `ito create sub-module auth --module 024` followed by `ito list --modules` showing nested sub-module; `ito show sub-module 024.01` showing metadata.

- [ ] **4.1** Add `ito create sub-module <name> --module <id>` command: allocate next sub-module number, write `module.md` at correct path, print confirmation
- [ ] **4.2** Add `--description <text>` flag to `ito create sub-module` and include it in generated `module.md`
- [ ] **4.3** Update `ito list --modules` display to render sub-modules indented under their parent module with id, name, and change count
- [ ] **4.4** Add `ito show sub-module <NNN.SS>` command: load sub-module, display metadata and associated change list
- [ ] **4.5** Add error handling: unknown parent module, unknown sub-module ID, duplicate sub-module name, canonical ID output in create/show flows
- [ ] **4.6** Update `ito validate module <id>` to also validate sub-modules under that module (correct directory layout, valid module.md)
- [ ] **4.7** Add remote-mode guard test showing `ito create sub-module ...` exits with actionable local-mode guidance

---

## Wave 5 — Backend repository support

**Depends On**: Wave 1

**Objective**: All backend-backed repository implementations correctly handle sub-module IDs in reads, listings, storage, and sync.

**Verify**: `cargo test -p ito-backend` passes; integration test writing/reading a sub-module change ID through each store backend.

- [ ] **5.1** Update `backend_change_repository.rs` so dotted change IDs are accepted as canonical IDs and returned with populated `sub_module_id`
- [ ] **5.2** Update `backend_module_repository.rs` to list and resolve sub-modules through the remote-backed repository implementation
- [ ] **5.3** Extend backend project store schemas/serialization so module records persist sub-module metadata as canonical backend state
- [ ] **5.4** Update backend list/show sorting and rendering paths to treat `NNN.SS-NN_name` and `NNN.SS` as valid canonical IDs
- [ ] **5.5** Update backend artifact store key generation to accept dots in change ID components (filesystem, SQLite, R2)
- [ ] **5.6** Update `backend_change_sync.rs` so sync push/pull preserves the sub-module component and uses updated ID parsing for parent-module scope resolution
- [ ] **5.7** Add integration tests: list modules with nested sub-modules in remote mode, show a sub-module, and read/write a sub-module change ID across supported backends
- [ ] **5.8** Add remote-mode CLI tests showing `ito list --modules` and `ito show sub-module` resolve through the runtime-selected `ModuleRepository`

---

## Wave 6 — Repo sweep prompt

**Depends On**: Wave 1 (for format definitions)

**Objective**: An agent sweep prompt exists for detecting old-only ID assumptions in prompt and instruction surfaces and guiding upgrades.

**Verify**: `ito agent instruction repo-sweep` outputs the sweep prompt without error; prompt contains scan targets, regex patterns, and upgrade guidance.

- [ ] **6.1** Add `repo-sweep` as a supported `ito agent instruction` target (no `--change` required)
- [ ] **6.2** Write the `repo-sweep` prompt template in `ito-templates` embedded assets, covering: scan targets (`.ito/user-prompts/`, `AGENTS.md`, `.opencode/`, `.github/`, `.codex/`, `ito-rs/crates/ito-templates/assets/`), detection patterns for old-only assumptions, reporting format, and upgrade guidance
- [ ] **6.3** Wire the `repo-sweep` template into `ito agent instruction` output path
- [ ] **6.4** Verify the sweep prompt is installed by `ito init` (or accessible without install) and does not require an active change context

---

## Wave 7 — Documentation and spec validation

**Depends On**: Wave 1–6

**Objective**: Specs are consistent with implementation; all new and modified specs validate cleanly.

**Verify**: `ito validate 000-12_sub-module-support --strict` passes with no errors or warnings.

- [ ] **7.1** Review and finalize `sub-module`, `sub-module-id-format`, `cli-sub-module`, and `repo-sweep-prompt` specs against actual implementation
- [ ] **7.2** Review and finalize `flexible-id-parser`, `module-repository`, `change-repository`, `change-creation`, `backend-artifact-store`, `backend-change-sync`, and `backend-project-store` delta specs
- [ ] **7.3** Add/update rustdoc on all new and modified public API items (`SubModule`, `ParsedChangeId`, `parse_change_id`, `classify_id`, repository methods)
- [ ] **7.4** Run `ito validate 000-12_sub-module-support --strict` and resolve any failures
