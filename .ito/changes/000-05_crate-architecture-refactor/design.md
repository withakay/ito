## Context

The `ito-core` crate has grown organically to contain configuration loading, path utilities, ID parsing, filesystem I/O, fuzzy matching, discovery, and business logic (workflow, validation, archiving, installers). This makes dependency management difficult - crates that only need utilities must import all of core, and there's no clear layering to prevent accidental coupling.

Current state:
- `ito-core` depends on: ito-fs, ito-templates, ito-harness
- `ito-domain` depends on: ito-schemas
- `ito-logging` depends on: ito-core (for ConfigContext)
- `ito-fs` is only used in one place (installers)

Research into top Rust projects (ripgrep, cargo, rustc, tokio) shows they favor:
- Generics over trait objects for abstraction
- Explicit context structs over DI containers
- Minimal abstraction - only add traits when there's concrete need

## Goals / Non-Goals

**Goals:**

1. Establish clear dependency hierarchy with foundational leaf crates
2. Enable trait-based DI for filesystem operations (testability)
3. Reduce compile times through smaller, more parallel crate compilation
4. Prevent accidental coupling (e.g., domain depending on CLI)
5. Keep domain "pure" (only data access, no config or business logic)

**Non-Goals:**

- Full DI container/framework (use explicit wiring)
- Abstracting network, process spawning, or other I/O (only filesystem)
- Changing public CLI interface
- Refactoring business logic within ito-core

## Decisions

### 1. Create two foundational crates: ito-common and ito-config

**Decision**: Extract utilities into `ito-common` and configuration into `ito-config` as separate crates.

**Rationale**: These serve different purposes - common is "tools everyone needs", config is "settings resolution". Separating them allows domain to use common without pulling in config.

**Alternatives considered**:
- Single `ito-foundation` crate: Rejected because it conflates utilities with configuration
- Keep in core: Rejected because it prevents proper layering

### 2. FileSystem trait with generics, not trait objects

**Decision**: Define `trait FileSystem` and use generics (`<F: FileSystem>`) for internal APIs.

**Rationale**: Top Rust projects (cargo, tokio) use generics for zero-cost abstraction. Trait objects (`dyn FileSystem`) add vtable overhead and complicate lifetimes.

**Alternatives considered**:
- Trait objects everywhere: Rejected for performance and ergonomics
- No abstraction (direct std::fs): Rejected because testing requires mocking
- Mockall/mockito: Rejected because simple trait + impl is sufficient

### 3. ItoContext struct for bundled state

**Decision**: Create `ItoContext` struct that holds resolved configuration, paths, and options.

**Rationale**: Following Cargo's pattern of a `Config` struct that bundles resolved state. Simpler than passing many individual parameters.

**Implementation**:
```rust
pub struct ItoContext {
    pub config_dir: Option<PathBuf>,
    pub project_root: PathBuf,
    pub ito_path: Option<PathBuf>,
    pub config: ResolvedConfig,
}
```

### 4. Discovery moves to ito-domain

**Decision**: Move the `discovery` module from core to domain.

**Rationale**: Discovery is data access (listing changes, modules, specs from filesystem) which aligns with domain's repository pattern. Domain already has ChangeRepository, ModuleRepository, TaskRepository.

### 5. Inline ito-fs into ito-core

**Decision**: Delete `ito-fs` crate and inline `update_file_with_markers` into `ito-core::installers`.

**Rationale**: Only one use site in the entire codebase. Maintaining a separate crate for one function is unnecessary overhead.

### 6. ito-logging becomes a leaf crate

**Decision**: Refactor `Logger::new()` to accept `config_dir: Option<PathBuf>` instead of `ConfigContext`.

**Rationale**: Allows any crate to use logging without depending on config. CLI resolves the config dir and passes it explicitly.

### 7. ito-domain depends on ito-common

**Decision**: Allow domain to depend on common (for paths, id, io utilities).

**Rationale**: "Pure" means domain doesn't touch config or business logic, not that it can't use shared utilities. Discovery needs path builders and I/O wrappers.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Breaking all import paths | Clear migration guide in proposal; can do mechanical find-replace |
| Compile time increase from more crates | Unlikely - more crates = more parallelism; monitor with `cargo build --timings` |
| FileSystem trait too limited | Start minimal, extend as needed; can add methods without breaking changes |
| ItoContext becomes a god object | Keep it focused on resolved paths/config; don't add business methods |

## Migration Plan

**Phase 1: Create ito-common**
1. Create new crate with Cargo.toml
2. Move modules: id/, paths, io, match_
3. Add FileSystem trait + StdFs
4. Update imports in all dependent crates

**Phase 2: Create ito-config**
1. Create new crate depending on ito-common
2. Move modules: config/, ito_dir/, output/
3. Create ItoContext struct
4. Update imports

**Phase 3: Move discovery to domain**
1. Add ito-common dependency to ito-domain
2. Move discovery module
3. Update imports

**Phase 4: Refactor ito-logging**
1. Change Logger::new signature
2. Remove ito-core dependency
3. Update CLI to pass config_dir explicitly

**Phase 5: Inline ito-fs**
1. Copy update_file_with_markers to core/installers
2. Remove ito-fs from workspace
3. Update Cargo.toml files

**Phase 6: Cleanup**
1. Remove re-exports from ito-core
2. Run `make check` to verify everything passes
3. Update documentation

**Rollback**: Each phase is independently revertible via git. No database migrations or external dependencies.

## Open Questions

1. **Should `paths` module use FileSystem?** Currently it just builds PathBufs. Probably not - path building doesn't need I/O abstraction.

2. **Should we add a MockFileSystem to ito-common?** Or leave mocking to individual test files? Leaning toward a simple in-memory implementation in common for reuse.

3. **Should ItoContext own a FileSystem?** Or should fs be passed separately? Leaning toward separate - keeps context simpler and allows different fs instances per operation.
