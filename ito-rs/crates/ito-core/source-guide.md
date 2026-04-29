# Source Guide: ito-core

## Responsibility
`ito-core` implements Ito's application semantics: creating and archiving changes, validating specs/tasks, rendering instructions, installing templates, managing coordination worktrees, synchronizing backend state, running Ralph/orchestration flows, and selecting repository runtimes.

## Entry Points
- `src/lib.rs`: public module map and re-exports.
- `src/repository_runtime.rs`: composition point for filesystem/backend repository implementations.
- `src/create`, `src/archive`, `src/validate`, `src/show`, `src/list`: core workflow use-cases.
- `src/installers`: `ito init` / `ito update` file installation behavior.
- `src/harness`, `src/orchestrate`, `src/ralph`: AI-agent workflow integrations.

## Design
- Policy-heavy, UI-light: command surfaces live outside this crate.
- Domain traits come from `ito-domain`; concrete filesystem/backend adapters live here.
- Template bytes come from `ito-templates`; this crate decides where and how to write/render them.
- Audit and coordination modules protect consistency across direct filesystem and coordination-worktree modes.

## Flow
1. Adapter code calls a core use-case with resolved config/context.
2. Runtime selection chooses filesystem, backend, or remote repository implementations.
3. Use-cases mutate or read artifacts through repository traits.
4. Audit, validation, and synchronization code reconcile side effects.

## Integration
- Upstream: `ito-cli`, `ito-backend`, `ito-web`.
- Downstream: `ito-domain`, `ito-config`, `ito-common`, `ito-templates`, `ito-logging`.

## Gotchas
- Many modules are public and `#![warn(missing_docs)]` is enabled; document new public APIs.
- Do not bypass repository abstractions for active-work artifacts unless a module is explicitly filesystem-only.
- Coordination worktree and backend modes often need the same behavior; check runtime selection before adding direct paths.

## Tests
- Targeted: `cargo test -p ito-core <module_or_test_name>`.
- CLI integration tests often cover core behavior from the outside.
- Run `make check` after broad core changes.
