# Refactor: Schema usage guidelines (`ito-schemas` boundaries)

## Why

- `ito-schemas` defines serde models for on-disk formats.
- Without explicit boundaries, schema types can leak into business logic and adapters in inconsistent ways.
- A pragmatic guideline avoids "two parallel type hierarchies" while still keeping format concerns contained.

## What

- Define a spec for the `ito-schemas` crate and how schema types should be used.
- Codify a pragmatic rule:
  - use schema types directly when they are pure data and match the domain concept
  - introduce domain types when behavior/rules diverge from the on-disk format
- Add guardrails (where feasible) to prevent `ito-schemas` from accumulating I/O or business logic.

## Scope

- Documentation + guardrail definition; follow-on changes can migrate specific hot spots (e.g., CLI schema usage) as needed.

## Depends on

- 015-01_refactor-arch-guardrails

## Verification

- `ito validate 015-11_refactor-schema-usage-guidelines --strict`
