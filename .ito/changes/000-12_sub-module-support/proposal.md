<!-- ITO:START -->
## Why

Large modules (e.g., `024_ito-backend` with 7+ changes) have no way to express internal grouping today — all changes sit flat under the module. Sub-modules provide a first-class way to organize related changes into named sub-areas (e.g., `024.01_auth`, `024.02_sync`) without splitting into unrelated top-level modules, reducing noise in listings and making ownership and scope clearer.

## What Changes

- **New concept: sub-module** — a named, numbered child of a module with its own `module.md`, living at `.ito/modules/NNN_parent/sub/SS_child/`
- **New ID format**: `NNN.SS-NN_name` (e.g., `024.01-03_add-jwt`) — module `024`, sub-module `01`, change `03`. Old format `NNN-NN_name` remains valid and canonical for module-level changes
- **ID parser extended** — `parse_change_id`, `extract_module_id`, and related functions updated to handle both formats transparently; new `ParsedChangeId` return type carries optional sub-module field
- **Module domain model extended** — `Module` gains an optional `sub_modules: Vec<SubModule>` field; new `SubModule` domain type added
- **Module repository extended** — `ModuleRepository` gains methods to list and get sub-modules; filesystem implementation reads `.ito/modules/NNN/sub/SS_name/module.md`
- **Backend repositories updated** — all backend-backed module/change repository implementations support sub-module ID format in queries, storage, and listing
- **New CLI commands** — `ito create sub-module <name> --module <id>`, sub-module listing nested under `ito list --modules`, `ito show sub-module <id>`
- **Migration prompts** — agent-facing sweep prompt(s) to detect and report hardcoded old-format IDs embedded in `.ito/` artifacts (proposals, specs, tasks, user-prompts), with guidance on how to upgrade

## Capabilities

### New Capabilities

- `sub-module`: Defines the sub-module entity, directory layout, and `module.md` schema
- `sub-module-id-format`: Defines the `NNN.SS-NN_name` ID format, canonical form, parsing rules, and disambiguation from plain `NNN-NN_name` IDs
- `cli-sub-module`: CLI surface for creating, listing, and showing sub-modules
- `repo-sweep-prompt`: Agent prompt that sweeps a repository for hardcoded old-format IDs embedded in Ito artifact files and reports findings with upgrade guidance

### Modified Capabilities

- `flexible-id-parser`: Extend to parse the new `NNN.SS-NN_name` format and return a structured result with optional sub-module field; old format still accepted unchanged
- `module-repository`: Add sub-module listing and retrieval methods; filesystem implementation reads sub-module directories; `ModuleSummary` gains optional nested sub-module summaries
- `change-creation`: `ito create change` gains optional `--sub-module <id>` flag; when provided, the allocated change ID uses the `NNN.SS-NN_name` format and the change is associated with the sub-module's `module.md`
- `backend-artifact-store`: Backend artifact storage, querying, and indexing must accept and correctly route `NNN.SS-NN_name` formatted change IDs without treating the dot as invalid
- `backend-change-sync`: Change sync operations must preserve and propagate the sub-module ID component when syncing between local and backend stores

## Impact

- `ito-domain`: new `SubModule` type, extend `Module`, extend `ChangeSummary`/`Change` with `sub_module_id`, update `parse_change_id`/`extract_module_id` to handle `NNN.SS-NN_name`
- `ito-core`: extend filesystem `ModuleRepository`, extend `ChangeRepository` allocation to support `--sub-module`
- `ito-backend`: all backend repository adapters (`backend_module_repository.rs`, `backend_change_repository.rs`, `backend_artifact_store`) must handle dotted IDs
- `ito-cli`: new `create sub-module`, `show sub-module` commands; update `list --modules` display
- `ito-templates`: add `repo-sweep-prompt` agent prompt template
- `.ito/specs/`: new spec files for `sub-module`, `sub-module-id-format`, `cli-sub-module`, `repo-sweep-prompt`
- No breaking changes to existing change IDs or module IDs; all old formats remain valid
<!-- ITO:END -->
