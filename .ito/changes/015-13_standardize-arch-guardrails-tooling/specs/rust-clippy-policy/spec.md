## ADDED Requirements

### Requirement: Domain-restriction checks prioritize lint or compiler-backed enforcement

Domain-layer restriction checks SHOULD prioritize lint/compiler-backed enforcement over textual baseline counting when practical.
If textual baseline checks remain, they MUST be documented as temporary and scoped to minimize long-term maintenance.

#### Scenario: Lint/compiler-backed checks are preferred

- **WHEN** defining checks for restricted APIs in Rust domain-layer crates
- **THEN** the policy SHOULD use clippy/lint/test/compiler-backed mechanisms before introducing new textual baseline counting

#### Scenario: Temporary textual baselines are explicitly tracked

- **WHEN** a textual baseline check is retained for compatibility
- **THEN** the check MUST have documented scope and migration notes toward lint/compiler-backed enforcement
