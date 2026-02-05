# Tasks for: 006-12_itors-init-parity

## 1. Baseline + Parity Targets

- \[x\] 1.1 Document current `itors init` behavior and diff vs `cli-init` spec
- \[x\] 1.2 Identify Rust code entrypoints for `init` and tool installers (`ito-rs/crates/ito-cli`, `ito-rs/crates/ito-core`)

## 2. Rust CLI Flag and Selection Parity

- \[x\] 2.1 Add/confirm `itors init --tools <tools>` and match TS parsing/validation (`all`, `none`, comma list)
- \[x\] 2.2 Add interactive CLI dependencies (`dialoguer`, `crossterm`, `indicatif`) and wire up a prompt-driven tool selection wizard
- \[x\] 2.3 Implement interactive tool selection when `--tools` is omitted in interactive sessions
- [ ] 2.4 Ensure extend mode preserves existing configured tools and adds selected tools only

## 3. Artifact Parity Verification

- \[x\] 3.1 Add fixture repo(s) for init parity (empty repo, existing `.ito/` repo)
- \[x\] 3.2 Add parity harness test for `init --tools all|none|subset`
- \[x\] 3.3 Add PTY-driven parity test for interactive init selection

## 4. Validation

- \[x\] 4.1 Run Rust tests and parity harness suite
- \[x\] 4.2 Update any relevant docs/help text so `itors init` usage matches TypeScript CLI guidance
