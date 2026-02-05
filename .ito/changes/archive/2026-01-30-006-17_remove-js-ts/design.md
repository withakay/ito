## Context

The project is mid-transition from a legacy TypeScript/Bun implementation to a supported Rust implementation in `ito-rs/`.
This change completes the transition by removing the JavaScript/TypeScript codebase and the Node/Bun toolchain.

## Goals / Non-Goals

- Goals:
  - No JavaScript/TypeScript source code required to build, test, lint, or release Ito.
  - `make build`, `make test`, and `make lint` run Rust-only commands.
  - CI runs Rust-only checks.
  - Docs/specs no longer prescribe Bun/Node/TypeScript tooling.
- Non-Goals:
  - Reproduce the npm packaging experience (publishing to npm) in this change.
  - Preserve the TypeScript parity harness/oracle approach.

## Decisions

- Decision: Remove `ito-bun/` and all Node/Bun build/test/release plumbing.
  - Why: Rust is the supported implementation; maintaining duplicate implementations and toolchains is high-cost.
- Decision: Treat Rust tests as the primary validation surface.
  - Why: Eliminates Node/Bun as a test/runtime dependency and simplifies CI.
- Decision: Keep shell completions as an explicit CLI feature (`ito completion install`) rather than an npm postinstall side effect.
  - Why: Rust-only distribution cannot rely on npm lifecycle hooks.

## Risks / Trade-offs

- Breaking change for users relying on npm/bun install flows.
- Loss of TypeScript test coverage; mitigated by ensuring Rust has adequate test coverage and CI gates.

## Migration Plan

1. Update docs and CI first to avoid “half-migrated” state.
1. Remove Node/Bun and TypeScript code.
1. Ensure `make build/test/lint` pass in a Rust-only environment.

## Rollback

Revert this change (restore `ito-bun/` and Node/Bun config) if critical functionality is missing in Rust.
