# Tasks for: 024-16_homebrew-service-bootstrap

## Status Tracking

```bash
ito tasks status 024-16_homebrew-service-bootstrap
ito tasks next 024-16_homebrew-service-bootstrap
ito tasks start 024-16_homebrew-service-bootstrap 1.1
ito tasks complete 024-16_homebrew-service-bootstrap 1.1
```

---

## Wave 1: Service bootstrap behavior
- **Depends On**: None

### Task 1.1: Add failing tests for service-mode auth bootstrap
- **Files**: `ito-rs/crates/ito-core/tests/backend_auth.rs`, `ito-rs/crates/ito-cli/tests/serve_api_service.rs`
- **Dependencies**: None
- **Action**: Add tests that describe `ito serve-api --service` bootstrapping missing auth silently, reusing existing auth when present, and failing on malformed config.
- **Verify**: `cargo test -p ito-core --test backend_auth` and `cargo test -p ito-cli --test serve_api_service`
- **Done When**: New tests fail before implementation and cover generated, existing, and malformed-config cases.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 1.2: Implement `ito serve-api --service`
- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/serve_api.rs`, `ito-rs/crates/ito-core/src/backend_auth.rs`
- **Dependencies**: 1.1
- **Action**: Add a `--service` flag that silently ensures backend auth exists, then continues into normal server startup without printing tokens.
- **Verify**: `cargo test -p ito-core --test backend_auth` and `cargo test -p ito-cli --test serve_api_service`
- **Done When**: Service mode is additive, idempotent, and only fails when auth setup cannot be completed.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

## Wave 2: Homebrew formula alignment
- **Depends On**: Wave 1

### Task 2.1: Rename the dist Homebrew formula to `ito`
- **Files**: `ito-rs/crates/ito-cli/Cargo.toml`
- **Dependencies**: None
- **Action**: Configure package-local dist metadata so the generated Homebrew formula is published as `ito` while the crate remains `ito-cli` and the binary remains `ito`.
- **Verify**: `cargo metadata --no-deps >/dev/null`
- **Done When**: Dist config clearly declares the Homebrew formula name as `ito`.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 2.2: Patch the generated formula with Homebrew service metadata
- **Files**: `.github/workflows/v-release.yml`
- **Dependencies**: 2.1
- **Action**: Add a strict publish-time patch step that injects a `service do` block into the generated formula, targeting `ito serve-api --service`.
- **Verify**: `rg -n "serve-api --service|service do|Formula/" .github/workflows/v-release.yml`
- **Done When**: Release publishing fails loudly if the generated formula shape changes and otherwise commits a service-capable `Formula/ito.rb`.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

## Wave 3: Docs and validation
- **Depends On**: Wave 2

### Task 3.1: Update Homebrew/backend docs to the single-name workflow
- **Files**: `README.md`, `docs/backend-client-mode.md`
- **Dependencies**: None
- **Action**: Document the `brew install ito` and `brew services start ito` flow, including that Homebrew service mode bootstraps backend auth on first run.
- **Verify**: `rg -n "brew (install|services start) ito|serve-api --service" README.md docs/backend-client-mode.md`
- **Done When**: Docs no longer mention `brew services start ito-cli` and explain the service bootstrap behavior.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Checkpoint: Wave 3
- **Depends On**: Wave 2
- **Verify**: `ito validate 024-16_homebrew-service-bootstrap --strict && make check`
- **Done When**: Proposal validates, tests pass, and packaging/docs changes are consistent.
- **Updated At**: 2026-03-07
- **Status**: [x] complete
