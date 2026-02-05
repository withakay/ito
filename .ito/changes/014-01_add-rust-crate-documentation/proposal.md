# Change: Add Comprehensive Rust Crate Documentation

## Why

The Rust codebase has 11 library crates that serve as the foundation of Ito. While the code compiles and passes tests, documentation coverage is inconsistent. The project's style guide (`.ito/user-rust-style.md`) establishes clear documentation standards, but not all crates fully adhere to these guidelines. Comprehensive documentation improves maintainability, onboarding, and enables the `#![warn(missing_docs)]` lint to catch gaps.

## What Changes

- Review each Rust crate's `lib.rs` and public modules for documentation completeness
- Add module-level documentation (`//!`) explaining purpose, usage, and examples
- Document all public APIs (`pub fn`, `pub struct`, `pub enum`, `pub trait`) per style guide
- Ensure documentation is genuinely useful (explains *why* and *when*, not just *what*)
- Fix any documentation warnings (e.g., unclosed HTML tags)
- Enable `#![warn(missing_docs)]` on crates that don't have it

## Capabilities

### New Capabilities

- `rust-documentation-standards`: Establishes documentation requirements and conventions for Rust crates

### Modified Capabilities

*None - this is a documentation-only change that doesn't alter runtime behavior*

## Impact

- **Affected code**: All 11 crates under `ito-rs/crates/`:
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
- **Build impact**: Documentation build (`cargo doc`) will pass without warnings
- **No breaking changes**: Documentation-only modifications
