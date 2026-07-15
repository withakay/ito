<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Published mirror path configuration
The `ito-config` crate SHALL no longer expose or default a published Ito mirror path.
- **Requirement ID**: ito-config-crate:published-mirror-path
**Reason**: Tracked `.ito` artifacts on `main` replace the generated mirror and have no output path to configure.
**Migration**: Remove the obsolete field and generated schema entry after mirror parity is proven; legacy values receive the repository's standard obsolete-configuration diagnostic.

#### Scenario: Configuration resolves without a mirror path
- **WHEN** Ito loads configuration after the authority cutover
- **THEN** no published-mirror output path is resolved
- **AND** schema generation contains no published-mirror path setting
<!-- ITO:END -->
