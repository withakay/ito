## ADDED Requirements

### Requirement: Deterministic allocation state serialization

The change-allocation state file SHALL be serialized deterministically with module IDs in ascending order.

#### Scenario: Allocation state write is module-ID ordered

- **WHEN** a new change number is allocated for any module
- **THEN** `.ito/workflows/.state/change-allocations.json` is written with module entries ordered by ascending module ID
- **AND** repeated writes with the same logical state produce equivalent key ordering

#### Scenario: Allocation state remains JSON snapshot format

- **WHEN** allocation state is persisted
- **THEN** the file format remains JSON object snapshot format
- **AND** readers continue to load existing JSON state without a migration step

### Requirement: Deterministic module change checklist ordering

Module change checklist entries SHALL be emitted in ascending canonical change ID order.

#### Scenario: Adding a change preserves sorted module checklist

- **WHEN** `ito create change` adds a new change to a module's `module.md`
- **THEN** entries under `## Changes` are written in ascending canonical change ID order
- **AND** existing entries are retained without duplication
