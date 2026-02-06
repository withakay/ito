# Tasks

- [x] Make `ito-web` an optional dependency in `ito-rs/crates/ito-cli/Cargo.toml`.
- [x] Add a `web` feature (default-on) that enables `ito-web` integration.
- [x] Gate any web-only CLI commands behind `cfg(feature = "web")`.
- [x] Add a check (CI and/or `make arch-guardrails`) that asserts `ito-cli` does not pull `ito-web` when built with `--no-default-features`.
- [x] Update documentation/help text if command availability changes under `--no-default-features`.
