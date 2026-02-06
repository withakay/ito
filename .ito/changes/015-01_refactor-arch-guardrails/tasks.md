# Tasks

- [x] Add `arch-guardrails` target (e.g., `make arch-guardrails`) to the repo Makefile.
- [x] Implement the guardrail runner (script or `xtask`) to:
  - [x] validate crate dependency direction via `cargo metadata`
  - [x] enforce domain bans (no *new* `std::fs` / `std::process::Command` usage in `ito-domain`)
- [x] Add a `prek` local hook in `.pre-commit-config.yaml` that runs `make arch-guardrails`.
- [x] Add CI step that runs `make arch-guardrails` (or `prek run --all-files`).
- [x] Document how to run guardrails locally.
