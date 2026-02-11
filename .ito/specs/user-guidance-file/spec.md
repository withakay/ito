## ADDED Requirements

### Requirement: User prompts directory

Ito SHALL support a project-local user prompts directory at `.ito/user-prompts/` for artifact-scoped guidance files.

#### Scenario: Directory is optional

- **WHEN** `.ito/user-prompts/` does not exist
- **THEN** instruction generation still works using existing guidance behavior

#### Scenario: Directory can store artifact-scoped markdown files

- **WHEN** users create markdown files in `.ito/user-prompts/` named by artifact ID
- **THEN** Ito can consume them as artifact-scoped guidance inputs

#### Scenario: Shared guidance remains backward-compatible

- **WHEN** `.ito/user-guidance.md` exists
- **THEN** Ito continues to support it as shared guidance across artifacts

### Requirement: Shared guidance file in user-prompts directory

Ito SHALL support `.ito/user-prompts/guidance.md` as a shared guidance file that applies across instruction artifacts.

#### Scenario: Preferred shared guidance file is recognized

- **WHEN** `.ito/user-prompts/guidance.md` exists
- **THEN** Ito can consume it as shared guidance across artifacts

#### Scenario: Legacy shared guidance file remains supported

- **WHEN** `.ito/user-prompts/guidance.md` does not exist
- **AND** `.ito/user-guidance.md` exists
- **THEN** Ito uses `.ito/user-guidance.md` as shared guidance fallback
