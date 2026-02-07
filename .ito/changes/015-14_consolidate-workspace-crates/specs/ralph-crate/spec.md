## REMOVED Requirements

### Requirement: Ralph is extracted into its own crate

Ralph orchestration logic SHALL reside in a dedicated workspace crate (`ito-ralph`).

Reason: Ralph orchestration is consolidated into `ito-core` and no longer lives in a separate crate.

#### Scenario: Ralph crate is no longer required

- **WHEN** implementing `ito ralph`
- **THEN** it uses `ito-core`'s Ralph module and does not require an `ito-ralph` crate

## ADDED Requirements

### Requirement: Ralph lives in ito-core as a module

Ralph orchestration logic SHALL reside in `ito-core/src/ralph/` as a module of `ito-core`, not as a separate `ito-ralph` crate.

The module SHALL contain: `runner` (main loop), `prompt` (prompt construction), `state` (state persistence), `validation` (project validation), `duration` (timing utilities).

`ito-cli` SHALL invoke Ralph through `ito_core::ralph::run_ralph()`, providing concrete implementations of repository traits and harness trait as dependency injection.

#### Scenario: Ralph module exists in ito-core

- **WHEN** examining `ito-core/src/ralph/`
- **THEN** it contains `runner`, `prompt`, `state`, `validation`, `duration`
