<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Planning slash command installation
The system SHALL install an `ito-plan` slash-command wrapper that loads a dedicated `ito-plan` skill.

**Reason**: Planning is consolidated into the retained `ito-proposal` lifecycle entrypoint, so a separate command and skill would violate the exact default inventory.
**Migration**: Invoke `ito-proposal` for exploratory planning or use `ito plan` direct CLI workspace commands where appropriate.

#### Scenario: Fresh install omits planning wrapper
- **WHEN** a supported harness installs or refreshes Ito-managed command assets
- **THEN** it does not emit an `ito-plan` command wrapper
- **AND** planning remains reachable through `ito-proposal`
<!-- ITO:END -->
