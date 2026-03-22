## Context

The existing `TaskRepository` work covers backend-backed reads, but normal task command flows still mutate local tracking files first and treat backend sync as a follow-up step. That prevents remote mode from behaving as a true repository-backed persistence mode.

## Goals / Non-Goals

- Goals: make task reads repository-backed in both modes; route task mutations through a selected persistence path; keep filesystem mode behavior stable.
- Non-Goals: redesigning the user-facing task model or replacing every task-related command in one shot.

## Decisions

- Keep one selected task persistence path per runtime mode.
- Allow the domain/core boundary to evolve as needed (either by extending `TaskRepository` or pairing it with a task mutation service) so command handlers stop editing markdown directly in remote mode.
- Keep rendered task ordering and status semantics stable across implementations.

## Implementation Preferences

- Keep task ports in `ito-domain`, and keep filesystem/remote implementations in `ito-core`.
- If reads and mutations need different contracts, prefer explicit traits/services over smuggling mutation logic into CLI handlers.
- Keep parsing/formatting concerns separate from persistence concerns, and keep `ito-cli` out of direct `tasks.md` editing in remote mode.
- The current trait home in `ito-rs/crates/ito-domain/src/tasks/repository.rs` is the right anchor point for this work.
- The current filesystem/remote implementation homes in `ito-rs/crates/ito-core/src/task_repository.rs` and `ito-rs/crates/ito-core/src/backend_task_repository.rs` are the natural starting point, with additional `ito-core` services if mutation orchestration needs its own abstraction.
- Markdown parsing and update helpers can stay focused on content transforms, while repository/service layers decide where task state is loaded from or persisted to.

## Testing Preference

- Prefer dedicated test files for mutation flow, repository parity, and conflict/error cases rather than folding the entire matrix into inline production-file tests.

## Contract Sketch

Illustrative only; the key point is separating persistence from command formatting.

```rust
pub trait TaskRepository {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult>;
}

pub trait TaskMutationService {
    fn complete(&self, change_id: &str, task_id: &str) -> DomainResult<TaskMutationResult>;
    fn start(&self, change_id: &str, task_id: &str) -> DomainResult<TaskMutationResult>;
}
```

High-level result/object shape:

```rust
pub struct TaskMutationResult {
    pub change_id: String,
    pub task_id: String,
    pub revision: Option<String>,
}

pub struct RemoteTaskMutationService<C> {
    client: Arc<C>,
}

impl<C> TaskMutationService for RemoteTaskMutationService<C>
where
    C: BackendSyncClient + Send + Sync + 'static,
{
    fn complete(&self, change_id: &str, task_id: &str) -> DomainResult<TaskMutationResult> {
        let bundle = self.client.pull(change_id)?;
        let updated = apply_complete(bundle, task_id)?;
        let result = self.client.push(change_id, &updated)?;

        Ok(TaskMutationResult {
            change_id: change_id.to_string(),
            task_id: task_id.to_string(),
            revision: Some(result.revision),
        })
    }

    fn start(&self, change_id: &str, task_id: &str) -> DomainResult<TaskMutationResult> {
        // same shape as complete; omitted for brevity
        todo!()
    }
}
```

The key preference is that command handlers receive a stable result object and do not need to know whether the mutation was filesystem-backed or remote-backed.

## Risks / Trade-offs

- The current read-only `TaskRepository` interface may need companion mutation abstractions.
- Enhanced task-tracking behavior must stay consistent across filesystem and remote implementations.

## Migration Plan

1. Define the runtime-selected task persistence boundary for reads and mutations.
2. Implement remote-backed task load/update behavior.
3. Migrate CLI task mutation commands to the selected persistence path.
4. Add regression coverage for remote mode with no local `tasks.md` file present.
