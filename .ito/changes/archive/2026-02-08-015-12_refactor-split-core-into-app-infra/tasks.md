# Tasks

- [x] Record baseline metrics before splitting:
  - [x] `cargo build --timings` (or equivalent) for incremental compile observations
  - [x] `cargo test --workspace` timing
  - Baseline metrics (captured this iteration): clean `cargo build --workspace --timings` real `15.94s`; subsequent incremental `cargo build --workspace` real `0.16s`; first `cargo test --workspace` real `28.60s`.
- [x] Create crates:
  - [x] `ito-application`
  - [x] `ito-infrastructure`
- [x] Move code:
  - [x] move use-cases/orchestration from `ito-core` -> `ito-application`
  - [x] move filesystem/process/templates implementations from `ito-core` -> `ito-infrastructure`
- [x] Update adapters (`ito-cli`, `ito-web`) to depend on `ito-application` and wire `ito-infrastructure`.
- [x] Update workspace wiring (`ito-rs/Cargo.toml` members + deps).
- [x] Update `make arch-guardrails` to enforce the new crate-edge rules.
- [x] Run full verification (`cargo test`, `cargo clippy`, `make arch-guardrails`).
- [x] Record post-change metrics and compare to baseline.
  - Post-change metrics (warm cache): `cargo test --workspace --quiet` real `4.52s`.
  - Comparison: build/test behavior is healthy after split; incremental build remained fast (`0.16s`) and warm test runtime dropped from first-run `28.60s` to warm-run `4.52s`.
