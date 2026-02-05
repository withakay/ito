# validator-warnings Specification

## Purpose

TBD - created by archiving change 001-02_interactive-splitting. Update Purpose after archive.

## Requirements

### Requirement: Validate delta count threshold

The validator SHALL warn when the number of delta specs in a change exceeds the configured threshold.

#### Scenario: Delta count within limit

- **WHEN** validating a change with \<= 10 deltas
- **THEN** validation passes without warning

#### Scenario: Delta count exceeds limit

- **WHEN** validating a change with > 10 deltas
- **THEN** validation produces a warning issue
- **AND** warning message indicates count and limit
- **AND** validation result is still considered "valid" (warnings don't fail validation)

### Requirement: Suggest interactive remediation

The validation report SHALL include metadata indicating that interactive remediation is available for specific warnings.

#### Scenario: Warning with remediation

- **WHEN** producing "max deltas" warning
- **THEN** issue object includes `remediation: "split_change"` property
- **AND** this property signals CLI to offer interactive split flow

### Requirement: Respect ignore configuration

The validator SHALL respect `ignore_warnings` configuration in the change's `.ito.yaml` file.

#### Scenario: Ignored warning

- **WHEN** validating a change with > 10 deltas
- **AND** change config has `ignore_warnings: ["max_deltas"]`
- **THEN** no warning is produced for delta count
