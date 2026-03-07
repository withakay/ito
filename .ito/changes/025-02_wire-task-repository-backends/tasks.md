## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change belongs to the first parallel repository wave after `025-04_add-repository-runtime-factory`.

## 1. Implementation
- [ ] 1.1 Define the task persistence boundary for repository-backed reads and mutations
- [ ] 1.2 Implement the remote-backed task persistence path
- [ ] 1.3 Update `ito tasks` mutation handlers to use the selected task persistence implementation
- [ ] 1.4 Add tests for filesystem mode, remote mode, conflict/error surfacing, and no-local-markdown behavior
