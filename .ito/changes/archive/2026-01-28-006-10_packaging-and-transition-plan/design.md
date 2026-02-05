## Context

The TypeScript package is distributed via npm. Rust distribution options include:

- standalone binaries per platform
- installers (brew/scoop/apt)
- an npm wrapper package that downloads a pinned binary

Any strategy must preserve the `ito` CLI surface and allow existing users to upgrade smoothly.

## Goals / Non-Goals

**Goals:**

- Choose a primary distribution mechanism for Rust.
- Define how npm installs map to Rust binaries.
- Define CI artifacts, versioning, and integrity verification.

**Non-Goals:**

- Perform the actual release in this change.

## Decisions

### Decision: Support an npm wrapper for transition

For parity with existing installs, prefer an npm package that installs a platform-specific Rust binary while keeping the `ito` command name.

### Decision: Keep TypeScript CLI as fallback during transition

If Rust binary is unavailable for a platform, fail with a clear message and optionally fall back to TS (if installed) without changing output shape.

## Testing Strategy

- Smoke test install scripts in CI (no network in unit tests)
- Integration tests in CI jobs per platform
