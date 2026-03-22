<!-- ITO:START -->
## Why

Large modules (e.g., `024_ito-backend` with 7+ changes) have no way to express internal grouping today — all changes sit flat under the module. Sub-modules provide a first-class way to organize related changes into named sub-areas (e.g., `024.01_auth`, `024.02_sync`) without splitting into unrelated top-level modules, reducing noise in listings and making ownership and scope clearer.

Success means a project can model module `024`, sub-module `024.01`, and changes under both `024-NN_*` and `024.01-NN_*` at the same time, with the format remaining unambiguous across filesystem, repository, and backend-backed read/sync flows.

## What Changes

- **New concept: sub-module** — a named, numbered child of a module with canonical ID `NNN.SS`, its own `module.md`, and filesystem metadata living at `.ito/modules/NNN_<parent>/sub/SS_<child>/`
- **New ID format**: `NNN.SS-NN_name` (e.g., `024.01-03_add-jwt`) — module `024`, sub-module `01`, change `03`. Old format `NNN-NN_name` remains valid and canonical for module-level changes
- **ID parser extended** — `parse_change_id`, `extract_module_id`, and related functions updated to handle both formats transparently; new `ParsedChangeId` return type carries optional sub-module field
- **Module domain model extended** — `Module` gains a `sub_modules: Vec<SubModule>` field; modules may own direct changes and sub-modules at the same time; new `SubModule` domain type added
- **Runtime-selected repositories extended** — `ModuleRepository` and `ChangeRepository` both understand the new IDs in filesystem-backed and remote-backed modes, including listing and resolving sub-modules and sub-module changes
- **Backend repositories updated** — backend-backed change/module repositories, artifact stores, and sync paths preserve and round-trip sub-module IDs without treating the dot as invalid
- **New CLI commands** — `ito create sub-module <name> --module <id>`, sub-module listing nested under `ito list --modules`, `ito show sub-module <id>`
- **Prompt sweep guidance** — agent-facing sweep prompt(s) to detect prompts, templates, regexes, and examples that assume only the old module-level change ID format exists, including embedded template sources, with guidance on how to generalize them for mixed module/sub-module repos
- **Initial scope boundary** — one sub-module level only; existing IDs are not renamed; remote-mode reads/list/show must work through runtime-selected repositories, but direct backend-native creation APIs are not introduced and sub-module creation stays explicitly local-mode only for now

## Capabilities

### New Capabilities

- `sub-module`: Defines the sub-module entity, directory layout, and `module.md` schema
- `sub-module-id-format`: Defines the `NNN.SS-NN_name` ID format, canonical form, parsing rules, and disambiguation from plain `NNN-NN_name` IDs
- `cli-sub-module`: CLI surface for creating, listing, and showing sub-modules
- `repo-sweep-prompt`: Agent prompt that sweeps prompt and instruction surfaces for old-only ID assumptions and reports findings with upgrade guidance

### Modified Capabilities

- `flexible-id-parser`: Extend to parse the new `NNN.SS-NN_name` format and return a structured result with optional sub-module field; old format still accepted unchanged
- `module-repository`: Extend the runtime-selected repository contract so filesystem-backed and remote-backed implementations can list and resolve sub-modules; `ModuleSummary` gains nested sub-module summaries
- `change-repository`: Extend the canonical change view so filesystem-backed and remote-backed implementations preserve sub-module-qualified IDs and expose optional `sub_module_id` metadata in summaries/details
- `change-creation`: `ito create change` gains optional `--sub-module <id>` flag; when provided, the allocated change ID uses the `NNN.SS-NN_name` format and the change is associated with the sub-module's `module.md`
- `backend-artifact-store`: Backend artifact storage, querying, and indexing must accept and correctly route `NNN.SS-NN_name` formatted change IDs without treating the dot as invalid
- `backend-change-sync`: Change sync operations must preserve and propagate the sub-module ID component when syncing between local and backend stores
- `backend-project-store`: Backend-managed project stores persist sub-module metadata as part of module state so remote-backed `ModuleRepository` implementations have a canonical source of truth for list/show flows

## Impact

- `ito-domain`: new `SubModule` type, extend `Module`, extend `ChangeSummary`/`Change` with `sub_module_id`, update `parse_change_id`/`extract_module_id` to handle `NNN.SS-NN_name`
- `ito-core`: extend filesystem and runtime-selected `ModuleRepository`/`ChangeRepository`, plus change allocation, checklist routing, and list/show flows for sub-modules
- `ito-backend`: backend repository adapters (`backend_module_repository.rs`, `backend_change_repository.rs`), project stores, storage adapters, and sync paths must handle dotted IDs and sub-module summaries/details
- `ito-cli`: new `create sub-module`, `show sub-module` commands; update `list --modules` display
- `ito-templates`: add `repo-sweep-prompt` agent prompt template focused on prompt/instruction upgrades
- `.ito/specs/`: new spec files for `sub-module`, `sub-module-id-format`, `cli-sub-module`, `repo-sweep-prompt`
- No breaking changes to existing change IDs or module IDs; all old formats remain valid
<!-- ITO:END -->
