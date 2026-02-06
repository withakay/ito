# Tasks

- [ ] Replace architecture dependency policy checks with `cargo-deny` configuration and commands.
- [ ] Replace CLI decoupling checks with Cargo-native `--no-default-features` verification (and optional `cargo-hack` matrix checks).
- [ ] Remove or narrow bespoke baseline string-count checks in favor of lint/compiler-backed checks where practical.
- [ ] Update Makefile, `prek`, and CI workflows to run the standardized toolchain for architecture guardrails.
- [ ] Document the new architecture guardrail workflow and migration trade-offs (strictness vs maintainability).
