## ADDED Requirements

### Requirement: Tool-Agnostic Updates

The update command SHALL update only existing AI tool configuration files and SHALL NOT create new ones.

#### Scenario: Updating existing tool files

- **WHEN** a user runs `ito update`
- **THEN** update each AI tool configuration file that exists (e.g., CLAUDE.md, COPILOT.md)
- **AND** do not create missing tool configuration files
- **AND** preserve user content outside Ito markers

### Requirement: Core Files Always Updated

The update command SHALL always update the core Ito files and display an ASCII-safe success message.

#### Scenario: Successful update

- **WHEN** the update completes successfully
- **THEN** replace `ito/README.md` with the latest template
- **AND** update existing AI tool configuration files within markers
- **AND** display the message: "Updated Ito instructions"
