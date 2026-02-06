# Tasks

- [ ] Add `arch-guardrails` target (e.g., `make arch-guardrails`) to the repo Makefile.
- [ ] Implement the guardrail runner (script or `xtask`) to:
  - [ ] validate crate dependency direction via `cargo metadata`
  - [ ] enforce domain bans (no *new* `std::fs` / `std::process::Command` usage in `ito-domain`)
- [ ] Add a `prek` local hook in `.pre-commit-config.yaml` that runs `make arch-guardrails`.
- [ ] Add CI step that runs `make arch-guardrails` (or `prek run --all-files`).
- [ ] Document how to run guardrails locally.
