[Codemap: ito-core]|L2: application semantics (create/archive changes, validate specs/tasks, render instructions, install templates, worktrees, backend sync, orchestration/ralph)

[Entry Points]|src/lib.rs: module map + re-exports |src/repository_runtime.rs: composition point for fs/backend repo impls
|src/{create,archive,validate,show,list}: core workflow use-cases |src/installers: ito init/update file installation
|src/{harness,orchestrate,ralph}: AI-agent workflow integrations

[Design]|policy-heavy, UI-light; domain traits from ito-domain; concrete fs/backend adapters here
|template bytes from ito-templates; this crate decides where/how to write/render them
|audit + coordination modules protect consistency across direct-fs and coordination-worktree modes

[Gotchas]|#![warn(missing_docs)]; document new pub APIs |never bypass repo abstractions for active-work artifacts
|coordination-worktree and backend modes need same behavior; check runtime selection before adding direct paths

[Tests]|targeted: cargo test -p ito-core <module_or_test_name> |CLI integration tests cover core from outside |make check after broad changes
