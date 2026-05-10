## Context

The domain already has a `ModuleRepository` interface and a filesystem implementation, and the backend server already exposes module endpoints. The missing piece is client-side module repository wiring that lets commands resolve modules through a selected persistence implementation.

## Goals / Non-Goals

- Goals: add a remote-backed module repository, preserve filesystem mode, and remove direct local module assumptions from command handlers.
- Non-Goals: redesign module semantics or introduce module-specific transport logic into CLI commands.

## Decisions

- Keep one `ModuleRepository` abstraction with filesystem and remote implementations.
- Use the selected repository implementation for module lookup/listing instead of direct filesystem scans.
- Keep module summary output deterministic across persistence implementations.

## Implementation Preferences

- Keep the canonical module trait in `ito-domain`, and keep filesystem/remote adapters in `ito-core`.
- Keep transport and aggregation logic out of the CLI.
- Repositories should return domain models; rendering and formatting stay in the app/CLI layer.
- The current trait home in `ito-rs/crates/ito-domain/src/modules/repository.rs` should remain the contract boundary.
- The current filesystem implementation in `ito-rs/crates/ito-core/src/module_repository.rs` suggests the matching remote-backed adapter should live nearby in `ito-core`, rather than leaking module transport concerns into `ito-cli`.
- If module summaries depend on coordinated change counts, prefer keeping that coordination in `ito-core` instead of turning the repository trait into a presentation-oriented API.

## Testing Preference

- Prefer dedicated test files for module repository parity and CLI integration behavior instead of expanding inline unit tests in production modules.

## Contract Sketch

Illustrative only; intended to keep the module path aligned with the other repositories.

```rust
pub trait ModuleRepository {
    fn exists(&self, id: &str) -> bool;
    fn get(&self, id_or_name: &str) -> DomainResult<Module>;
    fn list(&self) -> DomainResult<Vec<ModuleSummary>>;
}

pub trait BackendModuleReader {
    fn list_modules(&self) -> DomainResult<Vec<ModuleSummary>>;
    fn get_module(&self, id_or_name: &str) -> DomainResult<Module>;
}
```

High-level adapter shape:

```rust
pub struct RemoteModuleRepository<R> {
    reader: Arc<R>,
}

impl<R> ModuleRepository for RemoteModuleRepository<R>
where
    R: BackendModuleReader + Send + Sync + 'static,
{
    fn exists(&self, id: &str) -> bool {
        self.get(id).is_ok()
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        self.reader.get_module(id_or_name)
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        self.reader.list_modules()
    }
}
```

Again, the adapter hides transport/storage details and preserves the same trait contract the filesystem implementation already follows.

## Risks / Trade-offs

- Module/change association counts may require coordinated repository reads.
- Commands that mix modules and changes must avoid accidentally combining implementations.

## Migration Plan

1. Implement the remote-backed module repository path.
2. Route module commands and helpers through the selected `ModuleRepository`.
3. Add regression tests for remote mode with no local `.ito/modules/` data.
