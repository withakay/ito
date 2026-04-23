## Why

The current Homebrew story is split between a legacy `ito` formula that carries service metadata and the dist-generated `ito-cli` formula that actually tracks releases. That split breaks the intended happy path: users can install one name, manage services with another, and `brew services start` still cannot bootstrap backend auth on first run.

## What Changes

- Publish the dist-managed Homebrew formula under the user-facing name `ito` so install and service commands match.
- Patch the dist-generated formula during release publishing to add a Homebrew `service do` block instead of relying on a stale hand-maintained formula.
- Add `ito serve-api --service`, which silently initializes backend auth if needed and then starts the backend for unattended service managers.
- Update Homebrew/backend docs to standardize on `brew tap withakay/ito && brew install ito && brew services start ito`.

## Capabilities

### Modified Capabilities

- `homebrew-formula`: align the published formula name with the installed binary and publish a service-capable dist formula.
- `backend-state-api`: define the service startup contract for Homebrew-managed backend runtime.

## Impact

- Affected code: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/serve_api.rs`, `ito-rs/crates/ito-core/src/backend_auth.rs`, `ito-rs/crates/ito-core/tests/backend_auth.rs`, `ito-rs/crates/ito-cli/tests/*`.
- Affected packaging/release files: `ito-rs/crates/ito-cli/Cargo.toml`, `.github/workflows/v-release.yml`.
- Affected docs: `README.md`, `docs/backend-client-mode.md`.
- No API break for existing `ito serve-api` users; `--service` is additive.
