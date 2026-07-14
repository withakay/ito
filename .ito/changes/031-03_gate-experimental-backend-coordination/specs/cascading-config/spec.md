<!-- ITO:START -->
## ADDED Requirements

### Requirement: Configuration parsing is independent of compiled features

Ito SHALL retain backend and coordination configuration DTOs, serde behavior, schema definitions, and cascading merge behavior in every build. Feature selection SHALL control implementation availability, not whether existing project configuration can be parsed and diagnosed.

- **Requirement ID**: cascading-config:parse-compiled-out-features

#### Scenario: Default binary parses legacy feature configuration

- **GIVEN** a project configuration contains recognized backend and coordination fields
- **AND** the active binary was built without those features
- **WHEN** Ito loads cascading configuration
- **THEN** deserialization succeeds for the recognized fields
- **AND** capability preflight can report which compiled-out feature was requested

#### Scenario: Unknown configuration remains distinguishable

- **GIVEN** a configuration contains an invalid or unknown field value
- **WHEN** Ito loads the configuration in a default or experimental build
- **THEN** Ito reports the existing configuration validation error
- **AND** does not misclassify the invalid value as a feature-unavailable error
<!-- ITO:END -->
