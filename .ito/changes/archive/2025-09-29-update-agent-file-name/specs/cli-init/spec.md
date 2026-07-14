## MODIFIED Requirements

### Requirement: Directory Creation

The command SHALL create the complete Ito directory structure with all required directories and files.

#### Scenario: Creating Ito structure

- **WHEN** `ito init` is executed
- **THEN** create the following directory structure:

```
ito/
├── project.md
├── AGENTS.md
├── specs/
└── changes/
    └── archive/
```

### Requirement: File Generation

The command SHALL generate required template files with appropriate content for immediate use.

#### Scenario: Generating template files

- **WHEN** initializing Ito
- **THEN** generate `AGENTS.md` containing complete Ito instructions for AI assistants
- **AND** generate `project.md` with project context template

### Requirement: AI Tool Configuration Details

The command SHALL properly configure selected AI tools with Ito-specific instructions using a marker system.

#### Scenario: Creating new CLAUDE.md

- **WHEN** CLAUDE.md does not exist
- **THEN** create new file with Ito content wrapped in markers including reference to `@ito/AGENTS.md`

### Requirement: Success Output

The command SHALL provide clear, actionable next steps upon successful initialization.

#### Scenario: Displaying success message

- **WHEN** initialization completes successfully
- **THEN** include prompt: "Please explain the Ito workflow from ito/AGENTS.md and how I should work with you on this project"
