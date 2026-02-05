## ADDED Requirements

### Requirement: Module-Level Documentation

Every Rust library crate (`lib.rs`) SHALL have module-level documentation using `//!` comments that explains:
- The crate's purpose and when to use it
- Key concepts and entry points
- A brief usage example (when applicable)

#### Scenario: Crate lib.rs has module documentation
- **WHEN** reviewing any `lib.rs` file in `ito-rs/crates/*/`
- **THEN** the file MUST begin with `//!` documentation comments
- **AND** the documentation explains the crate's purpose

#### Scenario: Sub-modules have documentation when non-trivial
- **WHEN** a module contains multiple public items or complex logic
- **THEN** the module MUST have `//!` documentation explaining its purpose

### Requirement: Public API Documentation

All public items (`pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub mod`) SHALL have documentation comments that provide genuinely useful context.

Documentation MUST focus on:
- **Purpose**: What does this do and why does it exist?
- **When to use**: In what situations should someone reach for this?
- **Gotchas**: Any non-obvious behavior, edge cases, or invariants?

Documentation MUST NOT:
- Restate the obvious (e.g., "Returns an optional PathBuf" for `-> Option<PathBuf>`)
- List parameters perfunctorily without adding value
- Be empty placeholder comments

#### Scenario: Public function has useful documentation
- **WHEN** a public function is defined
- **THEN** it MUST have a `///` doc comment
- **AND** the comment explains the function's purpose and behavior

#### Scenario: Public struct has useful documentation
- **WHEN** a public struct is defined
- **THEN** it MUST have a `///` doc comment explaining its purpose
- **AND** fields are documented when their meaning isn't obvious from the name

#### Scenario: Public enum has useful documentation
- **WHEN** a public enum is defined
- **THEN** it MUST have a `///` doc comment explaining its purpose
- **AND** variants are documented when their meaning requires clarification

#### Scenario: Error types document causes
- **WHEN** an error enum or struct is defined
- **THEN** each variant/field MUST document what conditions cause that error

### Requirement: Documentation Lint Enforcement

Library crates SHALL enable documentation lints to catch missing docs at compile time.

#### Scenario: Missing docs lint is enabled
- **WHEN** building any library crate in `ito-rs/crates/`
- **THEN** the crate SHOULD have `#![warn(missing_docs)]` at the crate root
- **OR** documentation coverage is verified through `cargo doc` without warnings

#### Scenario: Documentation builds without warnings
- **WHEN** running `make docs` or `cargo doc --no-deps`
- **THEN** the build completes without documentation warnings

### Requirement: Documentation Quality Standards

Documentation SHALL follow the project's established style guide in `.ito/user-rust-style.md`.

#### Scenario: Documentation avoids perfunctory content
- **WHEN** reviewing documentation
- **THEN** it MUST provide value beyond what the type signature already shows
- **AND** explain *why* and *when* to use something, not just *what* it is

#### Scenario: Examples demonstrate common usage
- **WHEN** a public API has non-obvious usage patterns
- **THEN** the documentation SHOULD include a code example
