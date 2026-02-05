## MODIFIED Requirements

### Requirement: Tool-Agnostic Updates

The update command SHALL refresh Ito-managed files in a predictable manner while respecting each team's chosen tooling.

#### Scenario: Updating files

- **WHEN** updating files
- **THEN** completely replace `ito/AGENTS.md` with the latest template
- **AND** create or refresh the root-level `AGENTS.md` stub using the managed marker block, even if the file was previously absent
- **AND** update only the Ito-managed sections inside existing AI tool files, leaving user-authored content untouched
- **AND** avoid creating new native-tool configuration files (slash commands, CLAUDE.md, etc.) unless they already exist
