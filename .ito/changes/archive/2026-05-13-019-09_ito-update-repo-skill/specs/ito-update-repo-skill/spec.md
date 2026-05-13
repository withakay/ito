## ADDED Requirements

### Requirement: Ito Update Repo Skill

The system SHALL provide an `ito-update-repo` skill (and a matching `/ito-update-repo` command) distributed in the Ito templates bundle that refreshes Ito-managed files in a project and audits harness directories for orphan skills, commands, and prompts.

- **Requirement ID**: ito-update-repo-skill:skill-entrypoint

#### Scenario: Invoked without arguments

- **WHEN** the agent loads the `ito-update-repo` skill with no arguments
- **THEN** the skill SHALL run `ito init --update --tools all` without `--force`
- **AND** report the list of orphan skills and commands grouped by harness directory
- **AND** ask the user for approval before deleting any entry

#### Scenario: Invoked with `--dry-run`

- **WHEN** the agent loads the skill with `--dry-run` in its arguments
- **THEN** the skill SHALL perform the update and orphan audit
- **AND** SHALL NOT delete any files
- **AND** SHALL print the list of orphans that would be removed

### Requirement: Non-Destructive By Default

The skill SHALL default to non-destructive update semantics and never pass `--force` to `ito init` unless the user explicitly requests it.

- **Requirement ID**: ito-update-repo-skill:non-destructive

#### Scenario: Default invocation

- **WHEN** the skill runs the update step without a user-supplied `--force` flag
- **THEN** the invoked command SHALL be `ito init --update --tools all`
- **AND** SHALL NOT include `--force`

#### Scenario: User-edited file preserved

- **GIVEN** a harness skill file has been edited outside the Ito-managed markers
- **WHEN** the skill runs the update step
- **THEN** the user-edited content outside managed markers SHALL be preserved

### Requirement: Orphan Audit Across Harnesses

The skill SHALL compare each harness skill directory and each harness command/prompt directory against the asset manifest installed by the current Ito binary, and SHALL flag any entry whose basename is not present in that manifest.

- **Requirement ID**: ito-update-repo-skill:orphan-audit

#### Scenario: Skill renamed in a newer release

- **GIVEN** the project contains `.opencode/skills/ito-write-change-proposal/`
- **AND** the current Ito templates ship `ito-proposal` as its replacement
- **WHEN** the skill runs the orphan audit
- **THEN** `.opencode/skills/ito-write-change-proposal/` SHALL appear in the orphan report
- **AND** the report SHALL indicate that `ito-proposal` is the current replacement

#### Scenario: All harness roots are audited

- **WHEN** the skill runs the orphan audit
- **THEN** each of `.claude/skills/`, `.codex/skills/`, `.github/skills/`, `.opencode/skills/`, and `.pi/skills/` SHALL be scanned
- **AND** each of `.claude/commands/`, `.codex/prompts/`, `.github/prompts/`, `.opencode/commands/`, and `.pi/commands/` SHALL be scanned

### Requirement: Approval Gate Before Deletion

The skill SHALL NOT delete any orphan entry until it has either received explicit user approval or the user passed `--yes`/`-y` in the skill arguments.

- **Requirement ID**: ito-update-repo-skill:approval-gate

#### Scenario: User approves selected orphans

- **GIVEN** the skill has produced an orphan report
- **WHEN** the user approves a subset of the listed orphans
- **THEN** only the approved entries SHALL be deleted
- **AND** unapproved entries SHALL remain on disk untouched

#### Scenario: User aborts

- **GIVEN** the skill has produced an orphan report
- **WHEN** the user declines to approve any deletions
- **THEN** no files SHALL be deleted
- **AND** the skill SHALL exit reporting the update was applied but no cleanup was performed

#### Scenario: `--keep` list respected

- **GIVEN** the user passes `--keep repo-local-skill` in the arguments
- **WHEN** the orphan audit finds `repo-local-skill` in a harness directory
- **THEN** the skill SHALL treat it as kept
- **AND** SHALL NOT include it in the orphan report

### Requirement: Rerun Idempotence

After the cleanup step completes, running the skill again SHALL produce no further file changes from the update step.

- **Requirement ID**: ito-update-repo-skill:rerun-idempotent

#### Scenario: Second invocation is stable

- **GIVEN** the skill has completed a successful update and cleanup
- **WHEN** the user invokes the skill a second time
- **THEN** the update step SHALL report no file modifications
- **AND** the orphan audit SHALL report zero orphans

### Requirement: Distribution via Templates Bundle

The skill and its command wrapper SHALL live in the `ito-templates` crate's embedded assets so that `ito init` and `ito init --update` install them into every configured harness.

- **Requirement ID**: ito-update-repo-skill:distribution

#### Scenario: Installed on fresh init

- **GIVEN** a project has never had Ito installed
- **WHEN** the user runs `ito init --tools all`
- **THEN** the `ito-update-repo` skill SHALL be present in every configured harness skill directory
- **AND** the `ito-update-repo` command SHALL be present in every configured harness command/prompt directory

#### Scenario: Installed on update

- **GIVEN** a project was initialized before this skill existed
- **WHEN** the user runs `ito init --update --tools all`
- **THEN** the `ito-update-repo` skill and command SHALL be added to every configured harness
