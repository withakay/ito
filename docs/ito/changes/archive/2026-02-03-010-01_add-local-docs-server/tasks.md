# Tasks for: 010-01_add-local-docs-server

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 010-01_add-local-docs-server
ito tasks next 010-01_add-local-docs-server
ito tasks start 010-01_add-local-docs-server 1.1
ito tasks complete 010-01_add-local-docs-server 1.1
ito tasks show 010-01_add-local-docs-server
```

---

## Wave 1

- **Depends On**: None
- **Theme**: Research + Implementation (Core)

### Task 1.1: Confirm Caddy capabilities and constraints
- **Files**: `.ito/changes/010-01_add-local-docs-server/design.md`
- **Dependencies**: None
- **Action**:
  - Verify whether stock Caddy can enforce a token via a path prefix (e.g. `/t/<token>/`).
  - Confirm the token gating plan is enforceable without external plugins.
  - Decide whether Markdown rendering is client-side (SPA) or via a Caddy module.
- **Verify**: N/A
- **Done When**: `design.md` has concrete decisions and updated open questions
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

### Task 1.2: Add `ito serve start` command and config
- **Files**: `ito-rs/crates/ito-cli/src/commands/serve/`, `ito-rs/crates/ito-core/src/docs_server/`, `.ito/changes/010-01_add-local-docs-server/specs/cli-serve/spec.md`
- **Dependencies**: Task 1.1
- **Action**:
  - Implement `ito serve start` using project config keys `serve.bind`, `serve.port`, `serve.token`.
  - Keep implementation split across focused modules (avoid growing a single CLI source file; target <1000 SLOC per file).
  - Enforce dependency check for `caddy`.
  - Implement port probing/incrementing.
  - Create `.ito/.state/docs-server/` state files.
- **Verify**: `make test`
- **Done When**: `ito serve start` starts a server and prints a working URL
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 1.3: Serve UI + manifest + allowlisted paths
- **Files**: `ito-rs/crates/ito-cli/src/commands/serve/`, `ito-rs/crates/ito-core/src/docs_server/`, `ito-rs/crates/ito-templates/` (if templates are used), `.ito/changes/010-01_add-local-docs-server/design.md`
- **Dependencies**: Task 1.2
- **Action**:
  - Generate a static SPA and a `manifest.json` listing eligible Markdown files.
  - Configure Caddy to only serve the allowed roots.
  - Render Markdown to HTML in the browser with basic navigation.
- **Verify**: `make test`
- **Done When**: Browser UI can navigate and render files from `.ito/` and `docs/`
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 1.4: Add `ito serve stop` (and optional `status`)
- **Files**: `ito-rs/crates/ito-cli/src/commands/serve/`, `ito-rs/crates/ito-core/src/docs_server/`, `.ito/changes/010-01_add-local-docs-server/specs/cli-serve/spec.md`
- **Dependencies**: Task 1.2
- **Action**:
  - Stop server using recorded pid/state.
  - Handle not-running case gracefully.
  - (Optional) `ito serve status` prints running URL.
- **Verify**: `make test`
- **Done When**: Start/stop cycle works reliably
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Wave 2

- **Depends On**: Wave 1
- **Theme**: Hardening

### Task 2.1: Add tests for port selection and dependency checks
- **Files**: `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**:
  - Add tests for port probing behavior.
  - Add tests for missing caddy error output.
- **Verify**: `make test`
- **Done When**: Tests fail without changes and pass with them
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 2.2: Document configuration and security notes
- **Files**: `README.md`, `docs/` (if appropriate)
- **Dependencies**: Task 2.1
- **Action**:
  - Document `serve.*` config keys and default behavior.
  - Document token gating behavior and safe defaults.
- **Verify**: `make test`
- **Done When**: Docs explain how to run and configure the server
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Wave 3

- **Depends On**: Wave 2
- **Theme**: Review

### Task 3.1: Review security posture
- **Type**: review
- **Files**: `.ito/changes/010-01_add-local-docs-server/design.md`, `.ito/changes/010-01_add-local-docs-server/specs/cli-serve/spec.md`
- **Dependencies**: None
- **Action**:
  - Review allowed paths, binding defaults, and token enforcement.
- **Done When**: Security posture is verified against the spec
- **Updated At**: 2026-02-02
- **Status**: [x] complete
