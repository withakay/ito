<!-- ITO:START -->
## Context

`ito create module` currently forwards name/scope/dependency arguments but does not expose a description argument in the clap surface for module creation. This creates a gap between expected one-command module scaffolding and actual behavior, especially for scripted usage where users want to avoid post-create manual edits.

## Goals / Non-Goals

**Goals:**

- Add a first-class description argument to `ito create module`.
- Preserve existing create-module behavior for users who do not provide description.
- Keep Rust CLI behavior aligned with expected artifact workflow parity.

**Non-Goals:**

- Redesigning module file format.
- Changing module ID allocation or module naming rules.
- Introducing interactive prompts for module description entry.

## Decisions

- Extend the clap `create module` subcommand to accept a description argument (`--description <text>`), following existing CLI flag patterns.
  - Alternative considered: positional trailing description text; rejected because it introduces parsing ambiguity and diverges from existing long-flag conventions.
- Ensure forwarded argument vectors include description so create-module execution receives the value in both clap and compatibility pathways.
  - Alternative considered: write description in a post-processing step outside create flow; rejected due to duplicated logic and increased drift risk.
- Add integration coverage in create-command tests for description acceptance and resulting module metadata output.
  - Alternative considered: unit-only coverage; rejected because end-to-end command behavior is the user-visible contract.

## Risks / Trade-offs

- [Risk] Description handling in clap but not in forwarding path could regress behavior. -> Mitigation: add tests that execute `ito create module ... --description ...` through the full command.
- [Risk] Future TS parity expectations may change. -> Mitigation: keep parity requirement explicit in spec deltas and validate behavior in integration tests.

## Migration Plan

- No data migration required.
- Rollout is additive and backward compatible: existing invocations remain valid.
- Rollback can remove the flag from CLI parsing and forwarding without impacting created module directory names.

## Open Questions

- Should a short alias (for example `-d`) be added now or deferred to a broader CLI flag consistency pass?
<!-- ITO:END -->
