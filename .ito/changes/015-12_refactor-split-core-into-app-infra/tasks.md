# Tasks

- [ ] Record baseline metrics before splitting:
  - [ ] `cargo build --timings` (or equivalent) for incremental compile observations
  - [ ] `cargo test --workspace` timing
- [ ] Create crates:
  - [ ] `ito-application`
  - [ ] `ito-infrastructure`
- [ ] Move code:
  - [ ] move use-cases/orchestration from `ito-core` -> `ito-application`
  - [ ] move filesystem/process/templates implementations from `ito-core` -> `ito-infrastructure`
- [ ] Update adapters (`ito-cli`, `ito-web`) to depend on `ito-application` and wire `ito-infrastructure`.
- [ ] Update workspace wiring (`ito-rs/Cargo.toml` members + deps).
- [ ] Update `make arch-guardrails` to enforce the new crate-edge rules.
- [ ] Run full verification (`cargo test`, `cargo clippy`, `make arch-guardrails`).
- [ ] Record post-change metrics and compare to baseline.
