## Why

The `ito-core` crate has grown to contain configuration, utilities, and business logic mixed together. This makes it difficult to:
1. Avoid importing code you don't need (compile times, dependency hygiene)
2. Reason about what depends on what (no clear layering)
3. Test components in isolation (side effects scattered throughout)

Extracting foundational crates (`ito-common`, `ito-config`) establishes a clear dependency hierarchy and enables trait-based dependency injection for testability.

## What Changes

- **BREAKING**: Create `ito-common` crate with `FileSystem` trait, ID parsing, path utilities, I/O wrappers, and fuzzy matching
- **BREAKING**: Create `ito-config` crate with configuration loading, `ItoContext` struct, ito directory resolution, and UI options
- **BREAKING**: Move `discovery` module from `ito-core` to `ito-domain`
- **BREAKING**: Refactor `ito-logging` to take `config_dir: Option<PathBuf>` instead of `ConfigContext` (making it a leaf crate)
- **BREAKING**: Inline `ito-fs` into `ito-core` (delete the crate, only one use site)
- Introduce `FileSystem` trait for dependency injection (enables mocking filesystem in tests)
- Update all `Cargo.toml` files to reflect new dependency structure

## Capabilities

### New Capabilities

- `ito-common-crate`: Foundational crate containing `FileSystem` trait + `StdFs` implementation, ID parsing (`ChangeId`, `ModuleId`, `SpecId`), canonical path builders, miette-wrapped I/O utilities, and Levenshtein-based fuzzy matching
- `ito-config-crate`: Configuration crate containing `ItoContext` struct (resolved configuration context), cascading config loading (global, project, ito-dir), ito directory resolution, and UI options (no_color, interactive mode)
- `filesystem-trait`: Trait-based filesystem abstraction enabling dependency injection for testability without a DI container framework

### Modified Capabilities

- `ito-domain`: Absorbs `discovery` module from core, gains dependency on `ito-common`
- `ito-logging`: Becomes a leaf crate by accepting explicit paths instead of `ConfigContext`
- `ito-core`: Reduced to business logic only (workflow, archive, validate, installers, ralph, create, list, show); inlines `ito-fs` marker-update logic

## Impact

**Crate structure:**
```
Leaf crates (no ito-* deps):
  ito-common, ito-logging, ito-schemas, ito-templates, ito-harness, ito-models

Mid-tier:
  ito-config -> ito-common
  ito-domain -> ito-common, ito-schemas

Upper:
  ito-core -> ito-config, ito-domain, ito-templates, ito-harness

Top:
  ito-cli, ito-web
```

**Breaking changes:**
- All crates importing from `ito_core::{config, io, paths, id, match_, discovery, output, ito_dir}` must update imports
- `ito-fs` crate removed entirely
- `Logger::new()` signature changes

**Migration:**
- `ito_core::config::*` -> `ito_config::*`
- `ito_core::io::*` -> `ito_common::io::*`
- `ito_core::paths::*` -> `ito_common::paths::*`
- `ito_core::id::*` -> `ito_common::id::*`
- `ito_core::match_::*` -> `ito_common::match_::*`
- `ito_core::discovery::*` -> `ito_domain::discovery::*`
- `ito_core::output::*` -> `ito_config::output::*`
- `ito_core::ito_dir::*` -> `ito_config::ito_dir::*`
- `ito_fs::*` -> `ito_core::installers::markers::*` (or similar)
