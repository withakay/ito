<!-- ITO:START -->
## ADDED Requirements

These requirements extend workflow schema semantics and built-in spec-driven artifacts. They preserve the existing proposal → specs → design → tasks lifecycle and the existing `ito.delta-specs.v1` and `ito.tasks-tracking.v1` validator IDs.

### Requirement: Spec-driven proposal Change Shape block

The spec-driven proposal template SHALL include an optional Change Shape block with a defined vocabulary, and Ito MUST treat the block as advisory metadata when its values are valid.

- **Requirement ID**: ito-schemas:spec-driven-change-shape

#### Scenario: Change Shape uses defined vocabulary

- **GIVEN** a spec-driven proposal includes a `## Change Shape` block
- **WHEN** the block declares fields `Type`, `Risk`, `Stateful`, `Public Contract`, `Design Needed`, and `Design Reason`
- **THEN** Ito recognizes:
  - `Type ∈ {feature, fix, refactor, migration, contract, event-driven}`
  - `Risk ∈ {low, medium, high}`
  - `Stateful ∈ {yes, no}`
  - `Design Needed ∈ {yes, no}`
  - `Public Contract` as a comma-separated subset of `{none, openapi, jsonschema, asyncapi, cli, config}`
  - `Design Reason` as free text

#### Scenario: Invalid Change Shape values produce warnings

- **GIVEN** a Change Shape block declares a value outside its defined vocabulary (for example `Risk: catastrophic`)
- **WHEN** `ito validate <change-id>` runs
- **THEN** validation reports a warning naming the field and the invalid value
- **AND** the rest of validation continues unaffected

#### Scenario: Missing Change Shape is allowed

- **GIVEN** a spec-driven proposal omits the Change Shape block entirely
- **WHEN** `ito validate <change-id>` runs
- **THEN** validation does not require Change Shape and does not enable opt-in rules implicitly

### Requirement: Spec-driven requirements support behavioral metadata

The spec-driven spec template SHALL support optional `Tags`, `Contract Refs`, `Rules / Invariants`, and `State Transitions` sections at the requirement level. Each is advisory metadata for downstream validators and agent guidance.

- **Requirement ID**: ito-schemas:behavioral-requirement-metadata

#### Scenario: Tags metadata is parsed as a comma-separated list

- **GIVEN** a requirement contains `- **Tags**: behavior, ui`
- **WHEN** Ito parses the requirement
- **THEN** the parsed requirement exposes the tags `behavior` and `ui` as structured metadata

#### Scenario: Contract Refs metadata is parsed as a list of typed references

- **GIVEN** a requirement contains `- **Contract Refs**: openapi:POST /v1/password-reset, jsonschema:PasswordResetRequest`
- **WHEN** Ito parses the requirement
- **THEN** each reference is preserved as a typed pair `(scheme, identifier)` where `scheme ∈ {openapi, jsonschema, asyncapi, cli, config}` and the identifier is the trimmed remainder

#### Scenario: Rules and State Transitions are optional

- **GIVEN** a requirement governs stateful behavior
- **WHEN** the requirement includes `#### Rules / Invariants` and a `#### State Transitions` markdown table
- **THEN** Ito preserves both as requirement-scoped sections without making them mandatory for non-stateful requirements

### Requirement: Validation rules extension

A workflow schema's `validation.yaml` MUST allow a backward-compatible `rules:` map under any artifact entry and under a new `proposal:` entry. Rule names are stable identifiers that opt the artifact into additional checks performed by an existing validator. v1 introduces no new validator IDs.

- **Requirement ID**: ito-schemas:validation-rules-extension

#### Scenario: Rules extend an existing artifact validator

- **GIVEN** a schema declares
  ```
  artifacts:
    specs:
      validate_as: ito.delta-specs.v1
      rules:
        scenario_grammar: error
        contract_refs: warn
        ui_mechanics: warn
  ```
- **WHEN** Ito loads validation configuration
- **THEN** the existing `ito.delta-specs.v1` validator runs the additional rules at the configured severity
- **AND** diagnostics from each rule include both the validator id and the rule id

#### Scenario: Single `validate_as` schemas remain valid

- **GIVEN** a schema declares only `validate_as: ito.delta-specs.v1` with no `rules:` key
- **WHEN** Ito loads the configuration
- **THEN** validation runs exactly as it does today

#### Scenario: Unknown rule names are reported but do not abort

- **GIVEN** a `rules:` map references an unknown rule name
- **WHEN** Ito loads the configuration
- **THEN** Ito reports a configuration warning naming the unknown rule
- **AND** the remaining known rules still run

#### Scenario: Proposal artifact entry is supported

- **GIVEN** a schema declares
  ```
  proposal:
    validate_as: ito.delta-specs.v1
    rules:
      capabilities_consistency: error
  ```
- **WHEN** Ito loads validation configuration
- **THEN** the proposal artifact path resolves to `proposal.md` for that change
- **AND** the configured rule executes against the parsed proposal

### Requirement: Built-in spec-driven schema is opt-in for new rules

The built-in `spec-driven` `validation.yaml` MUST NOT enable any of the new opt-in rules by default in this change.

- **Requirement ID**: ito-schemas:opt-in-rules-default

#### Scenario: Default spec-driven schema runs only existing validators

- **GIVEN** a project that has not customized `spec-driven/validation.yaml`
- **WHEN** the user creates a new change with the default schema
- **THEN** validation runs the existing `ito.delta-specs.v1` and `ito.tasks-tracking.v1` checks only
- **AND** new rules from this change can be enabled by exporting the schema and editing `.ito/templates/schemas/spec-driven/validation.yaml`
<!-- ITO:END -->
