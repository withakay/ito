# Spec: stable-instruction-generation

## Purpose

Ensure the review instruction honors the same configurable testing policy and user guidance injection patterns as all other instruction types.

## MODIFIED Requirements

### Requirement: User guidance injection

All instruction templates, including the review template, SHALL inject user guidance from `.ito/user-guidance.md` when present. The guidance SHALL appear in a dedicated `<user_guidance>` section within the rendered output.

#### Scenario: Review instruction includes user guidance

- **WHEN** the review instruction is generated and `.ito/user-guidance.md` exists
- **THEN** the rendered output SHALL contain a `<user_guidance>` section with the contents of user-guidance.md

### Requirement: Testing policy injection

All instruction templates, including the review template, SHALL include the project's testing policy (TDD workflow and coverage target) derived from the cascading config system.

#### Scenario: Review instruction includes testing policy

- **WHEN** the review instruction is generated
- **THEN** the rendered output SHALL contain testing policy information consistent with the project's configured TDD workflow and coverage target
