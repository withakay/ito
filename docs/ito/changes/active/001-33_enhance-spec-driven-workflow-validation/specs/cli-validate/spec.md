<!-- ITO:START -->
## ADDED Requirements

These requirements add opt-in validation rules. Rules are activated through the `rules:` extension defined by `ito-schemas:validation-rules-extension` and produce diagnostics that include both the validator id and the rule id.

### Requirement: Scenario grammar rule

When the `scenario_grammar` rule is enabled, `ito.delta-specs.v1` SHALL apply additional grammar checks to each `#### Scenario:` block in delta requirements.

- **Requirement ID**: cli-validate:scenario-grammar-validation

#### Scenario: Scenario without WHEN is invalid

- **GIVEN** the `scenario_grammar` rule is enabled at severity `error`
- **AND** a delta requirement contains a `#### Scenario:` block whose body has no line beginning with `**WHEN**` (case-insensitive, allowing `- ` bullet prefix)
- **WHEN** validation runs
- **THEN** validation reports an error naming the scenario and the missing keyword

#### Scenario: Scenario without THEN is invalid

- **GIVEN** the `scenario_grammar` rule is enabled at severity `error`
- **AND** a scenario body has no line beginning with `**THEN**`
- **WHEN** validation runs
- **THEN** validation reports an error naming the scenario and the missing keyword

#### Scenario: Scenario without GIVEN is recommended

- **GIVEN** the `scenario_grammar` rule is enabled
- **AND** a scenario body has WHEN and THEN but no `**GIVEN**`
- **WHEN** validation runs
- **THEN** validation reports a warning recommending a `GIVEN` precondition for that scenario

#### Scenario: Excessive step count is warned at threshold 8

- **GIVEN** the `scenario_grammar` rule is enabled
- **AND** a scenario body contains more than 8 step lines (lines starting with `- **GIVEN**`, `- **WHEN**`, `- **THEN**`, or `- **AND**`)
- **WHEN** validation runs
- **THEN** validation reports a warning that the scenario exceeds the recommended step count of 8 and suggests splitting

#### Scenario: UI-mechanics warning uses conservative multi-token patterns

- **GIVEN** the `ui_mechanics` rule is enabled
- **AND** a requirement is not tagged `ui` (per `ito-schemas:behavioral-requirement-metadata` Tags metadata)
- **AND** a scenario body matches a known UI-mechanics regex pattern in the canonical pattern set:
  - `\bclick\s+(?:on\s+|the\s+)?\w+`
  - `\bwait\s+\d+\s*(?:ms|millisecond|second|s)\b`
  - `\bsleep\s+\d+\b`
  - `\bselector\s*[:=]`
  - `\bcss\s+selector\b`
- **WHEN** validation runs
- **THEN** validation reports a warning that the scenario may describe UI mechanics rather than domain behavior, and suggests adding the `ui` tag if intentional

#### Scenario: Bare anchor or class names do not trip the UI rule

- **GIVEN** the `ui_mechanics` rule is enabled
- **WHEN** a scenario body contains a markdown anchor like `[link](#section)` or a CSS-shaped token like `.unwrap` from a code phrase such as `.unwrap()`
- **THEN** validation does NOT emit a UI-mechanics warning for those tokens

### Requirement: Proposal-capability consistency rule

When the `capabilities_consistency` rule is enabled, `ito.delta-specs.v1` SHALL parse the proposal `## Capabilities` section and compare its lists to the change's delta directories and the project's archived baseline specs.

- **Requirement ID**: cli-validate:proposal-capabilities-consistency

#### Scenario: Capability list is parsed using a defined grammar

- **GIVEN** the proposal contains the section `## Capabilities` with subsections `### New Capabilities` and `### Modified Capabilities`
- **AND** each subsection contains markdown bullets where the first inline-code token (`` `<name>` ``) is the capability name
- **WHEN** Ito parses the proposal
- **THEN** the parsed capability list contains the names from the first inline-code token in each bullet
- **AND** literal placeholders such as `<name>`, `<existing-name>`, or HTML comments are ignored

#### Scenario: Listed capability has no delta

- **GIVEN** the proposal lists `auth` under `### New Capabilities` or `### Modified Capabilities`
- **AND** the change has no `specs/auth/spec.md`
- **WHEN** validation runs
- **THEN** validation reports an error naming the capability and the missing delta path

#### Scenario: Delta has no proposal entry

- **GIVEN** the change contains `specs/notifications/spec.md`
- **AND** the proposal does not list `notifications` under either subsection
- **WHEN** validation runs
- **THEN** validation reports an error naming the unlisted capability

#### Scenario: New vs Modified is checked against baseline specs

- **GIVEN** the project's baseline specs live at `.ito/specs/`
- **AND** the proposal lists `auth` under `### New Capabilities`
- **AND** `.ito/specs/auth/` already exists
- **WHEN** validation runs
- **THEN** validation reports an error that the capability already exists and should be listed under `### Modified Capabilities`
- **AND** the symmetric error fires for capabilities listed as Modified that have no baseline directory

#### Scenario: Capability matching is case-sensitive directory matching

- **GIVEN** capability matching uses exact directory-name comparisons against `specs/<name>/`
- **WHEN** the proposal lists `Auth` and the directory is `auth`
- **THEN** validation reports the mismatch as a missing delta error

### Requirement: Contract reference syntax rule

When the `contract_refs` rule is enabled, `ito.delta-specs.v1` SHALL parse `Contract Refs` metadata on requirements and validate the syntax. v1 does NOT resolve references against external contract files.

- **Requirement ID**: cli-validate:contract-reference-validation

#### Scenario: Valid contract ref syntax is accepted

- **GIVEN** a requirement declares `- **Contract Refs**: openapi:POST /v1/password-reset, jsonschema:PasswordResetRequest`
- **WHEN** validation runs
- **THEN** validation accepts both references

#### Scenario: Unknown scheme is invalid

- **GIVEN** a requirement declares `- **Contract Refs**: graphql:UserQuery`
- **WHEN** validation runs
- **THEN** validation reports an error naming the unknown scheme `graphql` and the supported set `{openapi, jsonschema, asyncapi, cli, config}`

#### Scenario: No contract discovery configured produces a single advisory diagnostic per change

- **GIVEN** the change has at least one requirement with `Contract Refs`
- **AND** no contract-discovery configuration is present in the project (the v1 discovery config path is undefined and no project-local configuration sets it)
- **WHEN** validation runs
- **THEN** validation reports exactly one INFO-level diagnostic per change explaining that contract resolution is not configured and how to enable it later
- **AND** validation does not fail solely because resolution is unavailable

#### Scenario: Public Contract facet without requirement anchors warns once

- **GIVEN** a proposal Change Shape declares `Public Contract: openapi`
- **AND** no requirement in the change includes any `Contract Refs: openapi:...` entry
- **WHEN** validation runs
- **THEN** validation reports exactly one warning that the declared public contract is not anchored to any requirement

### Requirement: Task quality rule

When the `task_quality` rule is enabled on `tracking`, `ito.tasks-tracking.v1` SHALL validate enhanced-task field quality using a defined severity table.

- **Requirement ID**: cli-validate:task-quality-validation

#### Scenario: Severity table is canonical

- **GIVEN** the `task_quality` rule is enabled at the rule's configured severity
- **WHEN** validation runs over enhanced tasks
- **THEN** the rule applies this severity table:
  - Missing `Status` → error
  - Missing `Done When` → error
  - Missing `Verify` for an active task → error
  - `Verify` matches the vague-verification denylist (case-insensitive exact match against `run tests`, `run the tests`, `run all tests`, `test it`, `verify manually`, `check it works`) → warning
  - Missing `Files` → warning
  - Missing `Action` → warning
  - `Requirements` references a Requirement ID not declared in the change's delta specs → error

#### Scenario: Concrete Verify is accepted

- **GIVEN** a task has `Verify: cargo test -p ito-core --test validate scenario_grammar`
- **WHEN** validation runs
- **THEN** validation does NOT emit a vague-verification warning, because the value is not in the denylist

#### Scenario: An implementation task is identified by file extension

- **GIVEN** a task lists at least one file with a code extension matching `(\.rs|\.ts|\.tsx|\.js|\.py|\.go|\.toml|\.yaml|\.yml|\.json|\.sh)$`
- **WHEN** validation runs
- **THEN** the task is treated as an implementation task for the missing-Verify error path
- **AND** non-implementation tasks (for example pure documentation) downgrade missing-Verify to a warning
<!-- ITO:END -->
