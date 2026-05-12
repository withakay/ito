<!-- ITO:START -->
## ADDED Requirements

These requirements keep built-in workflow schema templates aligned with the validators that Ito actually runs and ensure the export command remains a faithful starting point for project-local customization.

### Requirement: Built-in schema templates match configured validators

Built-in workflow schema templates MUST use a markdown shape that is accepted by the validators declared in the same schema directory's `validation.yaml`.

- **Requirement ID**: cli-templates-schemas:template-validator-alignment

#### Scenario: Minimalist specs parse as deltas

- **GIVEN** the built-in `minimalist` schema configures `specs` as `validate_as: ito.delta-specs.v1`
- **WHEN** the spec template is rendered into a new change
- **THEN** the rendered file uses `## ADDED Requirements`, `### Requirement:`, and `#### Scenario:` headers (delta-spec shape)
- **AND** does not use `## Stories` or `### Story:` headers

#### Scenario: Event-driven specs parse as deltas

- **GIVEN** the built-in `event-driven` schema configures `specs` as `validate_as: ito.delta-specs.v1`
- **WHEN** the spec template is rendered into a new change
- **THEN** the rendered file uses delta requirement headers and does not use story-shaped headers

#### Scenario: Rendered samples pass strict validation

- **GIVEN** a synthetic minimal change is generated from each built-in schema using only its templates
- **WHEN** `ito validate <change-id> --strict` runs against that synthetic change
- **THEN** validation does not fail because of template/validator format incompatibility

### Requirement: Exported schemas include validation configuration

The `ito templates schemas export` command SHALL include `validation.yaml` (when present) for each exported schema directory.

- **Requirement ID**: cli-templates-schemas:export-validation-assets

#### Scenario: Export includes validation.yaml

- **GIVEN** a built-in schema directory contains `validation.yaml`
- **WHEN** the user runs `ito templates schemas export -f <target>`
- **THEN** the exported directory contains `validation.yaml` alongside `schema.yaml` and `templates/`

#### Scenario: Export remains deterministic

- **GIVEN** the export command runs twice with no embedded changes
- **WHEN** the user inspects the resulting `validation.yaml` files
- **THEN** their content is byte-for-byte identical between runs
<!-- ITO:END -->
