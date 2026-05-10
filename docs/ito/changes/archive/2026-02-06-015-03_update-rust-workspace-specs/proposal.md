# Docs: Align `rust-workspace` spec with the actual workspace

## Why

The current `rust-workspace` specification asserts crate directories (`ito-fs`, `ito-workflow`) that do not exist in the repo. This reduces confidence in the specs and creates noise during validation.

## What

- Update the `rust-workspace` spec requirements that list crate directories so they match the actual workspace crates.

## Out of scope

- Creating new crates to satisfy the old spec.
- Renaming crates.

## Verification

- `ito validate 015-03_update-rust-workspace-specs --strict`
