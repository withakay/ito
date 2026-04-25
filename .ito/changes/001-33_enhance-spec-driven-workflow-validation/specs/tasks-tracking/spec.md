<!-- ITO:START -->
## ADDED Requirements

These requirements tighten enhanced task semantics so tasks stay close enough to implementation to guide agents without becoming implementation design documents.

### Requirement: Enhanced tasks expose quality-critical fields

Enhanced task blocks SHALL preserve Files, Dependencies, Action, Verify, Done When, Requirements, and Status metadata as structured fields for validation.

- **Requirement ID**: tasks-tracking:quality-critical-fields

#### Scenario: Enhanced task fields are parsed

- **GIVEN** an enhanced task block contains Files, Dependencies, Action, Verify, Done When, Requirements, and Status metadata
- **WHEN** Ito parses the tracking file
- **THEN** each metadata line is available to task quality and traceability validation

#### Scenario: Missing status is invalid

- **GIVEN** an enhanced task block omits Status metadata
- **WHEN** Ito validates the tracking file
- **THEN** validation reports an error for the missing task status

### Requirement: Enhanced task verification is concrete

Enhanced tasks that describe implementation work MUST include concrete verification commands or targets.

- **Requirement ID**: tasks-tracking:concrete-verification

#### Scenario: Vague verification is reported

- **GIVEN** an enhanced implementation task has `Verify: run tests`
- **WHEN** Ito validates the tracking file
- **THEN** validation reports a warning asking for a concrete command or target

#### Scenario: Specific verification is accepted

- **GIVEN** an enhanced implementation task has `Verify: cargo test -p ito-core validate::scenario_grammar`
- **WHEN** Ito validates the tracking file
- **THEN** validation accepts the verification metadata as concrete
<!-- ITO:END -->
