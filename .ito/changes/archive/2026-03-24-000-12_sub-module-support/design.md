## Context

Sub-modules introduce a second scope beneath existing modules without replacing the current module model. The system already relies on IDs for directory names, parsing, sorting, repository lookups, sync, and prompt examples, so even a small syntax change (`NNN.SS-NN_name`) has wide blast radius across filesystem and remote-backed code paths.

Existing `ito create module` and `ito create change` flows are filesystem-oriented today, while remote persistence mode already relies on runtime-selected repositories for reads. That means the design must separate two concerns cleanly: (1) the new identifier and metadata model, which must work everywhere, and (2) creation flows, which remain local scaffolding plus backend-compatible storage/sync behavior unless a later change introduces backend-native mutation APIs.

## Goals / Non-Goals

- Goals:
  - Support one level of sub-modules with canonical IDs `NNN.SS`
  - Allow parent modules to own direct changes while also owning sub-modules
  - Support module-level change IDs (`NNN-NN_name`) and sub-module change IDs (`NNN.SS-NN_name`) side by side
  - Ensure filesystem-backed and remote-backed repositories can parse, list, show, sync, and store the new IDs
  - Provide a prompt-sweep instruction that finds old-only ID assumptions in prompts/templates/instructions
- Non-Goals:
  - Sub-sub-modules or unbounded nesting in this change
  - Renaming existing module-level change IDs
  - Inventing backend-native create APIs for sub-modules or changes beyond keeping backend-backed repositories/storage/sync compatible with the new IDs

## Decisions

- Canonical identifiers:
  - Module ID: `NNN`
  - Sub-module ID: `NNN.SS`
  - Module change ID: `NNN-NN_name`
  - Sub-module change ID: `NNN.SS-NN_name`
- Filesystem layout:
  - Parent module metadata remains `.ito/modules/NNN_<parent>/module.md`
  - Sub-module metadata lives at `.ito/modules/NNN_<parent>/sub/SS_<child>/module.md`
  - Changes remain flat under `.ito/changes/`
- Ownership model:
  - Parent module checklists contain only module-level changes
  - Sub-module checklists contain only sub-module changes
  - A parent module can have both at the same time
- Repository model:
  - `ModuleRepository` becomes the canonical source for listing/getting sub-modules in both filesystem and remote-backed modes
  - `ChangeRepository` preserves canonical IDs and exposes optional `sub_module_id` metadata for filtering and display
  - Remote-backed project stores persist sub-module metadata as part of module state so `ModuleRepository` has a defined source of truth in backend mode
- Prompt sweep scope:
  - The sweep does not flag every legacy ID as a migration target
  - It flags places that assume `NNN-NN_name` is the only valid format and recommends generalizing those prompts/examples/regexes
  - It must cover both installed prompt surfaces and authoritative template sources so `ito init` does not reintroduce old-only examples

## Risks / Trade-offs

- Dotted IDs can break hand-written split/regex logic in multiple layers. Mitigation: centralize parsing/classification and require repositories/CLI to use it instead of inline string handling.
- Remote persistence support can look complete while still missing nested module data. Mitigation: require explicit remote-backed module repository scenarios and backend integration tasks.
- Prompt sweeps can create noisy false positives if they treat valid old IDs as obsolete. Mitigation: scope the sweep to old-only assumptions, not valid legacy identifiers.

## Migration Plan

1. Introduce parser/domain changes so both ID formats coexist.
2. Extend filesystem repositories and local creation flows.
3. Extend remote-backed repositories, storage adapters, and sync to preserve the new IDs.
4. Add prompt-sweep guidance for existing repos that want to update prompt/instruction surfaces.
5. Leave existing IDs untouched; only new sub-module changes use the dotted form.

## Open Questions

- Whether future backend-native mutation flows should create sub-modules and sub-module changes directly in remote persistence mode, rather than relying on local scaffolding plus sync.
