<!-- ITO:START -->
## ADDED Requirements

### Requirement: Schema guidance recommends the best-fit existing schema

Ito SHALL recommend an existing workflow schema based on change shape instead of only listing available schemas.

- **Requirement ID**: schema-selection-guidance:recommend-by-change-shape

#### Scenario: New capability recommends spec-driven

- **WHEN** the requested change introduces a new capability, cross-cutting behavior change, or unresolved feature scope
- **THEN** Ito SHALL recommend the `spec-driven` schema or the `event-driven` schema when the request is centered on event or message workflow behavior

#### Scenario: Localized fix recommends minimalist or tdd

- **WHEN** the requested change is a bounded bug fix or regression-oriented correction
- **THEN** Ito SHALL recommend `minimalist` or `tdd` before `spec-driven`, unless the blast radius or ambiguity requires a fuller workflow

### Requirement: Schema guidance covers supporting platform and infrastructure work

Schema recommendation guidance SHALL explicitly cover non-product changes such as platform, tooling, release, CI, and infrastructure work.

- **Requirement ID**: schema-selection-guidance:cover-supporting-platform-work

#### Scenario: Supporting platform change is still rigorous

- **WHEN** the requested change affects supporting platform or infrastructure behavior with a bounded scope
- **THEN** Ito SHALL describe when `minimalist` is an appropriate rigorous workflow for that change shape

#### Scenario: Supporting change with broad behavior impact escalates

- **WHEN** a platform, tooling, or infrastructure request changes architecture, cross-cutting behavior, or migration risk
- **THEN** Ito SHALL recommend `spec-driven` even if the user initially framed it as a fix or supporting change
<!-- ITO:END -->
