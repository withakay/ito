<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Deterministic allocation state serialization

The change-allocation state file SHALL be serialized deterministically with module IDs in ascending order, treating sub-module-qualified keys (`NNN.SS`) as distinct from plain module keys (`NNN`).

#### Scenario: Allocation state write is module-ID ordered

- **WHEN** a new change number is allocated for any module or sub-module
- **THEN** `.ito/workflows/.state/change-allocations.json` is written with module entries ordered by ascending canonical ID
- **AND** sub-module keys (e.g., `"024.01"`) appear after their parent module key (e.g., `"024"`) in the ordering
- **AND** repeated writes with the same logical state produce equivalent key ordering

#### Scenario: Allocation state remains JSON snapshot format

- **WHEN** allocation state is persisted
- **THEN** the file format remains JSON object snapshot format
- **AND** readers continue to load existing JSON state without a migration step

### Requirement: Deterministic module change checklist ordering

Change checklist entries SHALL be emitted in ascending canonical change ID order within the metadata file that owns them.

Module-level changes SHALL appear only in the parent module's `module.md`.

Sub-module changes SHALL appear only in the owning sub-module's `module.md`.

#### Scenario: Adding a module-level change preserves sorted module checklist

- **WHEN** `ito create change` adds a new change to a module's `module.md`
- **THEN** entries under `## Changes` are written in ascending canonical change ID order
- **AND** existing entries are retained without duplication

#### Scenario: Adding a sub-module change preserves sorted sub-module checklist

- **WHEN** `ito create change` adds a new change to a sub-module's `module.md`
- **THEN** entries under that sub-module's `## Changes` section are written in ascending canonical change ID order
- **AND** existing entries are retained without duplication

## ADDED Requirements

### Requirement: Changes can be created under a sub-module

`ito create change` SHALL accept `--sub-module <id>` as an alternative to `--module <id>`.

When `--sub-module` is provided, the allocated change ID SHALL use the `NNN.SS-NN_name` format, and the change's `module.md` checklist entry SHALL be added to the sub-module's `module.md`, not the parent module's.

#### Scenario: Allocation uses sub-module namespace

- **GIVEN** sub-module `024.01` exists and has 2 existing changes (`024.01-01_*`, `024.01-02_*`)
- **WHEN** `ito create change my-change --sub-module 024.01` is executed
- **THEN** the new change is allocated as `024.01-03_my-change`
- **AND** the change directory is `.ito/changes/024.01-03_my-change/`

#### Scenario: Sub-module checklist is updated, parent is not

- **WHEN** `ito create change my-change --sub-module 024.01` is executed
- **THEN** `.ito/modules/024_ito-backend/sub/01_auth/module.md` gains a checklist entry for `024.01-03_my-change`
- **AND** `.ito/modules/024_ito-backend/module.md` is NOT modified

#### Scenario: --sub-module and --module flags are mutually exclusive

- **WHEN** user provides both `--module 024` and `--sub-module 024.01` to `ito create change`
- **THEN** the command exits with an error indicating the flags are mutually exclusive

#### Scenario: Sub-module-scoped change creation is rejected in remote persistence mode

- **GIVEN** remote persistence mode is active
- **WHEN** user executes `ito create change my-change --sub-module 024.01`
- **THEN** the command exits with an actionable error indicating sub-module-scoped creation currently requires local filesystem mode
<!-- ITO:END -->
