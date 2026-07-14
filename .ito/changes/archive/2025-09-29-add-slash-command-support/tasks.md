# Implementation Tasks

## 1. Templates and Configurators

- \[x\] 1.1 Create shared templates for the Proposal, Apply, and Archive commands with instructions for each workflow stage from `ito/README.md`.
- \[x\] 1.2 Implement a `SlashCommandConfigurator` base and tool-specific configurators for Claude Code and Cursor.

## 2. Claude Code Integration

- \[x\] 2.1 Generate `.claude/commands/ito/{proposal,apply,archive}.md` during `ito init` using shared templates.
- \[x\] 2.2 Update existing `.claude/commands/ito/*` files during `ito update`.

## 3. Cursor Integration

- \[x\] 3.1 Generate `.cursor/commands/{ito-proposal,ito-apply,ito-archive}.md` during `ito init` using shared templates.
- \[x\] 3.2 Update existing `.cursor/commands/*` files during `ito update`.

## 4. Verification

- \[x\] 4.1 Add tests verifying slash command files are created and updated correctly.

## 5. OpenCode Integration

- \[x\] 5.1 Generate `.opencode/commands/{ito-proposal,ito-apply,ito-archive}.md` during `ito init` using shared templates.
- \[x\] 5.2 Update existing `.opencode/commands/*` files during `ito update`.
