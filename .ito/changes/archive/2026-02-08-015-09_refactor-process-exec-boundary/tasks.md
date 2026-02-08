# Tasks

- [x] Introduce a core process execution boundary (trait + default implementation).
- [x] Migrate existing process execution call sites to use the boundary.
- [x] Ensure `ito-domain` contains no process spawning.
- [x] Add tests for:
  - [x] capturing stdout/stderr
  - [x] non-zero exit codes
  - [x] missing executable / spawn failure
- [x] Run `cargo test --workspace`.
- [x] Run `make arch-guardrails`.
