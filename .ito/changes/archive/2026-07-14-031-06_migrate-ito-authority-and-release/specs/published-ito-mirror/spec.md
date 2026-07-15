<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Published Ito mirror exposes coordination state to plain checkouts
The generated published mirror SHALL be removed because tracked `.ito` artifacts on `main` provide plain-checkout and GitHub visibility directly.
- **Requirement ID**: published-ito-mirror:plain-checkout-visibility
**Reason**: The mirror exists only to expose authority hidden in an external coordination checkout; after cutover it would duplicate committed canonical state.
**Migration**: Consumers SHALL read `.ito/changes` and `.ito/specs` on `main` after parity with `docs/ito` has been recorded.

#### Scenario: Plain checkout reads tracked authority
- **WHEN** a plain checkout or GitHub reader inspects Ito state on `main`
- **THEN** active changes, archived changes, and current specs are available under tracked `.ito` paths
- **AND** no generated mirror is required

### Requirement: Published Ito mirror defaults to docs slash ito and remains configurable
The system SHALL no longer define a default or configurable published-mirror output path.
- **Requirement ID**: published-ito-mirror:default-and-configurable-path
**Reason**: There is no mirror output after tracked `.ito` becomes authoritative.
**Migration**: Remove published-mirror path configuration and reject or warn on obsolete configuration according to the project's compatibility policy.

#### Scenario: No mirror path is resolved
- **WHEN** Ito resolves configuration after the cutover
- **THEN** it does not default to or generate `docs/ito`

### Requirement: Published Ito mirror is generated read-only output
The system SHALL no longer generate or repair a read-only mirror of Ito authority.
- **Requirement ID**: published-ito-mirror:generated-read-only-output
**Reason**: The tracked `.ito` tree is directly reviewable and has no derived writable competitor.
**Migration**: Complete the recorded parity audit, then remove mirror generation and direct current guidance to tracked `.ito` artifacts.

#### Scenario: Ito state changes update canonical files
- **WHEN** an approved proposal, implementation, or archive changes Ito state
- **THEN** the reviewed change updates tracked `.ito` files directly
- **AND** no mirror regeneration step follows

### Requirement: Publication workflow commits mirror content onto main
The mirror publication workflow SHALL be removed.
- **Requirement ID**: published-ito-mirror:main-publication-workflow
**Reason**: Canonical Ito artifacts are already committed to `main`, so a second publication workflow creates drift risk without adding visibility.
**Migration**: Remove mirror publication commands, workflows, tests, and documentation only after cutover parity succeeds.

#### Scenario: Main integration publishes canonical state once
- **WHEN** an Ito artifact change is reviewed and merged
- **THEN** the canonical tracked `.ito` content becomes visible on `main`
- **AND** no follow-up mirror publication commit is produced
<!-- ITO:END -->
