# Refactor (Optional): Split `ito-core` into `ito-application` + `ito-infrastructure`

## Why

- After the earlier refactor waves, `ito-core` may still be a large integration surface that mixes orchestration (use-cases) with concrete I/O implementations.
- A physical split can reinforce the layering rules when multiple adapters (CLI/Web) share the same use-cases.
- This step is intentionally optional and should be executed only if the measured benefits outweigh the migration cost.

## What

- Introduce two crates:
  - `ito-application`: use-cases and ports; depends on `ito-domain`.
  - `ito-infrastructure`: concrete implementations (filesystem/process/templates/etc.); depends on `ito-application` + `ito-domain`.
- Move relevant code out of `ito-core` into the new crates.
- Keep adapters thin:
  - `ito-cli` and `ito-web` depend on `ito-application` and wire in `ito-infrastructure` implementations.

## Guardrails

- Update `make arch-guardrails` dependency checks to enforce:
  - adapters -> application -> domain
  - infrastructure -> application + domain
  - no infrastructure dependency back into adapters

## Depends on

- 015-01_refactor-arch-guardrails
- 015-05_refactor-change-repo-ports
- 015-06_refactor-module-repo-ports
- 015-07_refactor-task-repo-ports
- 015-10_refactor-adapter-thinning

## Verification

- In `ito-rs/`: `cargo test --workspace`
- In `ito-rs/`: `cargo clippy --workspace -- -D warnings`
- `make arch-guardrails`
