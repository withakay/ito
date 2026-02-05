## ADDED Requirements

### Requirement: ito-common crate exists as foundational utility layer

The `ito-common` crate SHALL exist as a leaf crate with no dependencies on other `ito-*` crates. It SHALL provide foundational utilities that any other crate can depend on.

#### Scenario: Crate has no ito dependencies
- **WHEN** examining `ito-common/Cargo.toml`
- **THEN** there are no dependencies on other `ito-*` crates

### Requirement: ID parsing utilities

The crate SHALL provide ID parsing for `ChangeId`, `ModuleId`, and `SpecId` types with validation and formatting.

#### Scenario: Parse valid change ID
- **WHEN** parsing "001-02_my-change"
- **THEN** returns ChangeId with module_id="001", change_num="02", name="my-change"

#### Scenario: Parse invalid change ID
- **WHEN** parsing "invalid"
- **THEN** returns an error indicating invalid format

### Requirement: Canonical path builders

The crate SHALL provide functions for building canonical paths to ito artifacts (changes, modules, specs, archives).

#### Scenario: Build change directory path
- **WHEN** calling `change_dir(ito_path, "001-02_my-change")`
- **THEN** returns `{ito_path}/changes/001-02_my-change`

#### Scenario: Build spec path
- **WHEN** calling `spec_path(ito_path, "auth")`
- **THEN** returns `{ito_path}/specs/auth/spec.md`

### Requirement: Miette-wrapped I/O utilities

The crate SHALL provide filesystem I/O utilities that wrap errors with miette diagnostics for better error messages.

#### Scenario: Read file with context
- **WHEN** reading a non-existent file using `read_to_string_miette`
- **THEN** error includes file path in diagnostic context

### Requirement: Fuzzy matching utilities

The crate SHALL provide Levenshtein distance calculation and nearest-match finding for user-friendly suggestions.

#### Scenario: Find nearest matches
- **WHEN** searching for "autho" in ["auth", "author", "oauth", "payment"]
- **THEN** returns ["auth", "author", "oauth"] as nearest matches (within threshold)
