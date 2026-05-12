<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Project planning initialization

The CLI SHALL initialize the planning workspace by ensuring `.ito/planning/` exists. It SHALL NOT create fixed planning markdown templates such as `PROJECT.md`, `ROADMAP.md`, or `STATE.md` during planning initialization.

- **Requirement ID**: `cli-plan:planning-workspace-initialization`

#### Scenario: Initialize planning workspace

- **WHEN** executing `ito plan init`
- **THEN** create the `.ito/planning/` directory if it does not exist
- **AND** preserve any existing planning documents already present in that directory
- **AND** NOT create `.ito/planning/PROJECT.md`
- **AND** NOT create `.ito/planning/ROADMAP.md`
- **AND** NOT create `.ito/planning/STATE.md`
- **AND** display a success message indicating the planning workspace is available

### Requirement: Planning status display

The CLI SHALL display the state of the planning workspace by reporting whether `.ito/planning/` exists and which planning documents are present, rather than assuming a fixed set of planning files.

- **Requirement ID**: `cli-plan:planning-workspace-status`

#### Scenario: Show planning workspace status

- **WHEN** executing `ito plan status`
- **THEN** check whether `.ito/planning/` exists
- **AND** enumerate planning markdown documents present under `.ito/planning/`
- **AND** indicate whether `.ito/research/` exists as a companion workspace
- **AND** print a hint to use `/ito-plan` when the planning workspace exists but contains no plan documents

### Requirement: Error handling

The CLI SHALL provide clear error messages and recovery suggestions when planning workspace commands encounter issues.

- **Requirement ID**: `cli-plan:planning-error-handling`

#### Scenario: Planning directory cannot be created

- **WHEN** the `.ito/planning/` directory cannot be created due to permissions or filesystem errors
- **THEN** display an error message explaining the failure
- **AND** suggest checking directory permissions and disk space
- **AND** exit with code 1

#### Scenario: Planning workspace has no plans yet

- **WHEN** executing `ito plan status`
- **AND** `.ito/planning/` exists but contains no planning markdown documents
- **THEN** display a non-error status showing the workspace is empty
- **AND** suggest using `/ito-plan` to create the first plan

## REMOVED Requirements

### Requirement: Project state management

This requirement is removed; the planning workflow SHALL NOT require a fixed `STATE.md`-centric state management model.

- **Requirement ID**: `cli-plan:remove-project-state-management`

**Reason**: The planning workflow is being repositioned as lightweight, pre-proposal exploration rather than a fixed state-tracking system centered on `STATE.md`.

**Migration**: Capture active planning context in topic-specific markdown files under `.ito/planning/`, and use `.ito/research/` for deeper investigations that support those plans.

#### Scenario: Planning no longer depends on STATE.md

- **WHEN** a user initializes or reviews the planning workspace
- **THEN** the workflow SHALL NOT require `.ito/planning/STATE.md` to exist
- **AND** planning context may be captured in topic-specific plan documents instead

### Requirement: Roadmap milestone management

This requirement is removed; the planning workflow SHALL NOT depend on a fixed `ROADMAP.md` milestone model.

- **Requirement ID**: `cli-plan:remove-roadmap-milestone-management`

**Reason**: The legacy roadmap-specific planning model depends on `ROADMAP.md`, which this change stops bootstrapping and no longer treats as the canonical planning workflow.

**Migration**: Record proposal-oriented milestones or sequencing in planning documents under `.ito/planning/`, and split approved work into one or more change proposals when the plan is ready.

#### Scenario: Planning no longer depends on ROADMAP.md

- **WHEN** a user initializes or reviews the planning workspace
- **THEN** the workflow SHALL NOT require `.ito/planning/ROADMAP.md` to exist
- **AND** sequencing may be captured directly in planning documents instead

### Requirement: Template quality

This requirement is removed; planning initialization SHALL NOT enforce fixed template content for `PROJECT.md`, `ROADMAP.md`, or `STATE.md`.

- **Requirement ID**: `cli-plan:remove-fixed-template-quality`

**Reason**: Planning initialization will no longer generate fixed planning templates, so template-shape requirements for `PROJECT.md`, `ROADMAP.md`, and `STATE.md` no longer apply.

**Migration**: Move planning guidance into the `ito-plan` prompt and skill so planning quality is enforced by workflow guidance rather than hard-coded markdown templates.

#### Scenario: Planning init skips legacy templates

- **WHEN** executing `ito plan init`
- **THEN** the workflow SHALL NOT create template content for `PROJECT.md`, `ROADMAP.md`, or `STATE.md`
- **AND** the planning experience relies on `ito-plan` guidance instead
<!-- ITO:END -->
