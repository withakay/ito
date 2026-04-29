<!-- ITO:START -->
## ADDED Requirements

### Requirement: ito-update-repo skill includes a pre-commit hook setup step

The canonical `ito-update-repo` skill (source: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`) SHALL include a "Pre-commit hook setup" step that runs after the templates refresh and orphan cleanup. The step SHALL invoke `detect_pre_commit_system` (or document the equivalent inspection the agent performs), then propose the appropriate hook entry, then apply it after explicit user approval.

- **Requirement ID**: ito-update-repo-skill:pre-commit-step

#### Scenario: Skill detects an existing prek setup

- **GIVEN** the project's repo root contains `.pre-commit-config.yaml` and a prek toolchain marker
- **WHEN** the agent follows the `ito-update-repo` skill
- **THEN** the agent SHALL identify the system as `prek` in its plan
- **AND** the agent SHALL propose a `local` hook entry that runs `ito validate repo --staged --strict` at the `pre-commit` stage

#### Scenario: Skill detects a Husky setup

- **GIVEN** the project's repo root contains a `.husky/` directory
- **WHEN** the agent follows the `ito-update-repo` skill
- **THEN** the agent SHALL identify the system as `Husky`
- **AND** the agent SHALL propose creating or appending a `.husky/pre-commit` script that calls `ito validate repo --staged --strict`

#### Scenario: Skill handles an unrecognized setup

- **GIVEN** the project has no recognized pre-commit framework markers
- **WHEN** the agent follows the `ito-update-repo` skill
- **THEN** the agent SHALL report `none detected`
- **AND** the agent SHALL list the supported systems and ask the user to choose one rather than auto-installing a framework

### Requirement: Pre-commit hook setup is dry-run by default

The skill's pre-commit setup step SHALL present the proposed edit as a diff or summary and SHALL require explicit user approval (or a non-interactive `--yes` flag) before writing any change.

- **Requirement ID**: ito-update-repo-skill:dry-run-default

#### Scenario: Default behaviour is a preview

- **GIVEN** the user invokes `ito-update-repo` without `--yes`
- **WHEN** the agent reaches the pre-commit setup step
- **THEN** the agent SHALL print the proposed edit
- **AND** the agent SHALL wait for user approval before applying it

#### Scenario: --yes skips approval but still verifies

- **GIVEN** the user invokes `ito-update-repo --yes`
- **WHEN** the agent reaches the pre-commit setup step
- **THEN** the agent SHALL apply the edit without prompting
- **AND** the agent SHALL still run the verification step afterwards

### Requirement: Pre-commit hook setup is verified after install

After applying the pre-commit hook entry, the skill SHALL run `ito validate repo --staged --strict` once and SHALL surface the exit code in its summary so the user knows whether the hook is functional.

- **Requirement ID**: ito-update-repo-skill:verify-after-install

#### Scenario: Verification reports exit 0 on a clean repo

- **GIVEN** the pre-commit hook entry has just been applied
- **AND** the repository state matches the resolved configuration
- **WHEN** the skill runs the verification step
- **THEN** the verification SHALL exit 0
- **AND** the skill summary SHALL state "pre-commit hook installed and verified"

#### Scenario: Verification reports the failing rules

- **GIVEN** the pre-commit hook entry has just been applied
- **AND** the repository has at least one configuration drift issue
- **WHEN** the skill runs the verification step
- **THEN** the verification SHALL exit non-zero
- **AND** the skill summary SHALL list the failing rules so the user knows what to fix

### Requirement: Harness command shells reflect the pre-commit setup scope

The harness-equivalent command shells for `ito-update-repo` (under `assets/commands/`, `.opencode/commands/`, `.claude/commands/`, `.codex/prompts/`, `.github/prompts/`, `.pi/commands/`) SHALL mention pre-commit hook setup in their description or notes block so users discover it from the harness command palette.

- **Requirement ID**: ito-update-repo-skill:harness-discoverability

#### Scenario: Harness command description mentions pre-commit setup

- **WHEN** any harness's `ito-update-repo` command shell is read
- **THEN** its description or notes block SHALL mention pre-commit hook setup as part of the skill's scope
<!-- ITO:END -->
