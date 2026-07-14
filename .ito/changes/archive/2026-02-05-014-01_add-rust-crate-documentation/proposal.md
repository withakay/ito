# Change: Add Comprehensive Rust Crate Documentation

## Why

The Rust codebase has 11 core crates that form Ito's foundation. The code compiles and tests pass, but documentation coverage is inconsistent across crates and public APIs.

This work standardizes crate/module/API docs to improve maintainability and onboarding, and makes documentation gaps visible via `cargo doc` and (where appropriate) `#![warn(missing_docs)]`.

## What Changes

- Add or improve crate-level docs (`//!` in each `lib.rs`) describing purpose, key concepts, and entry points
- Add module-level docs (`//!`) for non-trivial public modules
- Add or improve docs for public items (`pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub mod`) following `.ito/user-rust-style.md`
- Fix documentation warnings and broken doctests/markup (e.g., bad HTML tags)
- Where it helps catch regressions, enable `#![warn(missing_docs)]` at crate root; otherwise rely on `cargo doc` staying warning-free

Non-goals:
- No runtime behavior changes
- No public API redesigns or refactors beyond what is required to attach useful docs
- No new external dependencies

## Capabilities

### New Capabilities

- `rust-documentation-standards`: Documentation requirements and conventions for Rust crates

### Modified Capabilities

*None - this is a documentation-only change that doesn't alter runtime behavior*

## Impact

- **Affected specs**: `rust-documentation-standards` (new)
- **Affected code**: Crates under `ito-rs/crates/` (documentation edits only):
  - `ito-common` - Shared types and utilities
  - `ito-config` - Configuration loading and management
  - `ito-core` - Core Ito functionality
  - `ito-domain` - Domain models and repositories
  - `ito-harness` - AI harness integrations
  - `ito-logging` - Logging infrastructure
  - `ito-models` - Data models
  - `ito-schemas` - JSON schemas
  - `ito-templates` - Template management
  - `ito-test-support` - Testing utilities
  - `ito-web` - Web server functionality
- **Build impact**: `cargo doc` / `make docs` runs without warnings; any doc-related warnings are treated as failures for this change
- **Behavior**: No runtime behavior changes; the only expected difference is improved generated docs

Acceptance criteria:
- `make docs` (or `cargo doc --no-deps`) completes without warnings
- `cargo test --workspace` passes
- `cargo clippy --workspace --all-targets -- -D warnings` passes
- Each crate has clear crate-level documentation and public items are documented to the standards in the new spec
