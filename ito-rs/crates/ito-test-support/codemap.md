[Codemap: ito-test-support]|dev-dep only: shared test infra (output normalization, in-memory repo fakes, PTY helpers, deterministic binary execution); only workspace dep: ito-domain

[Entry Points]|src/lib.rs: run_rust_candidate, normalize_text, collect_file_bytes, dir helpers
|src/mock_repos.rs: in-memory Change/Module/TaskRepository fakes |src/pty/mod.rs: PTY helpers for interactive tests

[Design]|normalizes HOME paths, ANSI escapes, line endings, env vars for deterministic tests; MUST be in [dev-dependencies] only, never [dependencies]

[Gotchas]|candidate binary resolution scans adjacent deps; changes here affect many integration tests |keep mocks in sync when ito-domain repo traits change

[Tests]|cargo test -p ito-test-support (also exercised transitively by all integration tests)
