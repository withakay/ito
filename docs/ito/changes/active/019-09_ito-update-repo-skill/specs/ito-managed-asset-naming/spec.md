## ADDED Requirements

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

The `ito-templates` crate SHALL not ship any skill, command, prompt, or agent asset that violates the prefix rule.

- **Requirement ID**: ito-managed-asset-naming:templates-enforce

#### Scenario: Pre-existing unprefixed assets are renamed

- **GIVEN** the templates bundle previously shipped `skills/tmux/`, `skills/test-with-subagent/`, `skills/using-ito-skills/`, and `agents/opencode/test-runner.md`
- **WHEN** this change is applied
- **THEN** they SHALL be renamed to `skills/ito-tmux/`, `skills/ito-test-with-subagent/`, `skills/ito-using-ito-skills/`, and `agents/opencode/ito-test-runner.md`
- **AND** any internal reference inside those assets SHALL be updated to the new name

#### Scenario: CI guard

- **WHEN** a contributor adds a new file under `ito-rs/crates/ito-templates/assets/{skills,commands,agents}/`
- **AND** the basename does not satisfy the prefix rule
- **THEN** a test or guard SHALL fail the build
