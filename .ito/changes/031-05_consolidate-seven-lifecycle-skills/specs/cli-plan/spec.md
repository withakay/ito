<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Planning status display
The CLI SHALL display whether `.ito/planning/` exists, enumerate its planning documents, and report the companion `.ito/research/` workspace without assuming fixed planning files. Any empty-workspace hint SHALL point to retained `ito-proposal` guidance rather than an `ito-plan` helper.

#### Scenario: Show planning workspace status
- **WHEN** executing `ito plan status`
- **THEN** the CLI reports the planning directory and its markdown documents
- **AND** indicates whether `.ito/research/` exists
- **AND** points an empty workspace to `ito-proposal`

### Requirement: Error handling
The CLI SHALL provide clear errors and recovery suggestions for planning workspace failures, while normal empty-workspace guidance SHALL name the retained `ito-proposal` lifecycle skill.

#### Scenario: Planning directory cannot be created
- **WHEN** `.ito/planning/` cannot be created because of a filesystem error
- **THEN** the CLI explains the failure and suggests checking permissions and available space
- **AND** exits with code 1

#### Scenario: Planning workspace has no plans yet
- **WHEN** `ito plan status` finds an empty planning workspace
- **THEN** it reports a non-error empty status
- **AND** suggests using `ito-proposal` to develop the first plan

### Requirement: Template quality
Planning initialization SHALL NOT enforce fixed content for `PROJECT.md`, `ROADMAP.md`, or `STATE.md`. Planning quality SHALL instead come from the retained `ito-proposal` skill and its authoritative instruction artifacts.

#### Scenario: Planning init skips legacy templates
- **WHEN** executing `ito plan init`
- **THEN** the workflow does not create fixed `PROJECT.md`, `ROADMAP.md`, or `STATE.md` content
- **AND** the planning experience relies on `ito-proposal` guidance
<!-- ITO:END -->
