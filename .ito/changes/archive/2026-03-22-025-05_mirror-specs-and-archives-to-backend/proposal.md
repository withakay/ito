## Why

Promoted specs and archived changes are important long-term knowledge, but today they are primarily treated as Git-managed files. We also want them queryable through the backend so clients can perform reconciliation, full-history export, and large-scale retrieval without depending solely on local grep over the repo.

## What Changes

- Add a `SpecRepository` abstraction for promoted/current specs with filesystem-backed and remote-backed implementations.
- Mirror promoted specs and archived change artifacts into backend-managed queryable state while still keeping Git projections for backup and ergonomic scanning.
- Extend archive flows so archive/promotion updates the backend mirror as part of the same lifecycle.
- Support repository-backed truth-spec reads so clients can query specs even when local markdown is absent.

## Impact

- Affected specs: `spec-repository`, `cli-show`, `cli-archive`
- Affected code: promoted-spec access paths, archive orchestration, remote read/query surfaces
- Behavioral change: promoted specs and archived changes become queryable from backend-managed state in addition to their Git projections

## Execution Guidance

- Start after `025-04_add-repository-runtime-factory` and `025-01_wire-change-repository-backends` have settled runtime composition and lifecycle-aware change semantics.
- This change is not the first parallel wave because it depends on the final archived-change shape inside `ChangeRepository`.
- `025-06_improve-agent-backend-workflows` should wait until this change has clarified how promoted specs and archived changes are read versus mutated.
