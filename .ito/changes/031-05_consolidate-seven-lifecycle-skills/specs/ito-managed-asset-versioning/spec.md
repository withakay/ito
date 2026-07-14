<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Stamp Exposed Through Tooling
Ito tooling SHALL read managed version stamps without parsing complete documents so direct update, validation, and cleanup operations can distinguish current, stale, and retired assets. This diagnostic contract MUST NOT depend on the retired `ito-update-repo` skill.

#### Scenario: Direct update reports a stale managed asset
- **GIVEN** a managed harness asset carries an `ITO:VERSION` older than the installed CLI
- **WHEN** `ito update` or `ito init --upgrade` audits managed assets
- **THEN** the operation reports the asset as stale
- **AND** distinguishes a still-valid retained asset from an obsolete managed path

#### Scenario: Missing stamp remains diagnosable
- **WHEN** a known Ito-managed path lacks a readable version stamp
- **THEN** direct update or validation tooling reports the missing ownership/version evidence
- **AND** does not delete user content unless managed ownership is otherwise proven
<!-- ITO:END -->
