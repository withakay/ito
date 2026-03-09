## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change starts after `025-04_add-repository-runtime-factory` and can run in parallel with the first repository wave.

## 1. Implementation
- [x] 1.1 Extend persistence-mode/config resolution to support `sqlite`
- [x] 1.2 Add SQLite-backed repository adapters behind the existing repository traits
- [x] 1.3 Wire the repository factory to construct SQLite-backed repositories locally without HTTP
- [x] 1.4 Add parity tests proving representative commands behave consistently in filesystem, SQLite, and remote modes
