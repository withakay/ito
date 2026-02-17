## ADDED Requirements

### Requirement: Unit tests use sibling `*_tests.rs` modules

For Rust code in this repository, unit tests for a module `foo` MUST live in a sibling Rust source file named `foo_tests.rs`.

The sibling test file MUST be in the same directory as the module's defining file:

- If the module is defined by `foo.rs`, the test file MUST be `foo_tests.rs` in that directory.
- If the module is defined by `foo/mod.rs`, the test file MUST be `foo/foo_tests.rs`.

The module's defining file (`foo.rs` or `foo/mod.rs`) MUST include the sibling test module under `#[cfg(test)]`, so that the tests are only compiled in test builds.

#### Scenario: File-based module uses sibling test file

- **WHEN** a module `foo` is defined by `foo.rs`
- **THEN** unit tests for `foo` MUST be implemented in `foo_tests.rs`
- **AND** `foo.rs` MUST declare `#[cfg(test)] mod foo_tests;`

#### Scenario: Directory-based module uses sibling test file

- **WHEN** a module `foo` is defined by `foo/mod.rs`
- **THEN** unit tests for `foo` MUST be implemented in `foo/foo_tests.rs`
- **AND** `foo/mod.rs` MUST declare `#[cfg(test)] mod foo_tests;`

#### Scenario: Inline unit test modules are avoided

- **WHEN** adding or modifying unit tests for a module
- **THEN** tests MUST NOT be added as an inline `#[cfg(test)] mod tests { ... }` block in production code
- **AND** the tests MUST be placed in the module's sibling `*_tests.rs` file instead
