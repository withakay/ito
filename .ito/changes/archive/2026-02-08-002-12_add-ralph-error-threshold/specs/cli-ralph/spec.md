## ADDED Requirements

### Requirement: Ralph continues on harness failure by default

When a harness run returns a non-zero exit code, Ralph SHALL continue iterating by default and feed the harness failure output back into the next prompt context.

#### Scenario: Non-zero harness exit continues loop

- **GIVEN** `ito ralph` is running with default options
- **WHEN** the harness returns a non-zero exit code
- **THEN** Ralph SHALL NOT exit immediately
- **AND** Ralph SHALL record the harness stdout/stderr as failure context for the next iteration

### Requirement: Ralph enforces configurable harness error threshold

Ralph SHALL fail the run when non-zero harness exits reach the configured error threshold.

#### Scenario: Default threshold is applied

- **GIVEN** `ito ralph` is running without `--error-threshold`
- **WHEN** non-zero harness exits reach 10 attempts
- **THEN** Ralph SHALL exit with an error indicating the non-zero exit threshold was exceeded

#### Scenario: Custom threshold is applied

- **GIVEN** `ito ralph --error-threshold 3`
- **WHEN** non-zero harness exits reach 3 attempts
- **THEN** Ralph SHALL exit with an error indicating the threshold was exceeded

#### Scenario: Exit-on-error remains fail-fast

- **GIVEN** `ito ralph --exit-on-error --error-threshold 10`
- **WHEN** the first harness run returns a non-zero exit code
- **THEN** Ralph SHALL exit immediately
- **AND** the threshold counter SHALL NOT be used for that run
