## ADDED Requirements

### Requirement: Generator Version Stamp

Every Ito-managed asset that `ito init` or `ito init --update` writes into a project — skills, commands, prompts, agents, instruction templates, generated configuration, and any markdown or YAML/JSON file inside a managed block — SHALL embed the version of the Ito CLI that produced it so that downstream tooling and humans can identify stale or mismatched content.

- **Requirement ID**: ito-managed-asset-versioning:stamp-every-output

#### Scenario: Version recorded on install

- **WHEN** `ito init --tools all` writes a managed file
- **THEN** the file SHALL include the Ito CLI version that wrote it
- **AND** the recorded version SHALL be the same value reported by `ito --version`

#### Scenario: Version refreshed on update

- **GIVEN** a project already has managed files stamped with an older version
- **WHEN** `ito init --update --tools all` refreshes a managed file
- **THEN** the stamp in that file SHALL be updated to the current CLI version

#### Scenario: User-authored files are not stamped

- **GIVEN** a file outside the Ito-managed directories or outside the managed markers
- **WHEN** Ito runs init or update
- **THEN** no version stamp SHALL be written to that file

### Requirement: Stamp Format And Location

The version stamp SHALL be machine-parseable, SHALL live inside the Ito-managed region of the file, and SHALL be stable across runs when the version has not changed.

- **Requirement ID**: ito-managed-asset-versioning:stamp-format

#### Scenario: Markdown files carry a commented stamp

- **WHEN** Ito writes a managed markdown file (skill, command, prompt, instruction)
- **THEN** the managed block SHALL contain an HTML comment of the form `<!-- ITO:VERSION: <semver> -->`
- **AND** the comment SHALL appear immediately adjacent to the `<!-- ITO:START -->` marker (either on the following line or inline with it) so that `ito init --upgrade` can locate and rewrite it deterministically

#### Scenario: YAML/JSON configuration files carry a structured stamp

- **WHEN** Ito writes a managed YAML or JSON file (for example `.ito/config.json` managed regions or preset files)
- **THEN** the file SHALL contain an `ito_version` field (YAML) or `"ito_version"` key (JSON) inside the managed region
- **AND** the value SHALL be the semver string reported by `ito --version`

#### Scenario: Stamp is idempotent

- **GIVEN** a managed file already stamped with the current version
- **WHEN** `ito init --update` runs
- **THEN** the stamp SHALL NOT change
- **AND** the file's content SHALL be byte-identical to its pre-update state (aside from any other intentional template refresh)

### Requirement: Stamp Exposed Through Tooling

Ito tooling SHALL be able to read the stamp from a managed file without parsing the full document, so orphan detection and staleness reports can surface version drift cheaply.

- **Requirement ID**: ito-managed-asset-versioning:stamp-readable

#### Scenario: Stale-version detection in `ito-update-repo`

- **GIVEN** a harness skill carries an `ITO:VERSION` stamp older than the currently installed CLI
- **WHEN** the `ito-update-repo` skill audits the project after running the update step
- **THEN** the skill SHALL report any file whose stamp is older than the current CLI version as "stale"
- **AND** SHALL distinguish stale-but-still-valid assets from orphaned (removed-upstream) assets in its report

#### Scenario: Missing stamp surfaces as stale

- **GIVEN** a managed file is present but carries no `ITO:VERSION` stamp
- **WHEN** staleness detection runs
- **THEN** the file SHALL be reported as stale with reason `missing-stamp`
- **AND** the user SHALL be offered the option to re-run `ito init --update` to stamp it

### Requirement: Version Stamping Does Not Leak User Metadata

The stamp SHALL record only the CLI version string. It SHALL NOT include usernames, hostnames, timestamps, environment variables, or any other identifying metadata.

- **Requirement ID**: ito-managed-asset-versioning:privacy

#### Scenario: Stamp contains only semver

- **WHEN** Ito writes a stamp into any managed file
- **THEN** the stamp SHALL contain only the semver string produced by `ito --version`
- **AND** SHALL NOT contain any additional fields
