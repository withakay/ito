# Change: Remove JS/TS implementation and toolchain (Rust-only)

## Why

The repository still includes a legacy TypeScript/Bun implementation and a Node/Bun-based build/test/release toolchain.
This adds maintenance overhead, splits behavior across languages, and blocks a full commitment to the supported Rust implementation.

## What Changes

- **BREAKING**: Remove the legacy `ito-bun/` TypeScript implementation.
- **BREAKING**: Remove Node/Bun-based packaging, build, and test tooling (npm/bun/vitest/tsc/biome/changesets).
- Make Rust (`ito-rs/`) the only supported implementation for build, lint, test, and distribution workflows.
- Update CI, docs, and specs to reflect Rust-only workflows.
- Remove TypeScript test suites and parity/oracle dependencies; rely on Rust tests for validation.

## Impact

- Affected specs: bun-dev-workflow, bun-package-management, bun-ci-integration, biome-formatting, biome-linting, rust-packaging-transition, rust-installers, rust-parity-harness, cli-completion
- Affected code:
  - Deleted: `ito-bun/`, root Node/Bun scripts/configs
  - Updated: `Makefile`, `.github/workflows/*`, `README.md`, `AGENTS.md`
