//! Shared utilities used across Ito crates.
//!
//! `ito-common` is intentionally small and boring: it contains foundational
//! building blocks that are reused across the workspace but do not encode
//! domain-specific behavior.
//!
//! Most crates should depend on these helpers instead of duplicating ad-hoc
//! parsing, path construction, and I/O glue.

#![warn(missing_docs)]

/// File-system abstraction used to make I/O testable.
pub mod fs;

/// Parsing and validation helpers for Ito identifiers (change/module/spec IDs).
pub mod id;

/// Convenience wrappers around common file I/O operations.
pub mod io;

/// Simple fuzzy matching utilities.
pub mod match_;

/// Canonical `.ito/` path builders.
pub mod paths;
