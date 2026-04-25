<!-- ITO:START -->
## ADDED Requirements

### Requirement: <!-- requirement name -->

<!-- requirement text -->

- **Requirement ID**: <!-- capability:requirement-name -->

<!-- OPTIONAL: Add tags when validators or readers need extra context such as behavior, ui, or stateful. -->
- **Tags**: <!-- behavior, ui -->

<!-- OPTIONAL: Reference external contracts instead of copying them inline. -->
- **Contract Refs**: <!-- openapi:POST /v1/example, jsonschema:ExampleRequest -->

<!-- OPTIONAL: Capture invariant-style rules for stateful or contract-sensitive behavior. -->
#### Rules / Invariants

<!-- Example:
- Requests without an active session MUST be rejected.
- Duplicate events MUST be ignored.
-->

<!-- OPTIONAL: Prefer a compact table when state changes matter. -->
#### State Transitions

<!-- Example:
| From | Event | To | Notes |
| --- | --- | --- | --- |
| pending | approve | active | Audit entry recorded |
| active | suspend | suspended | Background work stops |
-->

#### Scenario: <!-- scenario name -->

- **WHEN** <!-- condition -->
- **THEN** <!-- expected outcome -->

<!-- Traceability note:
  - Requirement ID format: `<capability>:<requirement-name>` (e.g. `auth:two-factor-auth`)
  - When any requirement in a change includes a Requirement ID, ALL requirements must include one.
  - Requirement IDs enable `ito trace <change-id>` to report task coverage.
  - Omit the Requirement ID line entirely if you do not need traceability for this change.
-->
<!-- ITO:END -->
