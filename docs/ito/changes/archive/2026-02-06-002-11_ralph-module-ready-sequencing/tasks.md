# Tasks for: 002-11_ralph-module-ready-sequencing

## 1. Implementation (Retrospective)

- [x] 1.1 Add `--continue-module` flag to `RalphArgs` and CLI help text
- [x] 1.2 Wire `--continue-module` through `ito-cli` argument conversion and parsing
- [x] 1.3 Update core target resolution for `--module` to select lowest-ID ready change
- [x] 1.4 Implement module continuation loop that processes ready changes until completion
- [x] 1.5 Add preflight and post-run module readiness validation/reorientation for drift handling

## 2. Tests and Verification (Retrospective)

- [x] 2.1 Update/add `ito-core` tests for module selection and continuation behavior
- [x] 2.2 Update CLI help snapshot for new module continuation flag
- [x] 2.3 Run `cargo test -p ito-core --test ralph`
- [x] 2.4 Run `cargo test -p ito-cli --test ralph_smoke`
- [x] 2.5 Run `cargo test -p ito-cli snapshot_ralph_help`
