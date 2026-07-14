## ADDED Requirements

### Requirement: Architecture guardrails are enforced

The repository MUST provide an architecture guardrails check runnable as `make arch-guardrails`.

The architecture guardrails MUST be executed by `prek` (pre-commit) and by CI.

#### Scenario: Guardrails are runnable

- **WHEN** a developer runs `make arch-guardrails`
- **THEN** it MUST exit successfully when constraints are satisfied

#### Scenario: Guardrails run in prek

- **WHEN** inspecting `.pre-commit-config.yaml`
- **THEN** it MUST include a local hook `arch-guardrails`
- **AND** the hook MUST run `make arch-guardrails`

#### Scenario: Guardrails run in CI

- **WHEN** inspecting the repository's CI configuration
- **THEN** CI MUST run `make arch-guardrails` or `prek run --all-files`
