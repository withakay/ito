<!-- ITO:START -->
## ADDED Requirements

### Requirement: Embedded schemas ship validation.yaml

Ito SHALL ship a `validation.yaml` file alongside each embedded workflow schema's `schema.yaml` when that schema is intended to be validated automatically by `ito validate`.

#### Scenario: Embedded schema directory includes validation.yaml

- **GIVEN** a schema is embedded in the Ito binary
- **WHEN** the embedded schema includes `schema.yaml`
- **THEN** the embedded schema directory SHALL also include `validation.yaml`
- **AND** `ito validate <change-id>` SHALL report that it is using schema validation
<!-- ITO:END -->
