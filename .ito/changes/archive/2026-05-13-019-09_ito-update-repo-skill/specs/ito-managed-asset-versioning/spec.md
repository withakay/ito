## ADDED Requirements

### Requirement: Every Ito Markdown File Has A Managed Block

Every markdown file that `ito init` or `ito init --update` writes into a project SHALL contain an Ito-managed block delimited by `<!-- ITO:START -->` and `<!-- ITO:END -->`. The managed block wraps the Ito-owned content; any YAML frontmatter at the top of the file SHALL remain outside (above) the managed block so that frontmatter parsers are not disturbed.

- **Requirement ID**: ito-managed-asset-versioning:managed-block-everywhere

#### Scenario: Template bundle compliance

- **WHEN** a maintainer adds a markdown file under `ito-rs/crates/ito-templates/assets/` (skills, commands, agents, prompts, instruction artifacts, schemas, default project files)
- **THEN** the file SHALL contain exactly one `<!-- ITO:START -->` marker and exactly one `<!-- ITO:END -->` marker
- **AND** a CI test SHALL fail if any shipped markdown lacks the pair

#### Scenario: Frontmatter preserved

- **GIVEN** a skill file begins with a YAML frontmatter block (`---` / `---`)
- **WHEN** the file is wrapped with managed markers
- **THEN** the frontmatter SHALL remain above `<!-- ITO:START -->`
- **AND** the rest of the original body SHALL appear between the markers unchanged

#### Scenario: Non-markdown files are out of scope

- **GIVEN** a YAML, JSON, or shell script asset in the templates bundle
- **WHEN** the managed-block requirement is evaluated
- **THEN** the requirement SHALL NOT apply to that file
- **AND** such files MAY gain managed-block or stamping support in a future change

### Requirement: Generator Version Stamp

Every file `ito init` or `ito init --update` writes that contains a managed block SHALL embed the version of the Ito CLI that produced it so that downstream tooling and humans can identify stale or mismatched content.

- **Requirement ID**: ito-managed-asset-versioning:stamp-every-output

#### Scenario: Version recorded on install

- **WHEN** `ito init --tools all` writes a file that contains `<!-- ITO:START -->`
- **THEN** the file SHALL include the Ito CLI version that wrote it
- **AND** the recorded version SHALL be the same value reported by `ito --version`

#### Scenario: Version refreshed on update

- **GIVEN** a project already has a managed-block file stamped with an older version
- **WHEN** `ito init --update --tools all` refreshes that file
- **THEN** the stamp in the file SHALL be updated to the current CLI version

#### Scenario: Files without managed blocks are untouched

- **GIVEN** a template file that contains no `<!-- ITO:START -->` marker
- **WHEN** Ito runs init or update
- **THEN** no version stamp SHALL be written to that file

#### Scenario: User-authored files are not stamped

- **GIVEN** a file outside the Ito-managed directories or outside the managed markers
- **WHEN** Ito runs init or update
- **THEN** no version stamp SHALL be written to that file

### Requirement: Stamp Format And Location

The version stamp SHALL be a single-line HTML comment of the form `<!--ITO:VERSION:<semver>-->` (optionally with whitespace around or between tokens, e.g. `<!-- ITO:VERSION: 1.2.3 -->`). The Ito writer SHALL emit exactly one canonical form consistently across every file it produces; readers SHALL tolerate any of the permitted whitespace variants. The stamp SHALL live on its own line inside the managed block and SHALL be stable across runs when the version has not changed.

- **Requirement ID**: ito-managed-asset-versioning:stamp-format

#### Scenario: Canonical writer output

- **WHEN** Ito writes a managed-block file with the version `1.2.3-asd`
- **THEN** the file SHALL contain the line `<!--ITO:VERSION:1.2.3-asd-->` as emitted by the writer (the canonical tight form)
- **AND** every managed-block file produced by the same build of Ito SHALL use the same whitespace shape — the writer SHALL NOT mix tight and spaced forms across outputs

#### Scenario: Reader tolerates whitespace variants

- **GIVEN** a managed-block file whose existing stamp uses a spaced form such as `<!-- ITO:VERSION: 1.2.3 -->`
- **WHEN** Ito tooling (orphan audit, `--upgrade`, staleness detector) parses the stamp
- **THEN** the tooling SHALL match the semver against the case-sensitive regex `<!--\s*ITO:VERSION:\s*([^>\s]+)\s*-->` and extract the version correctly

#### Scenario: Location within the managed block

- **WHEN** Ito writes a managed-block file
- **THEN** the version-stamp line SHALL appear on the line immediately following `<!-- ITO:START -->`
- **AND** tooling locating or rewriting the stamp SHALL find it on that line

#### Scenario: Stamp is idempotent

- **GIVEN** a managed-block file already stamped with the current version in the canonical writer form
- **WHEN** `ito init --update` runs
- **THEN** the stamp SHALL NOT change
- **AND** the file's content SHALL be byte-identical to its pre-update state (aside from any other intentional template refresh)

#### Scenario: Non-canonical existing stamp is rewritten

- **GIVEN** a managed-block file carries the current semver in a non-canonical whitespace form (e.g. `<!-- ITO:VERSION: 1.2.3 -->` while the writer emits the tight form)
- **WHEN** `ito init --update` runs
- **THEN** the stamp SHALL be rewritten to the canonical form
- **AND** subsequent `ito init --update` runs SHALL produce no further changes

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
