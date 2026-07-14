# Refactor: Establish error boundaries (domain vs core vs adapters)

## Why

- Error handling currently risks mixing concerns: domain logic, I/O failures, and UI/diagnostic formatting.
- Onion-style layering is easier to enforce when error types clearly match the layer they originate from.
- Consistent error boundaries reduce duplication and make adapters thinner (CLI/Web present errors; core constructs them).

## What

- Define/normalize domain error types in `ito-domain` (business rule failures and validation).
- Define core/use-case error types in `ito-core` that wrap/translate infrastructure failures (I/O, schema parsing, process execution).
- Ensure adapters (`ito-cli`, `ito-web`) only format/present errors; they do not create business-rule errors.
- Add guardrails that prevent diagnostic/UI frameworks from leaking into `ito-domain`.

## Scope

- Type and boundary changes only; this should not change user-visible output beyond improving consistency.

## Depends on

- 015-01_refactor-arch-guardrails

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
