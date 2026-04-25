<!-- ITO:START -->
## ADDED Requirements

These requirements add validation rules that make spec-driven artifacts mechanically consistent while keeping the workflow lightweight.

### Requirement: Scenario grammar validation

Ito validation SHALL enforce basic behavior-scenario grammar for delta requirements when a schema enables scenario validation.

- **Requirement ID**: cli-validate:scenario-grammar-validation

#### Scenario: Scenario requires WHEN and THEN

- **GIVEN** a delta requirement contains a `#### Scenario:` block
- **WHEN** the scenario has no `WHEN` step or no `THEN` step
- **THEN** `ito validate <change-id>` reports an error for that scenario

#### Scenario: GIVEN is recommended

- **GIVEN** a scenario contains `WHEN` and `THEN` steps but no `GIVEN` step
- **WHEN** `ito validate <change-id>` runs
- **THEN** validation reports a warning recommending a `GIVEN` precondition

#### Scenario: UI mechanics are discouraged unless tagged

- **GIVEN** a requirement is not tagged `ui`
- **WHEN** a scenario contains UI mechanics such as `click`, `selector`, `wait 500ms`, `sleep`, `#id`, or `.class`
- **THEN** validation reports a warning that the scenario may describe mechanics instead of domain behavior

### Requirement: Proposal capabilities match spec deltas

Ito validation SHALL compare a proposal's Capabilities section with the spec delta files present in the change.

- **Requirement ID**: cli-validate:proposal-capabilities-consistency

#### Scenario: Listed capability has no delta

- **GIVEN** a proposal lists a capability under New Capabilities or Modified Capabilities
- **WHEN** no corresponding `specs/<capability>/spec.md` delta exists
- **THEN** validation reports an error naming the missing capability delta

#### Scenario: Delta is not listed in proposal

- **GIVEN** a change contains `specs/<capability>/spec.md`
- **WHEN** the proposal does not list that capability under New Capabilities or Modified Capabilities
- **THEN** validation reports an error naming the unlisted delta capability

#### Scenario: New and modified capabilities are checked against baseline specs

- **GIVEN** a proposal lists a capability as new or modified
- **WHEN** validation compares the capability to archived baseline specs
- **THEN** validation reports an error if a new capability already exists or a modified capability does not exist

### Requirement: Contract reference validation

Ito validation SHALL validate lightweight contract references when proposals or requirements declare public contracts.

- **Requirement ID**: cli-validate:contract-reference-validation

#### Scenario: Referenced contract is missing

- **GIVEN** a requirement declares `openapi:POST /v1/password-reset` or `jsonschema:PasswordResetRequest`
- **WHEN** the referenced operation or schema cannot be found in configured contract files
- **THEN** validation reports an error naming the missing contract reference

#### Scenario: Contract facet has no requirement references

- **GIVEN** a proposal declares Public Contract as OpenAPI, JSON Schema, AsyncAPI, CLI, or config
- **WHEN** no requirement in the change references that contract type
- **THEN** validation reports a warning that the public contract facet is not anchored to any requirement

### Requirement: Task quality validation

Ito validation SHALL validate enhanced task metadata when a change uses enhanced tasks and requirement IDs.

- **Requirement ID**: cli-validate:task-quality-validation

#### Scenario: Code-changing task needs concrete verification

- **GIVEN** an enhanced task modifies implementation files
- **WHEN** the task has no Verify field or uses a vague value such as `run tests`
- **THEN** validation reports an error for missing verification or a warning for vague verification

#### Scenario: Task needs done-when criteria

- **GIVEN** an enhanced task is active
- **WHEN** the task has no Done When field
- **THEN** validation reports an error for missing completion criteria

#### Scenario: Task references unknown requirement

- **GIVEN** an enhanced task declares a Requirements metadata value
- **WHEN** the referenced requirement ID does not exist in the change's delta specs
- **THEN** validation reports an error naming the unresolved requirement reference
<!-- ITO:END -->
