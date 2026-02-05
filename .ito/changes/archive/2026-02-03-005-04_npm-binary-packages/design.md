## Context

Ito is primarily a Rust CLI and should remain installable without Node.js. However, many developer environments already depend on npm, and npm is a familiar distribution mechanism for cross-platform CLIs when implemented as platform-specific packages plus a thin meta package.

## Goals / Non-Goals

**Goals:**

- Offer an optional npm install path that results in a working native `ito` binary on supported platforms.
- Avoid making Node.js/npm a runtime requirement for Ito itself.
- Keep npm packaging aligned with the GitHub Release artifacts and versions.

**Non-Goals:**

- Replace GitHub Releases / curl installer as the primary distribution method.
- Support every target immediately (start with the same subset as release artifacts).

## Decisions

### Decision: Packaging model (meta + per-platform)

Use a meta package (name TBD) that depends on per-platform packages (e.g. `*-darwin-arm64`, `*-linux-x64`) which each contain the `ito` binary for that platform.

Alternatives considered:

- Single package that downloads from GitHub in `postinstall`: simpler publishing, but adds network fetch logic and increases installer complexity.

### Decision: Version coupling

Npm package versions MUST match the released Ito version so that `npm install` yields a consistent binary.

## Risks / Trade-offs

- Registry credential handling and supply-chain concerns → mitigate by minimizing scripts, pinning versions, and documenting provenance.
- Package size across platforms → mitigate by publishing per-platform packages and keeping the meta package tiny.

## Open Questions

- **Package naming**: use the scoped meta package `@withakay/ito` plus platform packages:
  - `@withakay/ito-darwin-x64`
  - `@withakay/ito-darwin-arm64`
  - `@withakay/ito-linux-x64`
  - `@withakay/ito-linux-arm64`
  - `@withakay/ito-win32-x64`

- **In-scope targets (v1)**: match the GitHub Release artifacts (macOS x64/arm64, Linux x64/arm64, Windows x64).

- **Publish pipeline**: publish from the Release workflow on tag pushes, gated behind `NPM_TOKEN`.
