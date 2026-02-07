# Tasks

- [x] Replace architecture dependency policy checks with `cargo-deny` configuration and commands.
- [x] Replace CLI decoupling checks with Cargo-native `--no-default-features` verification (and optional `cargo-hack` matrix checks).
- [x] Remove or narrow bespoke baseline string-count checks in favor of lint/compiler-backed checks where practical.
- [x] Update Makefile, `prek`, and CI workflows to run the standardized toolchain for architecture guardrails.
- [x] Document the new architecture guardrail workflow and migration trade-offs (strictness vs maintainability).
