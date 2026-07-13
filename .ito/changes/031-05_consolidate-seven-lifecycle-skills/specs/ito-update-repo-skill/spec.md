<!-- ITO:START -->
## REMOVED Requirements

### Requirement: ito-update-repo skill includes a pre-commit hook setup step
The `ito-update-repo` skill SHALL include a pre-commit hook setup step after managed asset refresh and cleanup.

**Reason**: The standalone update-repo skill is retired and pre-commit framework setup is not part of the seven lifecycle entrypoints.
**Migration**: Use direct `ito init --upgrade` or `ito update` for managed assets and configure downstream pre-commit hooks explicitly using reference documentation.

#### Scenario: Retired setup skill is requested
- **WHEN** a user requests `ito-update-repo`
- **THEN** retained `ito` guidance points to direct update and validation commands

### Requirement: Pre-commit hook setup is dry-run by default
The skill's pre-commit setup step SHALL preview edits and require approval unless an explicit non-interactive option is supplied.

**Reason**: Ito no longer uses a managed skill to edit third-party pre-commit framework configuration.
**Migration**: Preview and apply hook changes with the repository's chosen tooling, then run `ito validate repo --staged --strict` directly.

#### Scenario: Hook setup remains explicit
- **WHEN** a downstream project adopts the Ito validation hook
- **THEN** the user or project tooling reviews and applies the change explicitly

### Requirement: Pre-commit hook setup is verified after install
The skill SHALL run staged repository validation after applying a hook entry.

**Reason**: Verification is no longer owned by a retired skill.
**Migration**: Run `ito validate repo --staged --strict` directly after manual hook configuration.

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
