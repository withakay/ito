## Purpose

Update `finishing-a-development-branch` skill to reference correct skills and add ito-archive option.

## MODIFIED Requirements

### Requirement: References ito-apply-change-proposal

The skill SHALL reference `ito-apply-change-proposal` instead of `executing-plans`.

#### Scenario: Execution reference

- **WHEN** the skill references task execution
- **THEN** it references `ito-apply-change-proposal`

### Requirement: Includes ito-archive option

The skill SHALL include a fifth option for archiving ito changes.

#### Scenario: Archive option presented

- **WHEN** the skill presents completion options
- **THEN** it includes option 5: "Archive ito change"
- **AND** this option invokes `ito-archive` skill

### Requirement: Ito change detection

The skill SHALL detect if working on a ito change.

#### Scenario: Ito change present

- **WHEN** `.ito/changes/` contains an in-progress change
- **THEN** the archive option is highlighted as relevant

#### Scenario: No ito change

- **WHEN** not working on a ito change
- **THEN** the archive option is shown but noted as not applicable
