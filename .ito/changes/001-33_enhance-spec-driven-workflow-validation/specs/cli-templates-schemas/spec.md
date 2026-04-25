<!-- ITO:START -->
## ADDED Requirements

These requirements keep built-in workflow schema templates aligned with the validators that Ito actually runs.

### Requirement: Built-in schema templates match configured validators

Built-in workflow schema templates MUST use an artifact format that is accepted by the validators declared in the same schema directory's `validation.yaml`.

- **Requirement ID**: cli-templates-schemas:template-validator-alignment

#### Scenario: Minimalist specs parse as deltas

- **GIVEN** the built-in minimalist schema declares `specs` validation as `ito.delta-specs.v1`
- **WHEN** its spec template is rendered for a new change
- **THEN** the rendered template uses delta requirement headers and `### Requirement:` blocks rather than `## Stories` and `### Story:` blocks

#### Scenario: Event-driven specs parse as deltas

- **GIVEN** the built-in event-driven schema declares `specs` validation as `ito.delta-specs.v1`
- **WHEN** its spec template is rendered for a new change
- **THEN** the rendered template uses delta requirement headers and `### Requirement:` blocks rather than `## Stories` and `### Story:` blocks

#### Scenario: Export includes validation configuration

- **GIVEN** a built-in schema directory contains `validation.yaml`
- **WHEN** the user runs `ito templates schemas export -f <target>`
- **THEN** the exported schema directory contains `validation.yaml` alongside `schema.yaml` and templates

#### Scenario: Exported templates validate strictly

- **GIVEN** exported built-in schema templates are rendered into a valid sample change
- **WHEN** `ito validate <change-id> --strict` runs for that sample
- **THEN** validation does not fail because a template uses a format incompatible with its configured validators
<!-- ITO:END -->
