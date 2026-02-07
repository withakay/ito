# Tasks

- [ ] Move `ito ralph` handler to `ito-rs/crates/ito-cli/src/commands/ralph.rs`
- [ ] Remove `ito-rs/crates/ito-cli/src/app/ralph.rs` (or leave only shared helpers in `app/`)
- [ ] Update `mod` declarations and imports to match the new module path
- [ ] Run `make check`
- [ ] Run `make test`
- [ ] Run `ito validate 015-15_move-ralph-command-to-commands --strict`
