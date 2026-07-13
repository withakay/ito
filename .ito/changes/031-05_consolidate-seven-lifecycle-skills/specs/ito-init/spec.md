<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: ito init emits a repo-validation advisory when at least one rule activates
After `ito init` and `ito init --upgrade` complete their primary work, the system SHALL emit a post-install advisory only when the resolved configuration activates at least one `ito validate repo` rule. The advisory SHALL name direct validation or instruction commands and MUST NOT delegate remediation to a retired helper skill.

#### Scenario: Active rule produces direct remediation
- **WHEN** initialization completes with at least one active repository-validation rule
- **THEN** the advisory names `ito validate repo`
- **AND** it identifies the direct CLI or emitted instruction that owns remediation
- **AND** it does not recommend `ito-update-repo`

#### Scenario: No active rule remains quiet
- **WHEN** initialization completes with no active repository-validation rule
- **THEN** no validation advisory is printed

## REMOVED Requirements

### Requirement: Advisory references the ito-update-repo skill rather than a new slash command
The advisory SHALL direct the user to invoke the existing `ito-update-repo` skill or slash-command wrapper.

**Reason**: `ito-update-repo` and its harness command shells are retired from the canonical seven-skill surface.
**Migration**: Name `ito validate repo`, `ito update`, or the specific CLI-emitted remediation instruction directly.

#### Scenario: Retired helper name is absent
- **WHEN** `ito init` emits a repository-validation advisory
- **THEN** the message does not contain `ito-update-repo`
- **AND** it names the direct supported remediation path
<!-- ITO:END -->
