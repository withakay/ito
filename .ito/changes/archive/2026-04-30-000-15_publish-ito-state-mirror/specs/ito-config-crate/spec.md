<!-- ITO:START -->
## ADDED Requirements

### Requirement: Published mirror path configuration

The `ito-config` crate SHALL provide configuration for the published Ito mirror path, defaulting to `docs/ito` when the project does not override it.

- **Requirement ID**: ito-config-crate:published-mirror-path

#### Scenario: Published mirror path defaults to docs slash ito

- **WHEN** the project omits published mirror path configuration
- **THEN** the resolved published mirror path is `docs/ito`

#### Scenario: Published mirror path can be overridden

- **WHEN** the project config sets a custom published mirror path
- **THEN** Ito resolves that configured path instead of `docs/ito`
<!-- ITO:END -->
