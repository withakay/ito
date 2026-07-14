<!-- ITO:START -->
## ADDED Requirements

### Requirement: Proposal intake clarifies change intent before scaffolding

Ito SHALL provide a proposal-intake workflow that clarifies the requested change before creating or scaffolding a change proposal.

- **Requirement ID**: proposal-intake:clarify-change-before-scaffold

#### Scenario: Intake precedes change scaffolding

- **WHEN** a user starts a proposal workflow with an underspecified request
- **THEN** Ito SHALL ask focused intake questions about the problem, desired outcome, scope, and constraints before scaffolding the change

### Requirement: Proposal intake produces an explicit handoff outcome

The proposal-intake workflow SHALL end with an explicit next-step outcome so downstream workflows do not repeat the same discovery work.

- **Requirement ID**: proposal-intake:produce-handoff-outcome

#### Scenario: Intake is ready for proposal creation

- **WHEN** the intake flow determines the request is clear enough to become a change proposal
- **THEN** Ito SHALL produce a concise summary of the clarified request and hand it off into proposal creation

#### Scenario: Intake redirects to another lane

- **WHEN** the intake flow determines the request needs broader design exploration or does not require a proposal
- **THEN** Ito SHALL produce an explicit outcome describing the recommended next lane instead of scaffolding a proposal immediately

### Requirement: Proposal intake uses repository facts for brownfield questions

For brownfield work, the proposal-intake workflow SHALL prefer repository and spec facts over asking the user to rediscover information already available in the codebase.

- **Requirement ID**: proposal-intake:ground-brownfield-questions

#### Scenario: Intake asks a brownfield confirmation question

- **WHEN** the intake workflow needs confirmation about an existing capability, code path, or workflow behavior
- **THEN** it SHALL cite the relevant repo or spec evidence that motivated the question
<!-- ITO:END -->
