# Tasks

- [x] Add a spec for `ito-schemas` responsibilities and boundaries.
- [x] Audit current `ito-schemas` usage from `ito-domain`, `ito-core`, and adapters.
- [x] Identify schema leakage that causes coupling/boilerplate and decide:
  - [x] keep schema type (pure data)
  - [x] wrap into domain type (rules/behavior)
- [x] Add guardrails that prevent I/O and process execution usage inside `ito-schemas`.
- [x] Run `ito validate 015-11_refactor-schema-usage-guidelines --strict`.
