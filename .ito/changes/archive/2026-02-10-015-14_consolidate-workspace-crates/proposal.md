# Proposal: Consolidate Workspace Crates

## Why

The onion architecture refactor (module 015) successfully established layering, port traits, and dependency direction — but it never addressed **crate count**. The workspace has 11 crates when the target architecture calls for fewer, better-justified boundaries. Three small crates (`ito-harness`, `ito-schemas`, and the non-existent `ito-models`) exist as separate workspace members without earning their keep. Two others (`ito-config`, `ito-logging`) are small but justify their boundaries for sound architectural reasons.

Additionally, the CLI adapter bypasses `ito-core` in ~20 call sites by importing directly from `ito-common`, `ito-config`, `ito-templates`, and `ito-schemas`. This violates the "thin adapter" principle. Crate consolidation without fixing the bypasses would just move deck chairs.

This change consolidates the workspace to **8 crates** (6 primary + `ito-config` + `ito-logging` + `ito-test-support`) and eliminates CLI bypass paths in the same pass.

### What's being consolidated and why

1. **`ito-schemas`** (648 lines) — Pure serde types for workflow definitions. `ito-domain` already depends on it; the types are domain concepts. Separate crate adds indirection for no benefit.

2. **`ito-harness`** (393 lines) — Only consumed by `ito-core` and `ito-cli`. The `Harness` trait and two implementations don't justify a separate crate at this scale.

3. **`ito-models`** — Listed in the `rust-workspace` spec but doesn't exist on disk. Ghost reference.

4. **`ralph-crate` spec** — Specifies an `ito-ralph` crate that was never created. Ralph lives in `ito-core/src/ralph/` which is where it should stay.

### What's being kept separate and why

5. **`ito-config`** (1,413 lines) — Justified boundary. `schemars` dependency stays isolated. Config is a shared concern; adapters (`ito-cli`, `ito-web`) and `ito-core` may depend on `ito-config` directly.

6. **`ito-logging`** (303 lines) — Justified boundary. Specialized dependencies (`sha2`, `rand`, `uuid`, `hex`) stay isolated. True leaf crate with zero `ito-*` dependencies. Clean single responsibility.

### CLI bypass problem

The CLI has ~20 non-legitimate direct imports from `ito-common` (10 leaked infrastructure I/O, 4 leaked domain logic, 6 re-export gaps). These exist because core doesn't expose the necessary functions, so the CLI reaches past core to do its own file reads/writes for STATE.md, tasks.md, ROADMAP.md, spec markdown, and module markdown.

## What Changes

### BREAKING: Workspace reduced from 11 crates to 8

Target crate structure:

| Crate | Role | Change |
|---|---|---|
| `ito-common` | Shared foundation: `FileSystem` trait, ID types, paths, I/O, matching | Unchanged |
| `ito-domain` | Pure entities, value objects, repository port traits. Zero I/O. | Absorbs `ito-schemas` |
| `ito-config` | Configuration loading, cascading merge, ito-dir resolution | Unchanged (stays separate) |
| `ito-core` | Use-cases, orchestration, ports, infrastructure implementations | Absorbs `ito-harness` |
| `ito-cli` | Thin CLI adapter + composition root | Bypass paths eliminated |
| `ito-web` | Thin web adapter | Unchanged |
| `ito-templates` | Embedded template assets | Unchanged |
| `ito-logging` | JSONL telemetry logger | Unchanged (stays separate) |
| `ito-test-support` | Dev-only test utilities and mock repositories | Unchanged |

### Dependency direction

```
ito-common              (leaf: zero ito-* deps)
ito-logging             (leaf: zero ito-* deps)
    ↑
ito-config              (depends: ito-common only)
ito-domain              (depends: ito-common only)
    ↑
ito-core                (depends: ito-domain, ito-common, ito-config, ito-templates)
    ↑
ito-cli / ito-web       (depends: ito-core and may depend on ito-config; plus ito-common for legitimate utility use)
ito-cli also depends on ito-logging
```

### Crate merge details

#### Merge 1: `ito-schemas` → `ito-domain`

**What moves:**
- `ito-schemas/src/workflow.rs` → `ito-domain/src/schemas/workflow.rs`
- `ito-schemas/src/workflow_plan.rs` → `ito-domain/src/schemas/workflow_plan.rs`
- `ito-schemas/src/workflow_state.rs` → `ito-domain/src/schemas/workflow_state.rs`

**Public API:** `ito_domain::schemas::{WorkflowDefinition, ExecutionPlan, WorkflowExecution, ...}` — all current `ito_schemas::*` types re-exported under `ito_domain::schemas`.

**Dependency impact:**
- `ito-domain/Cargo.toml` absorbs `serde_yaml` (from ito-schemas). Already has `serde` and `serde_json`.
- `ito-core` and `ito-cli` update imports from `ito_schemas::*` → `ito_domain::schemas::*`.
- `ito-core` re-exports `ito_domain::schemas` for CLI consumption.
- `ito-cli` drops direct `ito-schemas` dependency; accesses schema types through `ito-core` re-exports.

**Why this is safe:** `ito-schemas` is a leaf crate with zero internal deps. `ito-domain` already depends on it. The types are domain concepts (workflow definitions, execution plans). Moving them into domain is semantically correct — these are the domain's vocabulary, not external schema artifacts.

#### Merge 2: `ito-harness` → `ito-core`

**What moves:**
- `ito-harness/src/types.rs` → `ito-core/src/harness/types.rs`
- `ito-harness/src/opencode.rs` → `ito-core/src/harness/opencode.rs`
- `ito-harness/src/stub.rs` → `ito-core/src/harness/stub.rs`

**Public API:** `ito_core::harness::{Harness, HarnessRunConfig, HarnessRunResult, HarnessName, OpencodeHarness, StubHarness}` — all current `ito_harness::*` types re-exported under `ito_core::harness`.

**Dependency impact:**
- `ito-core/Cargo.toml` absorbs no new external deps (ito-harness only uses `thiserror`, `miette`, `serde`, `serde_json` — all already in ito-core).
- `ito-cli` updates imports from `ito_harness::*` → `ito_core::harness::*`.
- `ito-cli` drops direct `ito-harness` dependency.

**Why this is safe:** 393 lines, 2 consumers, no unique external deps. The `Harness` trait is already used inside `ito-core`'s ralph module. The concrete implementations (`OpencodeHarness`, `StubHarness`) are infrastructure that belongs in core.

#### Cleanup: Remove `ito-models` from spec

`ito-models` is listed in the `rust-workspace` spec as a required crate directory but does not exist on disk and has never existed. Remove from spec.

#### Cleanup: Update `ralph-crate` spec

The `ralph-crate` spec requires Ralph to be in a dedicated `ito-ralph` crate. Ralph lives in `ito-core/src/ralph/` and should stay there. The spec delta corrects this.

### CLI adapter thinning: eliminate 20 bypass paths

The following new core functions and re-exports eliminate the CLI's non-legitimate direct imports from `ito-common`, `ito-templates`, and `ito-schemas`.

#### New core functions

**1. `ito_core::state::read_state(ito_path: &Path) -> CoreResult<String>`**

Reads `{ito_path}/planning/STATE.md` and returns contents.
Eliminates: 1 bypass in `cli/commands/state.rs`.

**2. `ito_core::state::update_state(ito_path: &Path, action: StateAction) -> CoreResult<()>`**

Reads STATE.md, applies mutation, writes back. `StateAction` is an enum:
```rust
pub enum StateAction {
    AddDecision { text: String },
    AddBlocker { text: String },
    AddNote { text: String },
    SetFocus { text: String },
    AddQuestion { text: String },
}
```
Uses existing `ito_domain::state::{add_decision, add_blocker, add_note, set_focus, add_question}` for the pure mutation step.
Eliminates: 2 bypasses in `cli/commands/state.rs` (read + write).

**3. `ito_core::tasks::read_tasks_markdown(ito_path: &Path, change_id: &str) -> CoreResult<String>`**

Reads `{change_dir}/tasks.md` and returns raw contents.
Eliminates: 2 bypasses in `cli/commands/tasks.rs`.

**4. `ito_core::planning::read_planning_status(ito_path: &Path) -> CoreResult<String>`**

Reads `{ito_path}/planning/ROADMAP.md` and returns contents.
Eliminates: 1 bypass in `cli/commands/plan.rs`.

**5. `ito_core::show::read_module_markdown(ito_path: &Path, module_id: &str) -> CoreResult<String>`**

Reads `{module_dir}/module.md` and returns contents. Complements existing `read_spec_markdown` and `read_change_proposal_markdown`.
Eliminates: 1 bypass in `cli/app/show.rs`.

**6. `ito_core::validate::validate_tasks_file(ito_path: &Path, change_id: &str) -> CoreResult<Vec<ValidationIssue>>`**

Reads tasks.md, parses it, converts diagnostics to `ValidationIssue` items. Encapsulates the read → parse → diagnostic-conversion pipeline that currently lives in CLI.
Eliminates: 1 bypass in `cli/app/validate.rs`.

#### New re-exports from `ito_core`

**7. `pub use ito_common::match_::nearest_matches;`** in `ito_core/src/lib.rs`

CLI uses `nearest_matches` for "did you mean?" UX in `validate.rs` and `show.rs`.
Eliminates: 2 bypasses.

**8. `pub use ito_common::id::parse_module_id;`** in `ito_core/src/lib.rs`

CLI uses `parse_module_id` in `create.rs` to extract the module numeric prefix for spinner messages.
Eliminates: 2 bypasses.

#### Use existing core functions (zero new code)

**10. Replace manual spec path construction + read in `cli/app/show.rs`** with existing `ito_core::show::read_spec_markdown(ito_path, &item)`.

The CLI currently constructs the path via `core_paths::spec_markdown_path()` then calls `ito_common::io::read_to_string()`. The core function does exactly this already.
Eliminates: 2 bypasses.

**11. Replace manual `tasks_path` construction in `cli/commands/tasks.rs`** with existing `ito_core::tasks::tasks_path()` (already re-exported from domain).

The CLI does `core_paths::change_dir(ito_path, &change_id).join("tasks.md")` when `tasks_path(ito_path, &change_id)` is already available through core.
Eliminates: 1 bypass.

**12. Remove duplicate module-existence check in `cli/app/validate.rs`** (~10 lines, lines 94-104).

The CLI parses `change_id` via `ito_common::id::parse_change_id` to extract the module ID, then checks module directory existence. This duplicates `repo_integrity` logic that already runs during validation.
Eliminates: 1 bypass.

#### Residual legitimate CLI → `ito-common` usages (kept)

These 3 usages are genuine adapter concerns and remain:

| File | Usage | Why legitimate |
|---|---|---|
| `commands/config.rs:153` | `ito_common::io::create_dir_all_std(parent)` | Writing to a user-specified `--output` path for schema export — not Ito state |
| `commands/config.rs:158` | `ito_common::io::write_atomic_std(output, bytes)` | Same — user-requested file output |
| `app/ralph.rs:167` | `ito_common::io::read_to_string_std(&path_buf)` | Reading a user-supplied `--file` prompt — not Ito state |

After this work, `ito-cli`'s dependency on `ito-common` is limited to 3 utility I/O calls for user-specified file paths. Config access is treated as a shared concern and may remain a direct `ito-config` dependency in adapters.

### Architecture guardrails updates

The `arch_guardrails.py` script needs updating:

1. **Remove `ito-harness`, `ito-schemas`, `ito-models` from all edge checks** — these crates no longer exist.
2. **Update `FORBIDDEN_CRATE_EDGES`** — `ito-domain` must not depend on `ito-core`, `ito-cli`, `ito-web` (unchanged). `ito-core` must not depend on `ito-cli`, `ito-web` (unchanged).
3. **Update `REQUIRED_CRATE_EDGES`** — `ito-core` must depend on `ito-domain` and `ito-config`. `ito-cli` must depend on `ito-core`. `ito-web` must depend on `ito-core`. Remove requirement for `ito-web` → `ito-core` if `ito-web` still doesn't actually use it (the phantom dep problem — address separately or require genuine usage).
4. **Verify `ito-cli` → `ito-domain` is still in `FORBIDDEN_CRATE_EDGES`** (already present).
5. **Verify domain API bans still pass** — `std::fs` baseline of 9 in `discovery.rs` should be unchanged.

## Capabilities

### Modified

- **`rust-workspace`** — Update required crate list from 12 to 9 (8 primary + `ito-test-support`). Remove `ito-models` (never existed), `ito-schemas`, `ito-harness`. Add dependency direction rules.
- **`ito-core`** — Update dependency list (absorbs harness). Add `harness` module requirements. Remove `ito-harness` from "SHALL depend on" list.
- **`ito-domain`** — Update dependency list (absorbs schemas). Remove `ito-schemas` from deps; add `serde_yaml` to external deps. Add `schemas` module requirement.
- **`ralph-crate`** — Correct spec to reflect Ralph living in `ito-core/src/ralph/` rather than a separate `ito-ralph` crate.
- **`repo-precommit-quality-gates`** — Update guardrails to reflect new crate set.

### Unchanged (kept separate)

- **`ito-config-crate`** — Remains as-is. Config stays a separate crate.
- **`ito-logging`** — Remains as-is. Logging stays a separate crate.
- **`ito-common-crate`** — No structural changes. Remains leaf crate.

## Impact

### Build

- **Compile time:** Marginal improvement. Fewer crates means less cargo overhead for dependency resolution and linking. Incremental builds may be slightly faster due to fewer crate boundaries.
- **New external dep in domain:** `serde_yaml` enters `ito-domain` (from schemas merge). Already in the workspace dep tree.
- **No new deps in core.** `ito-config` stays separate, so `schemars` stays out of core.

### Risk

- **Low-medium risk:** Mechanical but moderate-surface-area refactor. Two crate merges (schemas + harness) are straightforward file moves. CLI bypass elimination requires ~20 import site updates and 6 new core functions.
- **Mitigation:** Execute merges sequentially in dependency-leaf-first order (schemas → harness). Run `make check && make test` after each merge. Guardrails validate dependency direction at each step.
- **No runtime behavior changes.** All merges are structural. No logic changes. No API changes. No serialization format changes.

### Testing

- Run `make test` after each individual merge to catch import breakage immediately.
- Run `make check` (includes `cargo clippy`, `cargo fmt`, and `arch_guardrails.py`) after each merge.
- Run full CI pipeline after all merges complete.
- Verify the 3 remaining `ito-common` usages in CLI are the legitimate ones.
- Verify `ito-cli` builds with `--no-default-features` (no `ito-web` pulled in).
- Verify adapter config dependencies are allowed by guardrails.

### Migration order

1. **`ito-schemas` → `ito-domain`** — Leaf crate, no downstream impact.
2. **`ito-harness` → `ito-core`** — Leaf crate, minimal downstream impact.
3. **CLI bypass elimination** — New core functions, re-exports, and call-site updates.
4. **Guardrails and spec updates** — Update `arch_guardrails.py`, workspace spec, and all affected capability specs.
