<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Ito Update Repo Skill
The system SHALL provide an `ito-update-repo` skill and matching harness command that refreshes Ito-managed files and audits orphan assets.

**Reason**: Update is a direct CLI responsibility and the standalone helper duplicates the retained root `ito` lifecycle guidance.
**Migration**: Use `ito update` or `ito init --upgrade`, inspect their ownership-aware cleanup report, and run `ito validate repo` directly.

#### Scenario: Retired update helper is requested
- **WHEN** a user requests `ito-update-repo`
- **THEN** retained `ito` guidance explains the direct update and validation commands
- **AND** no replacement skill is installed

### Requirement: Distribution via Templates Bundle
The templates bundle SHALL distribute `ito-update-repo` to every configured harness.

**Reason**: The canonical default contains exactly seven lifecycle skills and does not include `ito-update-repo`.
**Migration**: Remove managed skill and command copies during ownership-aware upgrade cleanup; preserve user-authored content.

#### Scenario: Fresh installation omits the retired helper
- **WHEN** Ito initializes any supported harness
- **THEN** no `ito-update-repo` skill or command wrapper is emitted

### Requirement: ito-update-repo skill includes a pre-commit hook setup step
The `ito-update-repo` skill SHALL include a pre-commit hook setup step after managed asset refresh and cleanup.

**Reason**: The standalone update-repo skill is retired and pre-commit framework setup is not part of the seven lifecycle entrypoints.
**Migration**: Configure downstream pre-commit hooks explicitly using reference documentation and verify them with direct CLI validation.

#### Scenario: Hook setup remains explicit
- **WHEN** a downstream project adopts the Ito validation hook
- **THEN** the user or project tooling reviews and applies the change explicitly

### Requirement: Pre-commit hook setup is dry-run by default
The skill's pre-commit setup step SHALL preview edits and require approval unless an explicit non-interactive option is supplied.

**Reason**: Ito no longer uses a managed skill to edit third-party pre-commit framework configuration.
**Migration**: Preview and apply hook changes with the repository's chosen tooling, then run `ito validate repo --staged --strict` directly.

#### Scenario: Direct hook configuration remains reviewable
- **WHEN** a downstream project changes its hook configuration
- **THEN** the change follows the project's normal review workflow

### Requirement: Pre-commit hook setup is verified after install
The skill SHALL run staged repository validation after applying a hook entry.

**Reason**: Verification is no longer owned by a retired skill.
**Migration**: Run `ito validate repo --staged --strict` directly after hook configuration.

#### Scenario: Manual verification remains available
- **WHEN** hook configuration is changed
- **THEN** direct Ito validation reports success or actionable rule failures

### Requirement: Harness command shells reflect the pre-commit setup scope
Harness command shells for `ito-update-repo` SHALL advertise the pre-commit setup behavior.

**Reason**: Harness command shells for the retired skill are removed with the skill.
**Migration**: Discover update and validation behavior through `ito`, CLI help, and lifecycle/reference documentation.

#### Scenario: Fresh install omits retired command shell
- **WHEN** harness manifests are generated
- **THEN** no `ito-update-repo` command or prompt shell is installed
<!-- ITO:END -->
