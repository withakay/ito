# future-ideas-docs Specification

## Purpose

TBD - created by archiving change 000-02_consolidate-workflow-docs. Update Purpose after archive.

## Requirements

### Requirement: Future Ideas Documentation

The project SHALL maintain a document (`docs/future-ideas.md`) capturing unimplemented but valuable concepts from experimental documentation for future consideration.

#### Scenario: Preserve unimplemented workflow concepts

- **WHEN** experimental documentation is removed
- **THEN** valuable unimplemented ideas (custom schemas, OPSX fluid workflow model, CLI enhancements) SHALL be preserved in `docs/future-ideas.md`
- **AND** each idea SHALL be clearly marked as "not yet implemented"

#### Scenario: Separate aspirational from implemented

- **WHEN** a user reads the future ideas documentation
- **THEN** they SHALL clearly understand these are proposals for future work
- **AND** they SHALL NOT confuse these ideas with current Ito capabilities
