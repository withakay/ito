## Context

Archived backend work already introduced `BackendChangeRepository`, but the CLI still constructs `FsChangeRepository` directly in many places. At the same time, archive/export work wants one lifecycle-aware view over both active and archived changes. The current split leaves backend mode vulnerable to split-brain reads.

## Goals / Non-Goals

- Goals: make `ChangeRepository` the canonical active-change read surface; include archived lifecycle access in the same repository model; support both filesystem and remote implementations.
- Non-Goals: introducing a separate archive repository, changing how promoted specs are stored, or defining new transport types beyond the current remote abstraction.

## Decisions

- Use one `ChangeRepository` with lifecycle-aware queries for `active`, `archived`, and `all` views.
- Keep change IDs stable across lifecycle transitions so commands can resolve the same canonical change through one interface.
- Treat REST as the initial remote transport implementation, but keep transport details behind the repository/client layer.
- Move command wiring toward repository injection/runtime selection rather than command-local repository construction.

## Implementation Preferences

- Keep the canonical port/trait in `ito-domain`, and keep concrete filesystem/remote adapters in `ito-core`.
- Keep `ito-cli` thin: commands should depend on the selected `ChangeRepository` and format output, not inspect `.ito/changes/` or perform transport-specific calls directly.
- Prefer small, focused adapters and services over large repository types that also take on archive or orchestration responsibilities.
- The likely home for the canonical trait remains `ito-rs/crates/ito-domain/src/changes/repository.rs`.
- The likely homes for concrete adapters remain alongside existing code in `ito-rs/crates/ito-core/src/change_repository.rs` and `ito-rs/crates/ito-core/src/backend_change_repository.rs`, or nearby modules if the files need to be split for clarity.
- If lifecycle-aware query/filter types are needed, prefer defining them next to the trait in `ito-domain` so both adapters implement the same contract.

## Testing Preference

- Prefer dedicated test files for repository behavior, composition, and transport-specific cases rather than growing large inline test modules inside production files.

## Contract Sketch

Illustrative only; final names can shift if a cleaner API emerges.

```rust
pub enum ChangeLifecycleFilter {
    Active,
    Archived,
    All,
}

pub trait ChangeRepository {
    fn get(&self, id: &str) -> DomainResult<Change>;
    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary>;
    fn list(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>>;
}
```

High-level adapter shape:

```rust
pub struct RemoteChangeRepository<R> {
    reader: Arc<R>,
}

impl<R> RemoteChangeRepository<R>
where
    R: BackendChangeReader + Send + Sync + 'static,
{
    pub fn new(reader: Arc<R>) -> Self {
        Self { reader }
    }
}

impl<R> ChangeRepository for RemoteChangeRepository<R>
where
    R: BackendChangeReader + Send + Sync + 'static,
{
    fn get(&self, id: &str) -> DomainResult<Change> {
        self.reader.get_change(id)
    }

    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        let change = self.reader.get_change(id)?;
        Ok(change.summary())
    }

    fn list(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        let changes = self.reader.list_changes(filter)?;
        Ok(changes)
    }
}
```

The important consistency point is that adapters normalize transport/storage behavior behind the trait and return the same `DomainResult<_>` shape as the filesystem implementation.

## Risks / Trade-offs

- Lifecycle-aware queries may require extending current repository interfaces.
- Remote mode will surface stale assumptions in commands that currently inspect `.ito/changes/` directly.

## Migration Plan

1. Extend change repository contracts for lifecycle-aware listing/resolution.
2. Implement the remote-backed change reader/repository path.
3. Migrate list/show and other read commands to use injected repositories.
4. Add regression tests proving remote mode ignores stray local active-change markdown.
