## ADDED Requirements

### Requirement: Adapter crates do not depend on each other

`ito-cli` MUST NOT have a hard dependency on `ito-web`.

If `ito-cli` offers web-related functionality, it MUST be behind an optional Cargo feature (for example, `web`) so that `ito-cli` can build without the web adapter.

#### Scenario: `ito-cli` builds without the web adapter

- **WHEN** running `cargo build -p ito-cli --no-default-features` in `ito-rs/`
- **THEN** the build MUST succeed

#### Scenario: `ito-cli` does not pull `ito-web` without the feature

- **WHEN** running `cargo tree -p ito-cli --no-default-features` in `ito-rs/`
- **THEN** the dependency graph MUST NOT include `ito-web`
