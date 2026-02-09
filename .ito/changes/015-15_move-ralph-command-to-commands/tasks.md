# Tasks

- [x] Move `ito ralph` handler to `ito-rs/crates/ito-cli/src/commands/ralph.rs`
- [x] Remove `ito-rs/crates/ito-cli/src/app/ralph.rs` (or leave only shared helpers in `app/`)
- [x] Update `mod` declarations and imports to match the new module path
- [x] Run `make check`
- [x] Run `make test`
- [x] Run `ito validate 015-15_move-ralph-command-to-commands --strict`
