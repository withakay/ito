# Tasks

## 1. Foundation Crates

- [x] 1.1 Document `ito-common` - shared types and utilities
- [x] 1.2 Document `ito-models` - data models
- [x] 1.3 Document `ito-schemas` - JSON schemas

## 2. Core Infrastructure

- [x] 2.1 Document `ito-config` - configuration loading and management
- [x] 2.2 Document `ito-logging` - logging infrastructure
- [x] 2.3 Document `ito-domain` - domain models and repositories

## 3. Feature Crates

- [x] 3.1 Document `ito-core` - core Ito functionality
- [x] 3.2 Document `ito-templates` - template management
- [x] 3.3 Document `ito-harness` - AI harness integrations
- [x] 3.4 Document `ito-web` - web server functionality

## 4. Support Crates

- [x] 4.1 Document `ito-test-support` - testing utilities

## 5. Verification

- [x] 5.1 Fix any documentation warnings (e.g., HTML tags in `ito-cli`)
- [x] 5.2 Run `make docs` and verify no warnings
- [x] 5.3 Review documentation coverage across all crates

## Notes

- This work appears to have been completed already on `main` (crate-level docs + `#![warn(missing_docs)]` across the workspace; docs/tests/clippy pass with warnings denied).
