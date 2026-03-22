## Why

Even where repository abstractions already exist, command handlers still instantiate concrete filesystem repositories directly. We need one central runtime selector/factory that chooses filesystem or remote repository implementations from resolved configuration so commands stop making persistence decisions ad hoc.

## What Changes

- Add a central repository runtime/factory that resolves the repository set for the current mode.
- Support two client-side persistence implementations initially: `filesystem` and `remote`.
- Treat REST as the first remote transport implementation without baking transport assumptions into command handlers.
- Make the current REST implementation explicit about HTTP semantics: reads stay on safe `GET` endpoints, mutations use the appropriate non-GET verbs, and retryable mutations are idempotent.
- Reuse the same concrete local repository implementations in direct/local composition and backend-server composition, so HTTP adds transport rather than duplicating repository behavior.
- Remove direct `Fs*Repository::new(...)` construction from command adapters in favor of runtime-selected repositories.

## Impact

- Affected specs: `repository-runtime-selection`, `backend-client-runtime`
- Affected code: runtime/config resolution, CLI command wiring, repository construction helpers, backend server composition
- Behavioral change: commands resolve persistence through one runtime-selected repository set instead of command-local concrete types

## Execution Guidance

- Do this first.
- This change establishes the repository bundle/factory shape, mode naming, and shared composition/error conventions used by the other repository proposals.
- The other repository-wiring changes should not finalize their implementation before this change is reviewed.
