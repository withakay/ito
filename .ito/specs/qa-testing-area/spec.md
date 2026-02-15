## ADDED Requirements

### Requirement: Ralph module minimum line coverage

The `ito-core` ralph module (`src/ralph/**`) SHALL maintain at least 80% line coverage as reported by `cargo llvm-cov`.

#### Scenario: Ralph coverage meets floor after test additions

- **WHEN** running `cargo llvm-cov report --package ito-core` and filtering for `ralph/` files
- **THEN** each file SHALL report at least 80% line coverage

### Requirement: Harness module minimum line coverage

The `ito-core` harness module (`src/harness/**`) SHALL maintain at least 80% line coverage as reported by `cargo llvm-cov`.

#### Scenario: Harness coverage meets floor after test additions

- **WHEN** running `cargo llvm-cov report --package ito-core` and filtering for `harness/` files
- **THEN** each file SHALL report at least 80% line coverage
