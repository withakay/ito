## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change belongs to the first parallel repository wave after `025-04_add-repository-runtime-factory`.

## 1. Implementation
- [ ] 1.1 Implement the remote-backed `ModuleRepository` path
- [ ] 1.2 Route module-aware CLI handlers and helpers through the selected `ModuleRepository`
- [ ] 1.3 Preserve deterministic module ordering and summaries across implementations
- [ ] 1.4 Add tests for filesystem mode, remote mode, and no-local-module-markdown behavior
