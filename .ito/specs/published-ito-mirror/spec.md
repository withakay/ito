<!-- ITO:START -->
## ADDED Requirements

### Requirement: Published Ito mirror exposes coordination state to plain checkouts

The system SHALL provide a published, read-only mirror of Ito state so a plain checkout of `main` or a GitHub browser session can inspect active changes, archived changes, and canonical specs without relying on `.ito/...` symlinks into a coordination worktree.

- **Requirement ID**: published-ito-mirror:plain-checkout-visibility

#### Scenario: Plain checkout can inspect active changes

- **WHEN** a plain checkout or GitHub reader opens the published Ito mirror on `main`
- **THEN** the mirror includes active changes from the coordination-backed Ito state
- **AND** the reader does not need Ito-installed symlink wiring to see them

#### Scenario: Plain checkout can inspect canonical specs

- **WHEN** a plain checkout or GitHub reader opens the published Ito mirror on `main`
- **THEN** the mirror includes canonical specs in a committed read-only form

#### Scenario: Plain checkout can inspect archived changes

- **WHEN** a plain checkout or GitHub reader opens the published Ito mirror on `main`
- **THEN** the mirror includes archived change visibility in committed form

### Requirement: Published Ito mirror defaults to docs slash ito and remains configurable

The published Ito mirror SHALL write to `docs/ito` by default and SHALL support a project-configured override path.

- **Requirement ID**: published-ito-mirror:default-and-configurable-path

#### Scenario: Default mirror path is docs slash ito

- **WHEN** the project does not configure a custom published mirror path
- **THEN** the published mirror is emitted under `docs/ito`

#### Scenario: Configured mirror path overrides the default

- **WHEN** the project config sets a custom published mirror path
- **THEN** the published mirror is emitted to that configured path instead of `docs/ito`

### Requirement: Published Ito mirror is generated read-only output

The published Ito mirror SHALL be treated as generated, read-only output derived from coordination-backed Ito state, and direct edits to the mirror SHALL be treated as drift rather than as a second writable source of truth.

- **Requirement ID**: published-ito-mirror:generated-read-only-output

#### Scenario: Direct mirror edits are drift

- **WHEN** a user or tool edits the published mirror directly
- **THEN** the system treats those edits as drift from generated output
- **AND** the workflow provides regeneration or repair guidance instead of treating the mirror as authoritative

#### Scenario: Coordination state remains authoritative

- **WHEN** published mirror content disagrees with coordination-backed Ito state
- **THEN** the coordination-backed state wins
- **AND** the mirror is refreshed from the coordination-backed source

### Requirement: Publication workflow commits mirror content onto main

The system SHALL provide a publication workflow that moves the generated mirror content onto `main` as committed files, so visibility does not depend on the local presence of a coordination worktree.

- **Requirement ID**: published-ito-mirror:main-publication-workflow

#### Scenario: Publication updates main-facing mirror content

- **WHEN** the publication workflow runs successfully
- **THEN** the mirror content is updated on `main` as committed files
- **AND** plain checkouts of `main` can read the refreshed mirror

#### Scenario: Publication does not require canonical .ito authoring

- **WHEN** the publication workflow updates the mirror
- **THEN** it does not make canonical `.ito/changes` or `.ito/specs` the writable authoring surface on `main`
- **AND** the coordination-backed Ito state remains the only writable source of truth
<!-- ITO:END -->
