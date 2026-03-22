## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change should start after `025-04_add-repository-runtime-factory` and `025-01_wire-change-repository-backends` have stabilized.

## 1. Implementation
- [x] 1.1 Define `SpecRepository` with filesystem and remote-backed implementations
- [x] 1.2 Mirror promoted specs and archived changes into backend-managed queryable state
- [x] 1.3 Update archive/promotion flows to maintain both backend mirror and Git projection
- [x] 1.4 Add tests for spec reads, archived history queries, and backend/Git reconciliation behavior
