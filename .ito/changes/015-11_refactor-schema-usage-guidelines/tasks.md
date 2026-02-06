# Tasks

- [ ] Add a spec for `ito-schemas` responsibilities and boundaries.
- [ ] Audit current `ito-schemas` usage from `ito-domain`, `ito-core`, and adapters.
- [ ] Identify schema leakage that causes coupling/boilerplate and decide:
  - [ ] keep schema type (pure data)
  - [ ] wrap into domain type (rules/behavior)
- [ ] Add guardrails that prevent I/O and process execution usage inside `ito-schemas`.
- [ ] Run `ito validate 015-11_refactor-schema-usage-guidelines --strict`.
