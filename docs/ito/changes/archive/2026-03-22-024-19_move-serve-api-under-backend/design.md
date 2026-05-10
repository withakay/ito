## Context

`ito serve-api` currently exists as a top-level dev-oriented entrypoint even though backend status and token generation already live under `ito backend ...`. Keeping server startup outside the backend group makes the CLI harder to discover and leaves backend lifecycle split across two namespaces.

## Goals / Non-Goals

**Goals:**

- Make `ito backend serve` the canonical way to start the backend server.
- Preserve current serve behavior, flags, and config semantics under the new command path.
- Remove `ito serve-api` before it becomes a stable external contract.
- Update tests, docs, and guidance so the new command is the only documented path.

**Non-Goals:**

- Changing backend server runtime semantics, auth bootstrap behavior, or config format.
- Introducing a long-lived compatibility alias for `ito serve-api`.

## Decisions

- Decision: Move the command directly to `ito backend serve` with no compatibility alias.
  - Rationale: the command has not stabilized and the user explicitly wants to avoid carrying forward a dev-only surface.
  - Alternative considered: deprecated alias. Rejected because it prolongs the split namespace.

- Decision: Reuse the existing serve handler implementation instead of creating a second backend-start path.
  - Rationale: command relocation should be a surface change, not a behavior fork.
  - Alternative considered: new backend-specific handler. Rejected because it duplicates validated startup/config logic.

- Decision: Update QA/docs/guidance in the same change.
  - Rationale: backend server instructions are only useful if every documented path matches the canonical command.
  - Alternative considered: follow-up docs-only cleanup. Rejected because stale examples would immediately confuse users.

## Risks / Trade-offs

- [Users still invoke `ito serve-api`] -> Return actionable guidance pointing them to `ito backend serve`.
- [Tests and scripts miss the rename] -> Update command-focused QA coverage and search docs/prompts for old references.
- [Help/completions drift from implementation] -> Keep command definitions centralized in the backend CLI group and verify via CLI tests.

## Migration Plan

1. Add `serve` under `ito backend` and route it to the existing serve implementation.
2. Remove the top-level `serve-api` command and replace it with actionable guidance if needed.
3. Update tests, QA walkthroughs, and docs to use `ito backend serve`.
4. Validate help/completion output and backend startup flows under the new command path.

## Open Questions

- Should `ito serve-api` fail with a custom migration message, or disappear entirely from clap parsing?
