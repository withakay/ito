[Codemap: ito-web]|L3 adapter: browser UI for browsing+editing Ito projects (HTTP routing, auth, frontend assets, WebSocket terminal); delegates to ito-core; built via `web` feature on ito-cli (default: enabled)

[Entry Points]|src/lib.rs: ServeConfig + serve export |src/main.rs: standalone dev binary
|src/server.rs: server/router |src/api.rs: web API routes |src/frontend.rs: asset serving |src/terminal.rs: WebSocket terminal |src/auth.rs: auth helpers

[Design]|adapter only; no duplicate Ito workflow semantics; browser interactions call ito-core use-cases or repo APIs — never mutate .ito ad hoc

[Gotchas]|keep browser auth separate from backend API auth unless intentionally unified |terminal processes need cleanup to avoid orphaned sessions

[Tests]|targeted: cargo test -p ito-web |UI changes: add browser/API verification
