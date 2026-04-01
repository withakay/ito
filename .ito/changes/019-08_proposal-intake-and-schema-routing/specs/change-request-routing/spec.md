<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ito exposes intent-biased proposal entrypoints

Ito SHALL provide intent-biased workflow entrypoints for fix-oriented and feature-oriented proposal creation alongside the neutral proposal lane.

- **Requirement ID**: change-request-routing:intent-biased-entrypoints

#### Scenario: Fix-oriented proposal entrypoint is available

- **WHEN** a user wants to start a change as a fix
- **THEN** Ito SHALL provide an `ito-fix` entrypoint that routes into proposal intake with fix-oriented defaults

#### Scenario: Feature-oriented proposal entrypoint is available

- **WHEN** a user wants to start a change as a feature
- **THEN** Ito SHALL provide an `ito-feature` entrypoint that routes into proposal intake with feature-oriented defaults

#### Scenario: Neutral proposal lane remains available

- **WHEN** a user wants to create a proposal without fix or feature bias
- **THEN** Ito SHALL preserve `ito-proposal` as the neutral fallback entrypoint

### Requirement: Intent-biased entrypoints bias defaults without removing override

Intent-biased proposal entrypoints SHALL recommend different questioning and schema defaults while still allowing the user or agent to override the recommendation.

- **Requirement ID**: change-request-routing:bias-with-override

#### Scenario: Fix lane recommends lighter-weight workflow

- **WHEN** a user starts from `ito-fix`
- **THEN** Ito SHALL prefer fix-oriented intake questions and recommend `minimalist` or `tdd` before `spec-driven` when the change is sufficiently bounded

#### Scenario: Feature lane recommends fuller discovery

- **WHEN** a user starts from `ito-feature`
- **THEN** Ito SHALL prefer feature-oriented intake questions and recommend `spec-driven` when the request introduces new capability or broader behavior change

#### Scenario: User chooses a different path than the default

- **WHEN** the recommended schema or lane does not fit the actual request
- **THEN** Ito SHALL allow the workflow to continue with a different schema or neutral proposal path
<!-- ITO:END -->
