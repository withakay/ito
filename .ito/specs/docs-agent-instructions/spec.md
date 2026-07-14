# Docs Agent Instructions

## Purpose

This spec defines the current behavior and requirements for docs agent instructions.

## Requirements

### Requirement: Docs mention project setup workflow
AI-facing documentation installed by Ito SHALL direct agents to run `ito agent instruction project-setup` and follow the emitted prompt. It MUST NOT install or recommend a separate `/ito-project-setup` wrapper.

#### Scenario: Docs include direct project setup instruction
- **WHEN** a user reads installed agent docs
- **THEN** they can find `ito agent instruction project-setup`
- **AND** no project-setup command wrapper expands the seven-command palette
<!-- ITO:END -->
