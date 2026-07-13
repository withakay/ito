<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Experimental Workflow Slash Commands
The system SHALL expose the experimental workflow via hyphenated `/ito-*` slash commands and SHALL NOT use `/opsx:*`.

**Reason**: The experimental setup command is absent and its wrapper list conflicts with the canonical seven lifecycle commands.
**Migration**: Use `ito-proposal`, `ito-apply`, `ito-review`, and `ito-archive`, or direct Ito CLI operations.

#### Scenario: Canonical lifecycle replaces experimental wrappers
- **WHEN** a fresh or updated installation is inspected
- **THEN** it exposes the seven lifecycle wrappers
- **AND** does not advertise experimental workflow wrappers

### Requirement: Claude Command File Generation
The system SHALL generate Claude command wrapper files as flat files under `.claude/commands/` using the `ito-*.md` naming convention.

**Reason**: Ito no longer generates the experimental wrapper set.
**Migration**: Use the seven shared lifecycle wrappers installed consistently across harnesses.

#### Scenario: Claude receives canonical wrappers only
- **WHEN** Ito initializes or updates Claude assets
- **THEN** it installs exactly the seven lifecycle command files
<!-- ITO:END -->
