[Codemap: ito-core]|L2: application semantics (create/archive changes, validate specs/tasks, enforce main-first readiness, render/install templates, worktrees, orchestration/ralph, experimental backend sync)

[Entry Points]|src/lib.rs: module map + re-exports |src/repository_runtime.rs: composition point for fs/backend repo impls
|src/{create,archive,validate,show,list}: core workflow use-cases |src/archive_specs.rs: requirement-level delta promotion into current specs
|src/installers: ito init/update installation + proof-based retired-surface cleanup
|src/{harness,orchestrate,ralph}: AI-agent workflow integrations
|src/legacy_coordination.rs: unconditional, side-effect-free inspection of legacy coordination evidence

[Design]|policy-heavy, UI-light; domain traits from ito-domain; concrete fs/backend adapters here
|template bytes from ito-templates; this crate decides where/how to write/render them
|tracked .ito on main is canonical; main-first readiness gates implementation against one immutable authority commit
|audit + coordination modules protect consistency across direct-fs and experimental coordination-worktree modes
|feature boundary: default empty; backend + coordination-branch modules opt in independently; capabilities.rs preflights compiled support

[Gotchas]|#![warn(missing_docs)]; document new pub APIs |never bypass repo abstractions for active-work artifacts
|coordination-worktree and backend modes need same behavior; check runtime selection before adding direct paths
|update-style installs prune retired managed surfaces before writes; never follow symlinked roots/targets
|retired cleanup removes only exact generated fingerprints; preserve/report customized or live-linked paths
|backend/coordination cfgs remain parseable when compiled out; unavailable modes must fail typed preflight before repository mutation

[Tests]|targeted: cargo test -p ito-core <module_or_test_name> |CLI integration tests cover core from outside |make check after broad changes
