## Why

If the repository abstraction is real, the client should be able to swap in another local persistence implementation without changing command behavior. Supporting a direct local SQLite repository mode would both prove the design and provide a structured local storage option that bypasses HTTP while still using the same repository interfaces.

## What Changes

- Add `sqlite` as a client-side persistence mode alongside `filesystem` and `remote`.
- Allow the repository runtime/factory to construct SQLite-backed repository implementations directly from local configuration.
- Reuse the same SQLite-backed repository implementations in both direct local composition and backend-server composition, so HTTP remains an outer transport layer rather than a second storage implementation.
- Ensure command behavior and result/error shapes remain consistent across filesystem, SQLite, and remote modes.

## Impact

- Affected specs: `repository-runtime-selection`, `config`
- Affected code: repository runtime/factory, SQLite-backed adapters, local configuration resolution
- Behavioral change: Ito can run locally against a SQLite-backed repository implementation without using the remote REST transport

## Execution Guidance

- Start after `025-04_add-repository-runtime-factory` has settled the shared repository bundle/factory shape.
- This change can run in parallel with `025-01_wire-change-repository-backends`, `025-02_wire-task-repository-backends`, and `025-03_wire-module-repository-backends`, but it should coordinate closely with them so shared repository contracts stay aligned.
- It does not need to wait for `025-05_mirror-specs-and-archives-to-backend` or `025-06_improve-agent-backend-workflows`.
