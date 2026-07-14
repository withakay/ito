## Overview

This change introduces a small `.ito/` path helper in `ito-core` and migrates call sites to reduce repetition.

## Design

Create a module such as `ito-rs/crates/ito-core/src/paths.rs` (or `paths/mod.rs`) containing either:

- a `ItoPaths` struct initialized from `(workspace_root, config_context)` that exposes `ito_dir`, `changes_dir`, `modules_dir`, etc.

or

- a set of free functions that take `&Path` and return `PathBuf` consistently.

Then replace duplicated path joins and string formatting in:

- `ito-rs/crates/ito-core/src/create/*`
- `ito-rs/crates/ito-core/src/list.rs`
- `ito-rs/crates/ito-cli/src/main.rs`

## What NOT to Change

- Do not change `.ito/` directory layout.
- Do not change id parsing rules.

## Testing Strategy

- Unit tests for path helpers.
- Integration tests to ensure commands still find the same files.
