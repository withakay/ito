<!-- ITO:START -->
# Cli Plan

## Purpose

This spec defines the current behavior and requirements for cli plan.

## Requirements

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
Planning initialization SHALL NOT enforce fixed content for `PROJECT.md`, `ROADMAP.md`, or `STATE.md`. Planning quality SHALL instead come from the retained `ito-proposal` skill and its authoritative instruction artifacts.

#### Scenario: Planning init skips legacy templates
- **WHEN** executing `ito plan init`
- **THEN** the workflow does not create fixed `PROJECT.md`, `ROADMAP.md`, or `STATE.md` content
- **AND** the planning experience relies on `ito-proposal` guidance
<!-- ITO:END -->
