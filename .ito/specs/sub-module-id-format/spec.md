<!-- ITO:START -->
## ADDED Requirements

### Requirement: Sub-module change IDs follow a dot-extended format

A change belonging to a sub-module SHALL use the canonical ID format `NNN.SS-NN_name`:

- `NNN` — three-digit zero-padded parent module number
- `.` — literal dot separating module from sub-module
- `SS` — two-digit zero-padded sub-module number
- `-` — dash separating sub-module qualifier from change number
- `NN` — two-digit zero-padded change number within the sub-module
- `_name` — kebab-case name suffix

Examples: `024.01-03_add-jwt`, `001.02-01_initial-spec`

#### Scenario: Sub-module change ID is distinct from a module change ID

- **WHEN** a parser receives `024.01-03_add-jwt`
- **THEN** it identifies this as a sub-module change ID (has a dot component)
- **AND** it does NOT confuse it with a plain module change ID like `024-03_add-jwt`

#### Scenario: Sub-module change ID canonical form

- **WHEN** inputs `24.1-3_foo`, `024.01-003_foo`, `024.1-3_foo` are parsed
- **THEN** all normalize to canonical form `024.01-03_foo`

### Requirement: Plain module change IDs remain valid and unchanged

Existing `NNN-NN_name` IDs SHALL remain valid, parseable, and canonical. No migration of existing IDs is required.

#### Scenario: Old-format ID parses without sub-module component

- **WHEN** a parser receives `024-03_add-jwt`
- **THEN** it returns `module_id = "024"`, `sub_module_id = None`, `change_num = "03"`, `name = "add-jwt"`

### Requirement: The four ID types are unambiguously distinguishable

A parser SHALL be able to determine from the ID string alone whether it is:
1. A **module ID** — `NNN` (no dash, no dot)
2. A **module-level change ID** — `NNN-NN_name` (dash, no dot)
3. A **sub-module change ID** — `NNN.SS-NN_name` (dot before the dash)
4. A **sub-module ID** — `NNN.SS` (dot, no dash)

#### Scenario: Module ID `024` is recognized

- **WHEN** parser receives `024`
- **THEN** it identifies the type as `ModuleId`

#### Scenario: Module-level change ID `024-03_foo` is recognized

- **WHEN** parser receives `024-03_foo`
- **THEN** it identifies the type as `ModuleChangeId`

#### Scenario: Sub-module change ID `024.01-03_foo` is recognized

- **WHEN** parser receives `024.01-03_foo`
- **THEN** it identifies the type as `SubModuleChangeId`

#### Scenario: Sub-module ID `024.01` is recognized

- **WHEN** parser receives `024.01`
- **THEN** it identifies the type as `SubModuleId`

### Requirement: Sub-module numbers use two-digit zero-padded format

Sub-module numbers SHALL be two-digit zero-padded (e.g., `01`, `12`), consistent with change numbers.

#### Scenario: Single-digit sub-module number is normalized

- **WHEN** input contains sub-module number `1`
- **THEN** it is normalized to `01`

### Requirement: Canonical and loose sub-module references remain distinguishable

The canonical sub-module ID SHALL be `NNN.SS`.

Forms such as `NNN.SS_name` MAY be accepted as loose input or display labels, but SHALL normalize to the canonical `NNN.SS` identifier.

#### Scenario: Loose sub-module reference normalizes to canonical ID

- **WHEN** a command receives `024.01_auth` as sub-module input
- **THEN** it resolves that input to canonical sub-module ID `024.01`
- **AND** it does not treat `024.01_auth` as a separate canonical ID kind
<!-- ITO:END -->
