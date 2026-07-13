<!-- ITO:START -->
## REMOVED Requirements

### Requirement: ito init emits a repo-validation advisory when at least one rule activates
After primary initialization work, Ito SHALL emit the configured repo-validation advisory when an applicable rule activates.

**Reason**: The advisory delegates setup to the retired `ito-update-repo` skill and adds non-core post-install workflow branching.
**Migration**: `ito` lifecycle guidance and CLI help document `ito validate repo`; projects opt into hook configuration explicitly.

#### Scenario: Init completes without helper-skill advisory
- **WHEN** `ito init` or `ito init --upgrade` completes
- **THEN** it does not recommend a retired helper skill
- **AND** direct validation commands remain available

### Requirement: Advisory names the detected pre-commit system
The advisory SHALL name the detected pre-commit framework when it is emitted.

**Reason**: Ito init no longer delegates third-party hook setup through a helper-skill advisory.
**Migration**: Pre-commit system detection, if retained for diagnostics, is invoked explicitly by validation/setup tooling rather than printed after every init.

#### Scenario: No automatic framework advisory
- **WHEN** init detects any pre-commit framework
- **THEN** it does not emit a helper-skill setup advisory

### Requirement: Advisory references the ito-update-repo skill rather than a new slash command
The advisory SHALL direct the user to the `ito-update-repo` skill rather than introducing another setup command.

**Reason**: `ito-update-repo` and its slash-command shells are retired.
**Migration**: Use the retained `ito` skill or direct `ito update` and `ito validate repo` commands.

#### Scenario: Retired name is absent
- **WHEN** init output is rendered
- **THEN** it does not contain `ito-update-repo`
<!-- ITO:END -->
