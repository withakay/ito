<!-- ITO:START -->

## Requirements

### Requirement: Ito-Managed Asset Prefix

Every Ito-managed asset distributed through the `ito-templates` bundle — skills, commands, prompts, and agents — SHALL have a basename that begins with `ito-`, with the sole exception of the bare root entrypoint named `ito` (for example, `skills/ito/` and `commands/ito.md`).

- **Requirement ID**: ito-managed-asset-naming:prefix-rule

#### Scenario: Shared skill naming

- **WHEN** a skill is added to `ito-rs/crates/ito-templates/assets/skills/`
- **THEN** its directory name SHALL match the pattern `ito` or `ito-<suffix>`
- **AND** a directory named anything else SHALL be treated as a spec violation

#### Scenario: Shared command naming

- **WHEN** a command file is added to `ito-rs/crates/ito-templates/assets/commands/`
- **THEN** its basename SHALL match `ito.md` or `ito-<suffix>.md`

#### Scenario: Shared agent naming

- **WHEN** an agent file is added to any harness agent directory under `ito-rs/crates/ito-templates/assets/agents/<harness>/`
- **THEN** its basename SHALL match `ito-<suffix>.md`

#### Scenario: Root entrypoint exemption

- **GIVEN** the root Ito entrypoint skill and command
- **WHEN** a reader checks naming compliance
- **THEN** `skills/ito/` and `commands/ito.md` SHALL be accepted as compliant

### Requirement: Prefix-Driven Orphan Detection

Tooling that identifies orphan Ito assets in a project (for example the `ito-update-repo` skill) SHALL treat any asset whose basename begins with `ito-` (or is exactly `ito`) in a harness-managed directory as Ito-owned, and SHALL ignore assets without the prefix as user- or third-party-owned.

- **Requirement ID**: ito-managed-asset-naming:prefix-drives-ownership

#### Scenario: Unprefixed asset is left alone

- **GIVEN** a harness directory contains a skill whose basename does not start with `ito-`
- **WHEN** the orphan audit runs
- **THEN** the asset SHALL NOT be reported as an orphan
- **AND** SHALL NOT be considered for deletion

#### Scenario: Prefixed asset absent from templates is an orphan

- **GIVEN** a harness directory contains a skill whose basename starts with `ito-`
- **AND** the same basename is not present in the current Ito templates bundle
- **WHEN** the orphan audit runs
- **THEN** the asset SHALL be reported as an orphan candidate

### Requirement: Enforce Prefix in Templates Bundle
The `ito-templates` crate SHALL ship exactly the canonical seven Ito-managed skill directories and SHALL enforce Ito naming rules for other managed command, prompt, and native-agent assets. It MUST NOT preserve obsolete prefixed helpers merely because their names satisfy the prefix rule.

#### Scenario: Canonical lifecycle skills pass the bundle guard
- **WHEN** embedded skill assets are audited
- **THEN** the Ito-managed skill names are exactly `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`
- **AND** every noncanonical helper skill fails the exact-inventory guard whether prefixed or not

#### Scenario: Native agents are checked separately
- **WHEN** embedded native-agent assets are audited
- **THEN** their names follow the applicable Ito prefix convention
- **AND** those native agents are not counted or installed as skills

#### Scenario: Retired tmux helpers are absent
- **WHEN** the templates bundle is built
- **THEN** it contains no `tmux` or `ito-tmux` skill directory or helper scripts
<!-- ITO:END -->
