[Codemap: ito-backend]|L3 adapter: multi-tenant HTTP API for Ito project state; org/repo-scoped REST; delegates to ito-core

[Entry Points]|src/lib.rs: serve fn + config/auth re-exports |src/server.rs: startup + router
|src/api.rs: REST handlers /api/v1/projects/{org}/{repo}/ |src/auth.rs: admin+project-token auth |src/state.rs: shared app state

[Design]|adapter only; business rules in ito-core, contracts in ito-domain; org/repo-scoped allowlist; JSON boundary format

[Gotchas]|route changes affect clients and agent instructions |auth errors must not leak project existence

[Tests]|targeted: cargo test -p ito-backend |e2e via CLI/backend client tests
