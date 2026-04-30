## ADDED Requirements

### Requirement: Selective template markdown compression
The template maintenance workflow SHALL support compaction planning for markdown prompt assets in `ito-rs/crates/ito-templates` that belong to AGENTS guidance, skills, agents, commands, and instructions asset families.

#### Scenario: Supported asset families are selected
- **WHEN** maintainers run the compaction workflow for template markdown assets
- **THEN** markdown files under crate/project `AGENTS.md`, `assets/skills/`, `assets/agents/`, `assets/commands/`, and `assets/instructions/` are treated as eligible inputs
- **AND** assets outside those families remain out of scope unless a later change expands the policy

### Requirement: Protected change authoring templates remain uncompressed
The template compaction workflow MUST exclude change-proposal authoring templates named `spec.md`, `design.md`, `proposal.md`, or `tasks.md` even when a file lives under an otherwise eligible template asset family.

#### Scenario: Excluded basenames are skipped
- **WHEN** an otherwise eligible template asset has one of the protected basenames
- **THEN** the compaction workflow skips that file
- **AND** proposal/spec authoring templates remain unchanged by this change
