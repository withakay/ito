## Execution Notes

- For sequencing and parallelization, see `/.ito/modules/025_repository-backends/module.md`.
- This change is the foundation and should be implemented first.

## 1. Implementation
- [ ] 1.1 Define the repository runtime/factory interface and selected repository bundle
- [ ] 1.2 Wire filesystem and remote repository implementations into the runtime selector
- [ ] 1.3 Migrate command handlers/helpers away from direct `Fs*Repository` construction
- [ ] 1.4 Add tests proving the same command paths work in both filesystem and remote modes
