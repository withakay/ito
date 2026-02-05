## Purpose

Fix the naming mismatch in `using-ito-skills` skill and add multi-harness support for OpenCode, Claude Code, and Codex.

## MODIFIED Requirements

### Requirement: Frontmatter name matches directory

The skill frontmatter `name` field SHALL match the directory name.

#### Scenario: Name consistency

- **WHEN** the skill SKILL.md is read
- **THEN** the `name` field is `using-ito-skills`
- **AND** the directory is `using-ito-skills/`

### Requirement: No superpowers references in content

The skill content SHALL NOT reference `superpowers`.

#### Scenario: Clean content

- **WHEN** the skill content is searched
- **THEN** no references to `superpowers` are found

### Requirement: Keyword-rich description

The skill SHALL have a description that triggers on skill discovery language.

#### Scenario: Description content

- **WHEN** the skill description is read
- **THEN** it contains keywords: skill, discover, find, invoke, load, use, before, first, priority

## ADDED Requirements

### Requirement: OpenCode skill instructions

The skill SHALL include instructions for using skills in OpenCode.

#### Scenario: OpenCode guidance

- **WHEN** running in OpenCode
- **THEN** the skill explains: use native `skill` tool, `skill list` to discover, `skill load <name>` to invoke

### Requirement: Claude Code skill instructions

The skill SHALL include instructions for using skills in Claude Code.

#### Scenario: Claude Code guidance

- **WHEN** running in Claude Code
- **THEN** the skill explains: use `mcp_skill` function with skill name parameter

### Requirement: Codex skill instructions

The skill SHALL include instructions for using skills in Codex.

#### Scenario: Codex guidance

- **WHEN** running in Codex
- **THEN** the skill explains: read skill files from `.codex/skills/ito-<name>/SKILL.md`

### Requirement: Harness detection guidance

The skill SHALL explain how to detect which harness is running.

#### Scenario: Detection hints

- **WHEN** the skill is invoked
- **THEN** it provides hints for detecting the current harness (tool availability, environment)
