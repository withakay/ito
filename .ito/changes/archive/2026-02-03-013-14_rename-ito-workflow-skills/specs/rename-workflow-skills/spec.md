## Purpose

Rename ito workflow skills to be more descriptive and discoverable. Keyword-stuff descriptions to trigger on common user language.

## MODIFIED Requirements

### Requirement: ito-proposal renamed to ito-write-change-proposal

The skill formerly known as `ito-proposal` SHALL be renamed to `ito-write-change-proposal`.

#### Scenario: Skill directory name

- **WHEN** the skill is installed
- **THEN** it lives at `.opencode/skills/ito-write-change-proposal/` (or equivalent for other harnesses)

#### Scenario: Skill frontmatter name

- **WHEN** the skill SKILL.md is read
- **THEN** the `name` field is `ito-write-change-proposal`

### Requirement: ito-apply renamed to ito-apply-change-proposal

The skill formerly known as `ito-apply` SHALL be renamed to `ito-apply-change-proposal`.

#### Scenario: Skill directory name

- **WHEN** the skill is installed
- **THEN** it lives at `.opencode/skills/ito-apply-change-proposal/` (or equivalent for other harnesses)

#### Scenario: Skill frontmatter name

- **WHEN** the skill SKILL.md is read
- **THEN** the `name` field is `ito-apply-change-proposal`

### Requirement: ito-write-change-proposal has keyword-rich description

The `ito-write-change-proposal` skill SHALL have a description that triggers on planning/design language.

#### Scenario: Description content

- **WHEN** the skill description is read
- **THEN** it contains keywords: create, design, plan, propose, specify, write, feature, change, requirement, enhancement, fix, modification, spec, tasks, proposal

### Requirement: ito-apply-change-proposal has keyword-rich description

The `ito-apply-change-proposal` skill SHALL have a description that triggers on implementation language.

#### Scenario: Description content

- **WHEN** the skill description is read
- **THEN** it contains keywords: implement, execute, apply, build, code, develop, feature, change, requirement, enhancement, fix, modification, spec, tasks

### Requirement: ito router updated

The `ito` skill (router) SHALL route to the new skill names.

#### Scenario: Routing to write skill

- **WHEN** user invokes `ito proposal` or `ito write-change-proposal`
- **THEN** the router invokes `ito-write-change-proposal`

#### Scenario: Routing to apply skill

- **WHEN** user invokes `ito apply` or `ito apply-change-proposal`
- **THEN** the router invokes `ito-apply-change-proposal`

### Requirement: Cross-references updated

All ito-* skills that reference the old names SHALL be updated.

#### Scenario: No old references

- **WHEN** any ito skill is read
- **THEN** it does not reference `ito-proposal` or `ito-apply` (uses new names)
