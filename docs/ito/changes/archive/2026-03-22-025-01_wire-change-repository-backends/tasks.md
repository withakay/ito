## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change belongs to the first parallel repository wave after `025-04_add-repository-runtime-factory`.

## 1. Implementation
- [x] 1.1 Extend `ChangeRepository` contracts for lifecycle-aware active and archived reads
- [x] 1.2 Implement the remote-backed change repository path for client use
- [x] 1.3 Route change-reading command handlers through the selected `ChangeRepository`
- [x] 1.4 Add tests for filesystem mode, remote mode, archived enumeration, and stray-local-file resistance
