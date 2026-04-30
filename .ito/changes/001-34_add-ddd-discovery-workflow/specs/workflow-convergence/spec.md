<!-- ITO:START -->
## ADDED Requirements

### Requirement: Domain-discovery entrypoint

Ito SHALL extend the canonical instruction-and-skill workflow with a domain-discovery lane that can run before proposal scaffolding for broad, ambiguous, or cross-context work.

- **Requirement ID**: `workflow-convergence:domain-discovery-entrypoint`

#### Scenario: Discovery lane precedes proposal lane

- **WHEN** a user request is still exploratory or the workflow detects substantial domain ambiguity
- **THEN** Ito routes the user toward domain discovery before proposal creation
- **AND** the proposal workflow consumes the discovery outputs rather than bypassing them

#### Scenario: Routine work keeps the direct path

- **WHEN** a request is already bounded, low-risk, and clear
- **THEN** Ito may continue directly into proposal creation or direct implementation
- **AND** the discovery lane remains optional rather than mandatory ceremony
<!-- ITO:END -->
